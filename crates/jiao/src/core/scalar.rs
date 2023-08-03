// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

#![allow(clippy::cast_possible_truncation)]
#![allow(clippy::cast_precision_loss)]
#![allow(clippy::module_name_repetitions)]

pub type Scalar = f32;

pub const NEARLY_ZERO: Scalar = 1.0 / (1 << 12) as Scalar;
pub const SIN_COS_NEARLY_ZERO: Scalar = 1.0 / (1 << 16) as Scalar;

pub trait ScalarExt {
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
}

impl ScalarExt for Scalar {
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
}

#[must_use]
pub const fn scalar_from_int(x: i32) -> Scalar {
    x as Scalar
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
