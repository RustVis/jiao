// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that can be found
// in the LICENSE file.

use super::vx;

/// 16-bit floating point value.
///
/// format is 1 bit sign, 5 bits exponent, 10 bits mantissa.
/// only used for storage.
pub type Half = u16;

/// a NaN value, not all possible NaN values
pub const HalfNaN: Half = 0x7c01;
pub const HalfInfinity: Half = 0x7c00;
/// 2^-14  (minimum positive normal value)
pub const HalfMin: Half = 0x0400;
/// 65504  (maximum positive normal value)
pub const HalfMax: Half = 0x7bff;
/// 2^-10
pub const HalfEpsilon: Half = 0x1400;
/// 1
pub const Half1: Half = 0x3C00;

/// Convert between half and single precision floating point.
///
/// Vectorized functions vx::from_half and vx::to_half are also available.
/// Unlike vx::to_half, this will correctly handle float NaN -> half NaN.
#[must_use]
#[inline]
pub fn HalfToFloat(h: Half) -> f32 {
    from_half(vx::Vec < 1, uint16_t > (h))[0]
}

#[must_use]
#[inline]
pub fn FloatToHalf(f: f32) -> Half {
    if f.is_nan() {
        HalfNaN
    } else {
        to_half(vx::Vec < 1, float > (f))[0]
    }
}
