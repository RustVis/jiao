// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by LGPL-3.0 License that can be found
// in the LICENSE file.

#![allow(clippy::cast_possible_truncation)]

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
pub fn floats_are_unit(_array: &[f32]) -> bool {
    unimplemented!()
}

/// Returns true iff the provided number is within a small epsilon of 0.
#[must_use]
pub fn f64_nearly_zero(_a: f64) -> bool {
    unimplemented!()
}

/// Compare two doubles and return true if they are within `max_ulps_diff` of each other.
///
/// - nan as a or b - returns false.
/// - infinity, infinity or -infinity, -infinity - returns true.
/// - infinity and any other number - returns false.
///
/// ulp is an initialism for Units in the Last Place.
pub fn doubles_nearly_equal_ulps(_a: f64, _b: f64, _max_ulps_diff: u8) -> bool {
    unimplemented!()
}

pub fn doubles_nearly_equal(a: f64, b: f64) -> bool {
    doubles_nearly_equal_ulps(a, b, 16)
}
