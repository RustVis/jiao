// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that can be found
// in the LICENSE file.

#![allow(clippy::cast_possible_truncation)]
#![allow(clippy::cast_precision_loss)]
#![allow(clippy::module_name_repetitions)]

pub type Scalar = f32;

pub const NEARLY_ZERO: Scalar = 1.0 / (1 << 12) as Scalar;
pub const SIN_COS_NEARLY_ZERO: Scalar = 1.0 / (1 << 16) as Scalar;
pub const ROOT_2_OVER_2: f32 = std::f32::consts::FRAC_1_SQRT_2;

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

    fn is_int(&self) -> bool {
        self.fuzzy_equal(self.floor())
    }

    fn sign_as_int(self) -> i32 {
        if self == 0.0 {
            0
        } else if self < 0.0 {
            -1
        } else {
            1
        }
    }

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
        self.abs() <= NEARLY_ZERO
    }

    fn nearly_zero_tolerance(self, tolerance: Scalar) -> bool {
        debug_assert!(tolerance >= 0.0);
        self.abs() <= tolerance
    }
    fn fuzzy_equal(self, other: Self) -> bool {
        (self - other).abs() <= Self::EPSILON
    }

    fn nearly_equal(self, other: Self) -> bool {
        (self - other).abs() <= NEARLY_ZERO
    }

    fn nearly_equal_tolerance(self, other: Self, tolerance: Scalar) -> bool {
        debug_assert!(tolerance >= 0.0);
        (self - other).abs() <= tolerance
    }

    fn sin_snap_to_zero(self) -> Self {
        let v = self.sin();
        if v.nearly_zero_tolerance(SIN_COS_NEARLY_ZERO) {
            0.0
        } else {
            v
        }
    }

    fn cos_snap_to_zero(self) -> Self {
        let v = self.cos();
        if v.nearly_zero_tolerance(SIN_COS_NEARLY_ZERO) {
            0.0
        } else {
            v
        }
    }

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
    for i in 0..n {
        if !a[i].fuzzy_equal(b[i]) {
            return false;
        }
    }
    true
}

#[must_use]
pub fn are_finite(array: &[Scalar]) -> bool {
    array.iter().all(|scalar| scalar.is_finite())
}
