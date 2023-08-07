// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

#![allow(clippy::module_name_repetitions)]

use std::ops::{Add, AddAssign, Mul, MulAssign, Neg, Sub, SubAssign};

use crate::core::scalar::ScalarExt;

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

impl IPoint {
    #[must_use]
    pub const fn make(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    /// Returns x-axis value of `IPoint`.
    #[must_use]
    pub const fn x(&self) -> i32 {
        self.x
    }

    /// Returns y-axis value of `IPoint`.
    #[must_use]
    pub const fn y(&self) -> i32 {
        self.y
    }

    /// Returns true if x and y are both zero.
    #[must_use]
    pub const fn is_zero(&self) -> bool {
        self.x | self.y == 0
    }

    /// Sets new x and y.
    pub fn set(&mut self, x: i32, y: i32) {
        self.x = x;
        self.y = y;
    }

    #[must_use]
    pub const fn equals(&self, x: i32, y: i32) -> bool {
        self.x == x && self.y == y
    }
}

impl Add<Self> for IPoint {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl AddAssign<Self> for IPoint {
    fn add_assign(&mut self, other: Self) {
        self.x += other.x;
        self.y += other.y;
    }
}

impl Sub<Self> for IPoint {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl SubAssign<Self> for IPoint {
    fn sub_assign(&mut self, other: Self) {
        self.x -= other.x;
        self.y -= other.y;
    }
}

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

impl Point {
    #[must_use]
    pub const fn make(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    /// Returns x-axis value of `Point` or vector.
    #[must_use]
    pub const fn x(&self) -> f32 {
        self.x
    }

    /// Returns y-axis value of `Point` or vector.
    #[must_use]
    pub const fn y(&self) -> f32 {
        self.y
    }

    /// Returns true if x and y are both zero.
    #[must_use]
    pub fn is_zero(&self) -> bool {
        self.x.fuzzy_equal(0.0) && self.y.fuzzy_equal(0.0)
    }

    /// Sets new x and y.
    pub fn set(&mut self, x: f32, y: f32) {
        self.x = x;
        self.y = y;
    }

    /// Sets new x and y, promoting integers to float values.
    #[allow(clippy::cast_precision_loss)]
    pub fn iset(&mut self, x: i32, y: i32) {
        self.x = x as f32;
        self.y = y as f32;
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
    pub fn offset(&mut self, dx: f32, dy: f32) {
        self.x += dx;
        self.y += dy;
    }

    /// Returns the Euclidean distance from origin, computed as:
    /// `(x * x + y * y).sqrt()`
    #[must_use]
    pub fn length(&self) -> f32 {
        unimplemented!()
    }

    /// Returns the Euclidean distance from origin, computed as:
    /// `(x * x + y * y).sqrt()`
    #[must_use]
    pub fn distance_to_origin(&self) -> f32 {
        self.length()
    }

    /// Scales (x, y) so that `length()` returns one, while preserving ratio of x to y,
    /// if possible.
    ///
    /// If prior length is nearly zero, sets vector to (0, 0) and returns
    /// false; otherwise returns true.
    pub fn normalize(&mut self) -> bool {
        unimplemented!()
    }

    /// Sets vector to (x, y) scaled so length() returns one, and so that
    /// (x, y) is proportional to (x, y).
    ///
    /// If (x, y) length is nearly zero, sets vector to (0, 0) and returns false;
    /// otherwise returns true.
    pub fn set_normalize(&mut self, _x: f32, _y: f32) -> bool {
        unimplemented!()
    }

    /// Scales vector so that `distance_to_origin()` returns length, if possible.
    ///
    /// If former length is nearly zero, sets vector to (0, 0) and return false;
    /// otherwise returns true.
    ///
    /// # Parameters
    ///
    /// - `length` - straight-line distance to origin
    pub fn set_length(&mut self, _length: f32) -> bool {
        unimplemented!()
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
    pub fn set_xy_length(_x: f32, _y: f32, _length: f32) -> bool {
        unimplemented!()
    }

    /// Sets `dst` to `Point` times `scale`.
    ///
    /// # Parameters
    /// - `scale` - factor to multiply `Point` by
    /// - `dst` - storage for scaled `Point`
    pub fn scale_dst(&mut self, _scale: f32, _dst: &mut Self) {
        unimplemented!()
    }

    /// Scales `Point` in place by scale.
    ///
    /// # Parameters
    /// - `value` - factor to multiply `Point` by
    pub fn scale(&mut self, _value: f32) {
        unimplemented!()
    }

    /// Changes the sign of x and y.
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
    pub fn equals(&self, x: f32, y: f32) -> bool {
        self.x.fuzzy_equal(x) && self.y.fuzzy_equal(y)
    }

    /// Scales so that `length()` returns one, while preserving ratio, if possible.
    ///
    /// If original length is nearly zero, sets vec to (0, 0) and returns
    /// zero; otherwise, returns length of vec before vec is scaled.
    ///
    /// Returned prior length may be INFINITY if it can not be represented by float.
    /// Note that normalize() is faster if prior length is not required.
    pub fn normalize_todo(&mut self) -> f32 {
        unimplemented!()
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
    pub fn cross(&self, _other: Self) -> f32 {
        unimplemented!()
    }

    /// Returns the dot product of self and other vector.
    ///
    /// # Parameters
    /// - `vec` - right side of dot product
    #[must_use]
    pub fn dot(&self, _other: Self) -> f32 {
        unimplemented!()
    }
}

impl From<IPoint> for Point {
    #[allow(clippy::cast_precision_loss)]
    fn from(point: IPoint) -> Self {
        Self::make(point.x() as f32, point.y() as f32)
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

impl Mul<f32> for Point {
    type Output = Self;
    fn mul(self, scale: f32) -> Self {
        Self {
            x: self.x * scale,
            y: self.y * scale,
        }
    }
}

impl MulAssign<f32> for Point {
    fn mul_assign(&mut self, scale: f32) {
        self.x *= scale;
        self.y *= scale;
    }
}
