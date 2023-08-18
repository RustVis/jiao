// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

// Max Signed 16 bit value
pub const MAX_S16: i16 = i16::MAX;
pub const MIN_S16: i16 = i16::MIN;

pub const MAX_S32: i32 = i32::MAX;
pub const MIN_S32: i32 = i32::MIN;
pub const NAN_S32: i32 = i32::MIN;

pub const MAX_S64: i64 = i64::MAX;
pub const MIN_S64: i64 = i64::MIN;

/// Return `a*b/((1 << shift) - 1)`, rounding any fractional bits.
///
/// Only valid if a and b are unsigned and <= 32767 and shift is > 0 and <= 8
pub fn mul_16_shift_round(a: u16, b: u16, shift: i32) -> u32 {
    debug_assert!(a <= 32767);
    debug_assert!(b <= 32767);
    debug_assert!(shift > 0 && shift <= 8);
    let a = u32::from(a);
    let b = u32::from(b);
    let prod = a * b + (1 << (shift - 1));
    (prod + (prod >> shift)) >> shift
}

/// Return `a*b/255`, rounding any fractional bits.
///
/// Only valid if a and b are unsigned and <= 32767.
#[must_use]
pub fn mul_div_255_round(a: u16, b: u16) -> u8 {
    mul_16_shift_round(a, b, 8) as u8
}
