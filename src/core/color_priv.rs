// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that can be found
// in the LICENSE file.

use crate::base::math::mul_div_255_round;
use crate::core::color::PMColor;
use crate::core::scalar::{Scalar, ScalarExt};
use crate::core::types::{A32_SHIFT, B32_SHIFT, G32_SHIFT, R32_SHIFT};

/// Turn 0..255 into 0..256 by adding 1 at the half-way point.
///
/// Used to turn a byte into a scale value, so that we can say `scale * value >> 8`
/// instead of `alpha * value / 255`.
#[must_use]
#[inline]
pub fn alpha_255_to_256(alpha: u8) -> u32 {
    // this one assues that blending on top of an opaque dst keeps it that way
    // even though it is less accurate than a+(a>>7) for non-opaque dsts
    u32::from(alpha) + 1
}

/// Multiplify value by 0..256, and shift the result down 8
/// (i.e. return (value * alpha256) >> 8)
#[must_use]
#[inline]
pub fn alpha_mul(value: u8, alpha256: u8) -> u32 {
    (u32::from(value) * u32::from(alpha256)) >> 8
}

#[must_use]
#[inline]
#[allow(clippy::cast_possible_truncation)]
#[allow(clippy::cast_sign_loss)]
pub fn unit_scalar_clamp_to_byte(x: Scalar) -> u8 {
    x.tpin(0.0, 1.0).mul_add(255.0, 0.5) as u8
}

pub const A32_BITS: usize = 8;
pub const R32_BITS: usize = 8;
pub const G32_BITS: usize = 8;
pub const B32_BITS: usize = 8;

pub const A32_MASK: u32 = (1 << A32_BITS) - 1;
pub const R32_MASK: u32 = (1 << R32_BITS) - 1;
pub const G32_MASK: u32 = (1 << G32_BITS) - 1;
pub const B32_MASK: u32 = (1 << B32_BITS) - 1;

// 32bit backend only supports 1 swizzle order at a time (compile-time).
//
// This is specified by R32_SHIFT=0 or R32_SHIFT=16.
//
// For easier compatibility with its GPU backend, we further restrict these
// to either (in memory-byte-order) RGBA or BGRA.
// Note that this "order" does not directly correspond to the same shift-order,
// since we have to take endianess into account.
//
// Here we enforce this constraint.
pub const RGBA_R32_SHIFT: u32 = 0;
pub const RGBA_G32_SHIFT: u32 = 8;
pub const RGBA_B32_SHIFT: u32 = 16;
pub const RGBA_A32_SHIFT: u32 = 24;

pub const BGRA_B32_SHIFT: u32 = 0;
pub const BGRA_G32_SHIFT: u32 = 8;
pub const BGRA_R32_SHIFT: u32 = 16;
pub const BGRA_A32_SHIFT: u32 = 24;

// TODO(Shaohua):
//SK_PMCOLOR_IS_RGBA
//SK_PMCOLOR_IS_BGRA

#[must_use]
#[inline]
pub const fn get_packed_a32(packed: u32) -> u8 {
    ((packed << (24 - A32_SHIFT)) >> 24) as u8
}

#[must_use]
#[inline]
pub const fn get_packed_r32(packed: u32) -> u8 {
    ((packed << (24 - R32_SHIFT)) >> 24) as u8
}

#[must_use]
#[inline]
pub const fn get_packed_g32(packed: u32) -> u8 {
    ((packed << (24 - G32_SHIFT)) >> 24) as u8
}

#[must_use]
#[inline]
pub const fn get_packed_b32(packed: u32) -> u8 {
    ((packed << (24 - B32_SHIFT)) >> 24) as u8
}

#[inline]
pub const fn a32_assert(a: u32) {
    debug_assert!(a <= A32_MASK);
}

#[inline]
pub const fn r32_assert(r: u32) {
    debug_assert!(r <= R32_MASK);
}

#[inline]
pub const fn g32_assert(g: u32) {
    debug_assert!(g <= G32_MASK);
}

#[inline]
pub const fn b32_assert(b: u32) {
    debug_assert!(b <= B32_MASK);
}

#[must_use]
#[inline]
pub fn premultiply_argb_inline(a: u8, mut r: u8, mut g: u8, mut b: u8) -> PMColor {
    a32_assert(u32::from(a));
    r32_assert(u32::from(r));
    g32_assert(u32::from(g));
    b32_assert(u32::from(b));

    if a != 255 {
        let a16 = u16::from(a);
        r = mul_div_255_round(u16::from(r), a16);
        g = mul_div_255_round(u16::from(g), a16);
        b = mul_div_255_round(u16::from(b), a16);
    }
    PMColor::from_argb(a, r, g, b)
}

// Forcing inlining significantly improves performance.
#[must_use]
#[inline]
pub const fn alpha_mul_q(c: u32, scale: u32) -> u32 {
    let mask: u32 = 0x00FF_00FF;

    let rb: u32 = ((c & mask) * scale) >> 8;
    let ag: u32 = ((c >> 8) & mask) * scale;
    (rb & mask) | (ag & !mask)
}

#[must_use]
pub fn pm_src_over(src: PMColor, dst: PMColor) -> PMColor {
    let src_u32: u32 = src.into();
    let dst_u32: u32 = dst.into();
    let scale = alpha_255_to_256(255 - src.alpha());
    let mask: u32 = 0x00FF_00FF;

    let mut rb: u32 = (((dst_u32 & mask) * scale) >> 8) & mask;
    let mut ag: u32 = (((dst_u32 >> 8) & mask) * scale) & !mask;

    rb += src_u32 & mask;
    ag += src_u32 & !mask;

    // Color channels (but not alpha) can overflow, so we have to saturate to 0xFF in each lane.
    let color: u32 = (rb & 0x0000_01FF).min(0x0000_00FF)
        | (ag & 0x0001_FF00).min(0x0000_FF00)
        | (rb & 0x01FF_0000).min(0x00FF_0000)
        | (ag & 0xFF00_0000);
    color.into()
}
