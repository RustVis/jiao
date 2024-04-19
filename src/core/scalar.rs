// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that can be found
// in the LICENSE file.

#![allow(clippy::cast_possible_truncation)]
#![allow(clippy::cast_precision_loss)]

use std::f32::consts;

use crate::base::floating_point::f32_saturate2int;

pub type Scalar = f32;

pub const SCALAR_1: Scalar = 1.0;
pub const SCALAR_HALF: Scalar = 0.5;
pub const SCALAR_SQRT_2: Scalar = consts::SQRT_2;
pub const SCALAR_PI: Scalar = consts::PI;
#[allow(clippy::excessive_precision)]
pub const SCALAR_TAN_PI_OVER_8: Scalar = 0.414_213_562;
pub const SCALAR_MAX: Scalar = f32::MAX;
pub const SCALAR_MIN: Scalar = f32::MIN;
pub const SCALAR_INFINITY: Scalar = f32::INFINITY;
pub const SCALAR_NEGATIVE_INFINITY: Scalar = f32::NEG_INFINITY;
pub const SCALAR_NAN: Scalar = f32::NAN;

pub const SCALAR_NEARLY_ZERO: Scalar = 1.0 / (1 << 12) as Scalar;
pub const SCALAR_SIN_COS_NEARLY_ZERO: Scalar = 1.0 / (1 << 16) as Scalar;
pub const SCALAR_ROOT_2_OVER_2: f32 = consts::FRAC_1_SQRT_2;

pub trait ScalarExt {
    #[must_use]
    fn invert(self) -> Self;

    #[must_use]
    fn average(self, b: Self) -> Self;

    #[must_use]
    fn half(self) -> Self;

    #[must_use]
    fn square(self) -> Self;

    #[must_use]
    fn ceil_to_int(self) -> i32;

    #[must_use]
    fn floor_to_int(self) -> i32;

    #[must_use]
    fn round_to_int(self) -> i32;

    #[must_use]
    fn trunc_to_int(self) -> i32;

    #[must_use]
    fn is_int(&self) -> bool;

    /// Returns -1 || 0 || 1 depending on the sign of value:
    /// - -1 if x < 0
    /// -  0 if x == 0
    /// -  1 if x > 0
    #[must_use]
    fn sign_as_int(self) -> i32;

    /// Scalar result version of above
    #[must_use]
    fn sign_as_scalar(self) -> Self;

    #[must_use]
    fn nearly_zero(self) -> bool;

    #[must_use]
    fn nearly_zero_tolerance(self, tolerance: Scalar) -> bool;

    #[must_use]
    fn fuzzy_equal(self, other: Self) -> bool;
    #[must_use]
    fn nearly_equal(self, other: Self) -> bool;

    #[must_use]
    fn nearly_equal_tolerance(self, other: Self, tolerance: Scalar) -> bool;

    #[must_use]
    fn sin_snap_to_zero(self) -> Self;

    #[must_use]
    fn cos_snap_to_zero(self) -> Self;

    /// Linearly interpolate between self and other, based on t.
    ///
    /// - If t is 0, return self
    /// - If t is 1, return other
    /// - else interpolate.
    ///
    /// t must be [0..1.0]
    #[must_use]
    fn interp(self, other: Self, t: Self) -> Self;

    // TODO(Shaohua): Remove this method when f32::midpoint() is availabel in stable API.
    #[must_use]
    fn mid_point(self, other: Self) -> Self;

    #[must_use]
    fn tpin(self, low: Self, hi: Self) -> Self;
}

impl ScalarExt for Scalar {
    fn invert(self) -> Self {
        debug_assert!(self != 0.0);
        1.0 / self
    }

    fn average(self, b: Self) -> Self {
        (self + b) / 2.0
    }

    fn half(self) -> Self {
        self / 2.0
    }

    fn square(self) -> Self {
        self * self
    }

    fn ceil_to_int(self) -> i32 {
        self.ceil() as i32
    }

    fn floor_to_int(self) -> i32 {
        self.floor() as i32
    }

    fn round_to_int(self) -> i32 {
        self.round() as i32
    }

    fn trunc_to_int(self) -> i32 {
        f32_saturate2int(self)
    }

    fn is_int(&self) -> bool {
        self.fuzzy_equal(self.floor())
    }

    /// Returns -1 || 0 || 1 depending on the sign of value:
    ///   -1 if x < 0
    ///   0 if x == 0
    ///   1 if x > 0
    fn sign_as_int(self) -> i32 {
        if self == 0.0 {
            0
        } else if self < 0.0 {
            -1
        } else {
            1
        }
    }

    /// Returns -1.0 || 0 || 1.0 depending on the sign of value:
    ///   -1.0 if x < 0
    ///   0.0 if x == 0
    ///   1.0 if x > 0
    fn sign_as_scalar(self) -> Self {
        if self == 0.0 {
            0.0
        } else if self < 0.0 {
            -1.0
        } else {
            1.0
        }
    }

    fn nearly_zero(self) -> bool {
        self.abs() <= SCALAR_NEARLY_ZERO
    }

    fn nearly_zero_tolerance(self, tolerance: Scalar) -> bool {
        debug_assert!(tolerance >= 0.0);
        self.abs() <= tolerance
    }

    fn fuzzy_equal(self, other: Self) -> bool {
        (self - other).abs() <= Self::EPSILON
    }

    fn nearly_equal(self, other: Self) -> bool {
        (self - other).abs() <= SCALAR_NEARLY_ZERO
    }

    fn nearly_equal_tolerance(self, other: Self, tolerance: Scalar) -> bool {
        debug_assert!(tolerance >= 0.0);
        (self - other).abs() <= tolerance
    }

    fn sin_snap_to_zero(self) -> Self {
        let v = self.sin();
        if v.nearly_zero_tolerance(SCALAR_SIN_COS_NEARLY_ZERO) {
            0.0
        } else {
            v
        }
    }

    fn cos_snap_to_zero(self) -> Self {
        let v = self.cos();
        if v.nearly_zero_tolerance(SCALAR_SIN_COS_NEARLY_ZERO) {
            0.0
        } else {
            v
        }
    }

    /// Linearly interpolate between self and other, based on t.
    ///
    /// If t is 0, return self
    /// If t is 1, return other
    /// else interpolate.
    /// t must be `[0..SCALAR_1]`
    fn interp(self, other: Self, t: Self) -> Self {
        debug_assert!((0.0..=1.0).contains(&t));
        (other - self).mul_add(t, self)
    }

    fn mid_point(self, other: Self) -> Self {
        const LO: f32 = f32::MIN_POSITIVE * 2.0;
        const HI: f32 = f32::MAX / 2.0;

        let (a, b) = (self, other);
        let abs_a = a.abs();
        let abs_b = b.abs();

        if abs_a <= HI && abs_b <= HI {
            // Overflow is impossible
            (a + b) / 2.0
        } else if abs_a < LO {
            // Not safe to halve a
            a + (b / 2.0)
        } else if abs_b < LO {
            // Not safe to halve b
            (a / 2.0) + b
        } else {
            // Not safe to halve a and b
            (a / 2.0) + (b / 2.0)
        }
    }

    #[must_use]
    fn tpin(self, low: Self, hi: Self) -> Self {
        low.max(self.min(hi))
    }
}

/// Helper to compare an array of scalars.
#[must_use]
pub fn scalars_equal(a: &[Scalar], b: &[Scalar], n: usize) -> bool {
    debug_assert!(a.len() >= n);
    debug_assert!(b.len() >= n);

    for i in 0..n {
        if !a[i].fuzzy_equal(b[i]) {
            return false;
        }
    }
    true
}

#[must_use]
pub fn scalars_are_finite(array: &[Scalar]) -> bool {
    array.iter().all(|scalar| scalar.is_finite())
}

/// Interpolate along the function described by (keys[length], values[length])
/// for the passed searchKey.
///
/// `SearchKeys` outside the range keys[0]-keys[Length] clamp to the min or max value.
/// This function assumes the number of pairs (length) will be small and a linear search is used.
///
/// Repeated keys are allowed for discontinuous functions (so long as keys is
/// monotonically increasing). If key is the value of a repeated scalar in
/// keys the first one will be used.
#[must_use]
pub fn scalar_interp_func(
    search_key: Scalar,
    keys: &[Scalar],
    values: &[Scalar],
    length: usize,
) -> Scalar {
    debug_assert!(length > 0);
    debug_assert!(keys.len() >= length);
    debug_assert!(values.len() >= length);
    #[cfg(debug_assertions)]
    for i in 1..length {
        debug_assert!(keys[i - 1] <= keys[i]);
    }

    let mut right = 0;
    while right < length && keys[right] < search_key {
        right += 1;
    }

    // Could use sentinel values to eliminate conditionals, but since the
    // tables are taken as input, a simpler format is better.
    if right == length {
        return values[length - 1];
    }
    if right == 0 {
        return values[0];
    }

    // Otherwise, interpolate between right - 1 and right.
    let left_key = keys[right - 1];
    let right_key = keys[right];
    debug_assert!(right_key > left_key);
    let fract = (search_key - left_key) / (right_key - left_key);
    values[right - 1].interp(values[right], fract)
}
