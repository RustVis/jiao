// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that can be found
// in the LICENSE file.

#![allow(clippy::cast_possible_truncation)]

use std::mem::size_of_val;
use std::ptr;

pub const MAX_I32_FITS_IN_F32: i32 = 2_147_483_520;
#[allow(clippy::cast_precision_loss)]
pub const MAX_I32_FITS_IN_F32_F32: f32 = MAX_I32_FITS_IN_F32 as f32;
pub const MIN_I32_FITS_IN_F32: i32 = -MAX_I32_FITS_IN_F32;
#[allow(clippy::cast_precision_loss)]
pub const MIN_I32_FITS_IN_F32_F32: f32 = MIN_I32_FITS_IN_F32 as f32;

// 0x7fffff8000000000
pub const MAX_I64_FITS_IN_F32: i64 = i64::MAX >> (63 - 24) << (63 - 24);
#[allow(clippy::cast_precision_loss)]
pub const MAX_I64_FITS_IN_F32_F32: f32 = MAX_I64_FITS_IN_F32 as f32;
pub const MIN_I64_FITS_IN_F32: i64 = -MAX_I64_FITS_IN_F32;
#[allow(clippy::cast_precision_loss)]
pub const MIN_I64_FITS_IN_F32_F32: f32 = MIN_I64_FITS_IN_F32 as f32;

/// Return the closest int for the given float.
///
/// Returns `MAX_I32_FITS_IN_F32` for NaN.
#[must_use]
pub fn f32_saturate2int(mut x: f32) -> i32 {
    x = if x < MAX_I32_FITS_IN_F32_F32 {
        x
    } else {
        MAX_I32_FITS_IN_F32_F32
    };
    x = if x > MIN_I32_FITS_IN_F32_F32 {
        x
    } else {
        MIN_I32_FITS_IN_F32_F32
    };
    x as i32
}

/// Return the closest int for the given double.
///
/// Returns `i32::MAX` for NaN.
pub fn f64_saturate2int(mut x: f64) -> i32 {
    x = if x < i32::MAX.into() {
        x
    } else {
        i32::MAX.into()
    };
    x = if x > i32::MIN.into() {
        x
    } else {
        i32::MIN.into()
    };
    x as i32
}

/// Return the closest i64 for the given float.
///
/// Returns `MAX_I64_FITS_IN_F32` for NaN.
pub fn f32_saturate2int64(mut x: f32) -> i64 {
    x = if x < MAX_I64_FITS_IN_F32_F32 {
        x
    } else {
        MAX_I64_FITS_IN_F32_F32
    };
    x = if x > MIN_I64_FITS_IN_F32_F32 {
        x
    } else {
        MIN_I64_FITS_IN_F32_F32
    };
    x as i64
}

#[must_use]
pub fn f32_floor2int(x: f32) -> i32 {
    f32_saturate2int(x.floor())
}

#[must_use]
pub fn f32_round2int(x: f32) -> i32 {
    f32_saturate2int(x.round())
}

#[must_use]
pub fn f32_ceil2int(x: f32) -> i32 {
    f32_saturate2int(x.ceil())
}

/// Calculate the midpoint between a and b.
///
/// Similar to `std::midpoint` in c++20.
pub fn f32_midpoint(a: f32, b: f32) -> f32 {
    let a = f64::from(a);
    let b = f64::from(b);
    // Use f64 math to avoid underflow and overflow.
    (0.5 * (a + b)) as f32
}

/// Returns false if any of the floats are outside the range [0...1].
///
/// Returns true if count is 0.
pub fn floats_are_unit(array: &[f32]) -> bool {
    let mut is_unit = true;
    let range = 0.0..=1.0;
    for num in array {
        is_unit &= range.contains(num);
    }
    is_unit
}

#[must_use]
#[inline]
pub fn f32_rsqrt(x: f32) -> f32 {
    1.0 / x.sqrt()
}

//pub const FLT_DECIMAL_DIG: usize = std::numeric_limits<float>::max_digits10;
/// The number of significant digits to print.
pub const FLOAT_DECIMAL_DIG: usize = 9;
pub const DOUBLE_DECIMAL_DIG: usize = 17;

#[must_use]
#[inline]
pub fn sk_floats_are_finite(array: &[f32]) -> bool {
    let mut prod: f32 = 0.0;

    for &num in array {
        prod *= num;
    }
    // At this point, prod will either be NaN or 0
    // if prod is NaN, this check will return false
    !prod.is_nan()
}

/// Returns true iff the provided number is within a small epsilon of 0.
#[must_use]
#[inline]
pub fn f64_nearly_zero(a: f64) -> bool {
    a.abs() < f64::EPSILON
}
// Return the positive magnitude of a double.
// * normalized - given 1.bbb...bbb x 2^e return 2^e.
// * subnormal - return 0.
// * nan & infinity - return infinity
#[must_use]
fn magnitude(a: f64) -> f64 {
    const EXTRACT_MAGNITUDE: i64 =
        0b0111_1111_1111_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000;
    let mut bits: i64 = 0;
    //memcpy(&bits, &a, sizeof(bits));
    unsafe {
        ptr::copy_nonoverlapping(
            ptr::from_ref(&a).cast::<()>(),
            ptr::from_mut(&mut bits).cast::<()>(),
            size_of_val(&bits),
        );
    }
    bits &= EXTRACT_MAGNITUDE;
    let mut out: f64 = 0.0;
    //memcpy(&out, &bits, sizeof(out));
    unsafe {
        ptr::copy_nonoverlapping(
            ptr::from_ref(&bits).cast::<()>(),
            ptr::from_mut(&mut out).cast::<()>(),
            size_of_val(&out),
        );
    }
    out
}

/// Compare two doubles and return true if they are within `max_ulps_diff` of each other.
///
/// - nan as a or b - returns false.
/// - infinity, infinity or -infinity, -infinity - returns true.
/// - infinity and any other number - returns false.
///
/// ulp is an initialism for Units in the Last Place.
#[must_use]
#[inline]
pub fn doubles_nearly_equal_ulps(a: f64, b: f64, max_ulps_diff: u8) -> bool {
    // The maximum magnitude to construct the ulp tolerance. The proper magnitude for
    // subnormal numbers is minMagnitude, which is 2^-1021, so if a and b are subnormal (having a
    // magnitude of 0) use minMagnitude. If a or b are infinity or nan, then maxMagnitude will be
    // +infinity. This means the tolerance will also be infinity, but the expression b - a below
    // will either be NaN or infinity, so a tolerance of infinity doesn't matter.
    let min_magnitude: f64 = f64::MIN;
    let max_magnitude = magnitude(a).max(min_magnitude).max(magnitude(b));

    // Given a magnitude, this is the factor that generates the ulp for that magnitude.
    // In numbers, 2 ^ (-precision + 1) = 2 ^ -52.
    let ulp_factor: f64 = f64::EPSILON;

    // The tolerance in ULPs given the maxMagnitude. Because the return statement must use <
    // for comparison instead of <= to correctly handle infinities, bump maxUlpsDiff up to get
    // the full maxUlpsDiff range.
    let tolerance: f64 = max_magnitude * (ulp_factor * f64::from(max_ulps_diff + 1));

    // The expression a == b is mainly for handling infinities, but it also catches the exact equals.
    (a.is_infinite() && b.is_infinite()) || (a - b).abs() < tolerance
}

#[must_use]
#[inline]
pub fn doubles_nearly_equal(a: f64, b: f64) -> bool {
    doubles_nearly_equal_ulps(a, b, 16)
}
