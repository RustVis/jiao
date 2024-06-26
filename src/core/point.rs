// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that can be found
// in the LICENSE file.

use std::ops::{Add, AddAssign, Mul, MulAssign, Neg, Sub, SubAssign};

use crate::core::scalar::{Scalar, ScalarExt};

/// `IVector` provides an alternative name for `IPoint`.
///
/// `IVector` and `IPoint` can be used interchangeably for all purposes.
pub type IVector = IPoint;

/// `IPoint` holds two 32-bit integer coordinates.
#[derive(Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct IPoint {
    /// x-axis value
    x: i32,
    /// y-axis value
    y: i32,
}

impl Default for IPoint {
    fn default() -> Self {
        Self::new()
    }
}

impl IPoint {
    #[must_use]
    #[inline]
    pub const fn new() -> Self {
        Self { x: 0, y: 0 }
    }

    /// Sets to (x, y)
    #[must_use]
    #[inline]
    pub const fn from_xy(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    /// Returns x-axis value of `IPoint`.
    #[must_use]
    #[inline]
    pub const fn x(&self) -> i32 {
        self.x
    }

    /// Returns y-axis value of `IPoint`.
    #[must_use]
    #[inline]
    pub const fn y(&self) -> i32 {
        self.y
    }

    /// Returns true if x and y are both zero.
    #[must_use]
    #[inline]
    pub const fn is_zero(&self) -> bool {
        (self.x | self.y) == 0
    }

    /// Sets new x and y.
    pub fn set(&mut self, x: i32, y: i32) {
        self.x = x;
        self.y = y;
    }

    #[must_use]
    #[inline]
    pub const fn equals(&self, x: i32, y: i32) -> bool {
        self.x == x && self.y == y
    }

    /// Changes the sign of x and y.
    pub fn negate(&mut self) {
        self.x = -self.x;
        self.y = -self.y;
    }
}

impl Add<Self> for IPoint {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self {
            x: self.x.saturating_add(other.x),
            y: self.y.saturating_add(other.y),
        }
    }
}

impl AddAssign<Self> for IPoint {
    fn add_assign(&mut self, other: Self) {
        self.x = self.x.saturating_add(other.x);
        self.y = self.y.saturating_add(other.y);
    }
}

impl Sub<Self> for IPoint {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x.saturating_sub(other.x),
            y: self.y.saturating_sub(other.y),
        }
    }
}

impl SubAssign<Self> for IPoint {
    fn sub_assign(&mut self, other: Self) {
        self.x = self.x.saturating_sub(other.x);
        self.y = self.y.saturating_sub(other.y);
    }
}

/// Returns `IPoint` changing the signs of x and y.
impl Neg for IPoint {
    type Output = Self;

    fn neg(self) -> Self {
        Self {
            x: -self.x,
            y: -self.y,
        }
    }
}

/// `Vector` provides an alternative name for `Point`.
///
/// `Vector` and `Point` can be used interchangeably for all purposes.
pub type Vector = Point;

/// `Point` holds two 32-bit floating point coordinates.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Point {
    /// x-axis value
    x: f32,
    /// y-axis value
    y: f32,
}

impl Default for Point {
    fn default() -> Self {
        Self::new()
    }
}

impl Point {
    #[must_use]
    #[inline]
    pub const fn new() -> Self {
        Self { x: 0.0, y: 0.0 }
    }

    #[must_use]
    #[inline]
    pub const fn from_xy(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    /// Returns x-axis value of `Point` or vector.
    #[must_use]
    #[inline]
    pub const fn x(&self) -> f32 {
        self.x
    }

    /// Returns y-axis value of `Point` or vector.
    #[must_use]
    #[inline]
    pub const fn y(&self) -> f32 {
        self.y
    }

    /// Returns true if x and y are both zero.
    #[must_use]
    #[inline]
    pub fn is_zero(&self) -> bool {
        self.x.fuzzy_equal(0.0) && self.y.fuzzy_equal(0.0)
    }

    /// Sets new x and y.
    #[inline]
    pub fn set(&mut self, x: f32, y: f32) {
        self.x = x;
        self.y = y;
    }

    /// Sets new x and y, promoting integers to float values.
    #[inline]
    #[allow(clippy::cast_precision_loss)]
    pub fn iset(&mut self, x: i32, y: i32) {
        self.x = x as f32;
        self.y = y as f32;
    }
    /// Sets new x and y, promoting integers to float values.
    #[inline]
    pub fn iset_point(&mut self, point: IPoint) {
        self.iset(point.x, point.y);
    }

    /// Sets x to absolute value of `pt.x`; and y to absolute value of `pt.y`.
    pub fn set_abs(&mut self, pt: Self) {
        self.x = pt.x.abs();
        self.y = pt.y.abs();
    }

    /// Adds offset (dx, dy) to each `Point` in points array of length count.
    pub fn offset_slice(points: &mut [Self], dx: f32, dy: f32) {
        for point in points {
            point.offset(dx, dy);
        }
    }

    /// Adds offset (dx, dy) to `Point`.
    #[inline]
    pub fn offset(&mut self, dx: f32, dy: f32) {
        self.x += dx;
        self.y += dy;
    }

    /// Returns the Euclidean distance from origin, computed as:
    /// `(x * x + y * y).sqrt()`
    #[must_use]
    #[allow(clippy::cast_possible_truncation)]
    pub fn length(&self) -> f32 {
        let mag2 = self.x.mul_add(self.x, self.y * self.y);
        if mag2.is_finite() {
            mag2.sqrt()
        } else {
            let xx = f64::from(self.x);
            let yy = f64::from(self.y);
            xx.mul_add(xx, yy * yy).sqrt() as f32
        }
    }

    #[must_use]
    #[inline]
    pub fn length_sqd(&self) -> Scalar {
        Self::dot_product(self, self)
    }

    /// Scales (x, y) so that `length()` returns one, while preserving ratio of x to y,
    /// if possible.
    ///
    /// If prior length is nearly zero, sets vector to (0, 0) and returns
    /// false; otherwise returns true.
    #[must_use]
    #[inline]
    pub fn normalize(&mut self) -> bool {
        self.set_length(1.0)
    }

    /// Scales so that `length()` returns one, while preserving ratio of x to y, if possible.
    ///
    /// If original length is nearly zero, sets to (0, 0) and returns
    /// zero; otherwise, returns length of self before self is scaled.
    ///
    /// Returned prior length may be INFINITY if it can not be represented by float.
    ///
    /// Note that `normalize()` is faster if prior length is not required.
    #[must_use]
    pub fn normalize_length(&mut self) -> f32 {
        let mut mag = 0.0;
        if self.set_point_length(self.x, self.y, 1.0, &mut mag, false) {
            mag
        } else {
            0.0
        }
    }

    /// Sets vector to (x, y) scaled so `length()` returns one, and so that
    /// (x, y) is proportional to (x, y).
    ///
    /// If (x, y) length is nearly zero, sets vector to (0, 0) and returns false;
    /// otherwise returns true.
    pub fn set_normalize(&mut self, x: f32, y: f32) -> bool {
        self.set_xy_length(x, y, 1.0)
    }

    /// Scales vector so that `distance_to_origin()` returns length, if possible.
    ///
    /// If former length is nearly zero, sets vector to (0, 0) and return false;
    /// otherwise returns true.
    ///
    /// # Parameters
    ///
    /// - `length` - straight-line distance to origin
    pub fn set_length(&mut self, length: f32) -> bool {
        self.set_xy_length(self.x, self.y, length)
    }

    /// Sets vector to (x, y) scaled to length, if possible.
    ///
    /// If former length is nearly zero, sets vector to (0, 0) and return false;
    /// otherwise returns true.
    ///
    /// # Parameters
    /// - `x` - proportional value for x
    /// - `y` - proportional value for y
    /// - `length` - straight-line distance to origin
    #[must_use]
    pub fn set_xy_length(&mut self, x: f32, y: f32, length: f32) -> bool {
        let mut orig_length = 0.0;
        self.set_point_length(x, y, length, &mut orig_length, false)
    }

    /// Sets `dst` to `Point` times `scale`.
    ///
    /// # Parameters
    /// - `scale` - factor to multiply `Point` by
    /// - `dst` - storage for scaled `Point`
    #[inline]
    pub fn scale_dst(&self, scale: f32, dst: &mut Self) {
        dst.set(self.x * scale, self.y * scale);
    }

    /// Scales `Point` in place by scale.
    ///
    /// # Parameters
    /// - `scale` - factor to multiply `Point` by
    #[inline]
    pub fn scale(&mut self, scale: f32) {
        self.x *= scale;
        self.y *= scale;
    }

    /// Changes the sign of x and y.
    #[inline]
    pub fn negate(&mut self) {
        self.x = -self.x;
        self.y = -self.y;
    }

    /// Returns true if both x and y are measurable values.
    #[must_use]
    pub fn is_finite(&self) -> bool {
        let mut accum = 0.0;
        accum *= self.x;
        accum *= self.y;

        // accum is either NaN or it is finite (zero).
        debug_assert!(0.0 == accum || accum.is_nan());

        // value==value will be true iff value is not NaN
        !accum.is_nan()
    }

    /// Returns true if `Point` is equivalent to `Point` constructed from (x, y).
    #[must_use]
    #[inline]
    pub fn equals(&self, x: f32, y: f32) -> bool {
        self.x.fuzzy_equal(x) && self.y.fuzzy_equal(y)
    }

    #[must_use]
    #[inline]
    pub fn equals_point(&self, other: &Self) -> bool {
        self.x.fuzzy_equal(other.x) && self.y.fuzzy_equal(other.y)
    }

    /// Returns the Euclidean distance between self and other.
    #[must_use]
    pub fn distance(&self, other: Self) -> f32 {
        (*self - other).length()
    }

    /// Returns the Euclidean distance from origin, computed as:
    /// `(x * x + y * y).sqrt()`
    #[must_use]
    pub fn distance_to_origin(&self) -> f32 {
        self.length()
    }

    /// Returns the dot product of vector a and vector b.
    #[must_use]
    #[inline]
    pub fn dot_product(a: &Self, b: &Self) -> Scalar {
        a.x.mul_add(b.x, a.y * b.y)
    }

    /// Returns the cross product of vector a and vector b.
    ///
    /// a and b form three-dimensional vectors with z-axis value equal to zero. The
    /// cross product is a three-dimensional vector with x-axis and y-axis values equal
    /// to zero. The cross product z-axis component is returned.
    #[must_use]
    #[inline]
    pub fn cross_product(a: &Self, b: &Self) -> f32 {
        a.x.mul_add(b.y, -a.y * b.x)
    }

    /// Returns the cross product of self and other vector.
    ///
    /// `Vector` and vec form three-dimensional vectors with z-axis value equal to zero.
    /// The cross product is a three-dimensional vector with x-axis and y-axis values
    /// equal to zero. The cross product z-axis component is returned.
    ///
    /// # Parameters
    /// - `vec` - right side of cross product
    #[must_use]
    pub fn cross(&self, other: &Self) -> f32 {
        Self::cross_product(self, other)
    }

    /// Returns the dot product of self and other vector.
    ///
    /// # Parameters
    /// - `vec` - right side of dot product
    #[must_use]
    pub fn dot(&self, other: &Self) -> f32 {
        Self::dot_product(self, other)
    }

    #[must_use]
    pub fn nearly_equal(&self, other: Self) -> bool {
        self.x.nearly_equal(other.x) && self.y.nearly_equal(other.y)
    }

    // We have to worry about 2 tricky conditions:
    // 1. underflow of mag2 (compared against nearlyzero^2)
    // 2. overflow of mag2 (compared w/ isfinite)
    //
    // If we underflow, we return false. If we overflow, we compute again using
    // doubles, which is much slower (3x in a desktop test) but will not overflow.
    #[allow(clippy::cast_possible_truncation)]
    pub(crate) fn set_point_length(
        &mut self,
        mut x: f32,
        mut y: f32,
        length: f32,
        orig_length: &mut f32,
        _use_sqrt: bool,
    ) -> bool {
        // our mag2 step overflowed to infinity, so use doubles instead.
        // much slower, but needed when x or y are very large, other wise we
        // divide by inf. and return (0,0) vector.
        let xx = f64::from(x);
        let yy = f64::from(y);
        let dmag = xx.mul_add(xx, yy * yy).sqrt();
        let scale = (f64::from(length) / dmag) as f32;
        x *= scale;
        y *= scale;
        // check if we're not finite, or we're zero-length
        if !x.is_finite() || !y.is_finite() || (x == 0.0 && y == 0.0) {
            self.set(0.0, 0.0);
            return false;
        }

        *orig_length = dmag as f32;
        self.set(x, y);
        true
    }
}

impl From<IPoint> for Point {
    #[allow(clippy::cast_precision_loss)]
    fn from(point: IPoint) -> Self {
        Self::from_xy(point.x() as f32, point.y() as f32)
    }
}

impl Add<Self> for Point {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl AddAssign<Self> for Point {
    fn add_assign(&mut self, other: Self) {
        self.x += other.x;
        self.y += other.y;
    }
}

impl Sub<Self> for Point {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl SubAssign<Self> for Point {
    fn sub_assign(&mut self, other: Self) {
        self.x -= other.x;
        self.y -= other.y;
    }
}

impl Neg for Point {
    type Output = Self;

    fn neg(self) -> Self {
        Self {
            x: -self.x,
            y: -self.y,
        }
    }
}

/// Returns Point multiplied by scale.
impl Mul<f32> for Point {
    type Output = Self;
    fn mul(self, scale: f32) -> Self {
        Self {
            x: self.x * scale,
            y: self.y * scale,
        }
    }
}

/// Multiplies Point by scale.
impl MulAssign<f32> for Point {
    fn mul_assign(&mut self, scale: f32) {
        self.x *= scale;
        self.y *= scale;
    }
}
