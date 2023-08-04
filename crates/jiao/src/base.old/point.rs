// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

use core::ops;
use serde::{Deserialize, Serialize};

use crate::util::{fuzzy_compare, fuzzy_is_zero};

/// The Point struct defines a point in the plane using integer precision.
///
/// A point is specified by a x coordinate and an y coordinate which can be accessed
/// using the `x()` and `y()` functions.
///
/// The `is_null()` function returns true if both x and y are set to 0.
///
/// The coordinates can be set (or altered) using the `set_x()` and `sety()` functions,
/// or alternatively the `x_mut()` and `y_mut()` functions which return references
/// to the coordinates (allowing direct manipulation).
///
/// A Point object can also be used as a vector: Addition and subtraction are defined as for vectors
/// (each component is added separately). A `QPoint` object can also be divided or
/// multiplied by an int or a qreal.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Point {
    x: i32,
    y: i32,
}

impl Point {
    /// Constructs a null point, i.e. with coordinates (0, 0)
    #[must_use]
    pub const fn new() -> Self {
        Self::from(0, 0)
    }

    /// Constructs a point with the given coordinates `(x, y)`.
    #[must_use]
    pub const fn from(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    /// Sets the x coordinate of this point to the given `x` coordinate.
    pub fn set_x(&mut self, x: i32) {
        self.x = x;
    }

    /// Returns the x coordinate of this point.
    #[must_use]
    pub const fn x(&self) -> i32 {
        self.x
    }

    /// Returns a mutable reference to the x coordinate of this point.
    pub fn x_mut(&mut self) -> &mut i32 {
        &mut self.x
    }

    /// Sets the y coordinate of this point to the given `y` coordinate.
    pub fn set_y(&mut self, y: i32) {
        self.y = y;
    }

    /// Returns the y coordinate of this point.
    #[must_use]
    pub const fn y(&self) -> i32 {
        self.y
    }

    /// Returns a mutable reference to the y coordinate of this point.
    pub fn y_mut(&mut self) -> &mut i32 {
        &mut self.y
    }

    /// Update x and y coordinates.
    pub fn set(&mut self, x: i32, y: i32) {
        self.x = x;
        self.y = y;
    }

    /// Returns the dot product of two points.
    #[must_use]
    pub const fn dot_product(&self, other: Self) -> i32 {
        self.x * other.x + self.y * other.y
    }

    /// Returns true if both the x and y coordinates are set to 0, otherwise returns false.
    #[must_use]
    pub const fn is_null(&self) -> bool {
        self.x == 0 && self.y == 0
    }

    /// Returns the sum of the absolute values of x() and y(),
    /// traditionally known as the "Manhattan length" of the vector from the origin to the point.
    #[must_use]
    pub const fn manhattan_length(&self) -> i32 {
        self.x.abs() + self.y.abs()
    }

    /// Returns a point with x and y coordinates exchanged:
    #[must_use]
    pub const fn transposed(&self) -> Self {
        Self {
            x: self.y,
            y: self.x,
        }
    }
}

impl From<(i32, i32)> for Point {
    fn from((x, y): (i32, i32)) -> Self {
        Self::from(x, y)
    }
}

impl From<&Point> for (i32, i32) {
    fn from(p: &Point) -> Self {
        (p.x, p.y)
    }
}

impl ops::Add for Point {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl ops::Sub for Point {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl ops::AddAssign for Point {
    fn add_assign(&mut self, other: Self) {
        self.x += other.x;
        self.y += other.y;
    }
}

impl ops::SubAssign for Point {
    fn sub_assign(&mut self, other: Self) {
        self.x -= other.x;
        self.y -= other.y;
    }
}

impl ops::MulAssign<i32> for Point {
    fn mul_assign(&mut self, factor: i32) {
        self.x *= factor;
        self.y *= factor;
    }
}

impl ops::MulAssign<f32> for Point {
    #[allow(clippy::cast_possible_truncation)]
    #[allow(clippy::cast_precision_loss)]
    fn mul_assign(&mut self, factor: f32) {
        self.x = (self.x as f32 * factor).round() as i32;
        self.y = (self.y as f32 * factor).round() as i32;
    }
}

impl ops::MulAssign<f64> for Point {
    #[allow(clippy::cast_possible_truncation)]
    #[allow(clippy::cast_precision_loss)]
    fn mul_assign(&mut self, factor: f64) {
        self.x = (f64::from(self.x) * factor).round() as i32;
        self.y = (f64::from(self.y) * factor).round() as i32;
    }
}

impl ops::DivAssign<f64> for Point {
    #[allow(clippy::cast_possible_truncation)]
    #[allow(clippy::cast_precision_loss)]
    fn div_assign(&mut self, factor: f64) {
        assert!(factor != 0.0);
        self.x = (f64::from(self.x) / factor).round() as i32;
        self.y = (f64::from(self.y) / factor).round() as i32;
    }
}

/// The `PointF` class defines a point in the plane using floating point precision.
///
/// A point is specified by a x coordinate and an y coordinate which can be accessed using
/// the `x()` and `y()` functions. The coordinates of the point are specified
/// using floating point numbers for accuracy.
///
/// The `is_null()` function returns true if both x and y are set to 0.0.
///
/// The coordinates can be set (or altered) using the `set_x()` and `set_y()` functions,
/// or alternatively the `x_mut()` and `y_mut()` functions which return
/// references to the coordinates (allowing direct manipulation).
#[allow(clippy::module_name_repetitions)]
#[derive(Debug, Default, Clone, Copy, Serialize, Deserialize)]
pub struct PointF {
    x: f64,
    y: f64,
}

impl PartialEq for PointF {
    fn eq(&self, other: &Self) -> bool {
        fuzzy_compare(self.x, other.x) && fuzzy_compare(self.y, other.y)
    }
}

impl PointF {
    /// Constructs a null point, i.e. with coordinates (0, 0)
    #[must_use]
    pub const fn new() -> Self {
        Self::from(0.0, 0.0)
    }

    /// Constructs a point with the given coordinates `(x, y)`.
    #[must_use]
    pub const fn from(x: f64, y: f64) -> Self {
        Self { x, y }
    }

    /// Sets the x coordinate of this point to the given `x` coordinate.
    pub fn set_x(&mut self, x: f64) {
        self.x = x;
    }

    /// Returns the x coordinate of this point.
    #[must_use]
    pub const fn x(&self) -> f64 {
        self.x
    }

    /// Returns a mutable reference to the x coordinate of this point.
    pub fn x_mut(&mut self) -> &mut f64 {
        &mut self.x
    }

    /// Sets the y coordinate of this point to the given `y` coordinate.
    pub fn set_y(&mut self, y: f64) {
        self.y = y;
    }

    /// Returns the y coordinate of this point.
    #[must_use]
    pub const fn y(&self) -> f64 {
        self.y
    }

    /// Returns a mutable reference to the y coordinate of this point.
    pub fn y_mut(&mut self) -> &mut f64 {
        &mut self.y
    }

    /// Update x and y coordinates.
    pub fn set(&mut self, x: f64, y: f64) {
        self.x = x;
        self.y = y;
    }

    /// Returns the dot product of two points.
    #[must_use]
    pub fn dot_product(&self, other: Self) -> f64 {
        self.x.mul_add(other.x, self.y * other.y)
    }

    /// Returns true if both the x and y coordinates are set to 0, otherwise returns false.
    #[must_use]
    pub fn is_null(&self) -> bool {
        self.x == 0.0 && self.y == 0.0
    }

    /// Returns the sum of the absolute values of x() and y(),
    /// traditionally known as the "Manhattan length" of the vector from the origin to the point.
    #[must_use]
    pub fn manhattan_length(&self) -> f64 {
        self.x.abs() + self.y.abs()
    }

    /// Rounds the coordinates of this point to the nearest integer,
    /// and returns a Point object with the rounded coordinates.
    #[must_use]
    pub fn to_point(&self) -> Point {
        #[allow(clippy::cast_possible_truncation)]
        Point::from(self.x.round() as i32, self.y.round() as i32)
    }

    /// Returns a point with x and y coordinates exchanged:
    #[must_use]
    pub const fn transposed(&self) -> Self {
        Self {
            x: self.y,
            y: self.x,
        }
    }
}

impl From<(f64, f64)> for PointF {
    fn from((x, y): (f64, f64)) -> Self {
        Self::from(x, y)
    }
}

impl From<&PointF> for (f64, f64) {
    fn from(p: &PointF) -> Self {
        (p.x, p.y)
    }
}

impl ops::Add for PointF {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl ops::Sub for PointF {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl ops::AddAssign for PointF {
    fn add_assign(&mut self, other: Self) {
        self.x += other.x;
        self.y += other.y;
    }
}

impl ops::SubAssign for PointF {
    fn sub_assign(&mut self, other: Self) {
        self.x -= other.x;
        self.y -= other.y;
    }
}

impl ops::Mul<f64> for PointF {
    type Output = Self;

    /// Returns a copy of the given point, multiplied by the given `factor`.
    fn mul(self, factor: f64) -> Self {
        Self::from(self.x * factor, self.y * factor)
    }
}

impl ops::MulAssign<f64> for PointF {
    fn mul_assign(&mut self, factor: f64) {
        self.x *= factor;
        self.y *= factor;
    }
}

impl ops::DivAssign<f64> for PointF {
    fn div_assign(&mut self, factor: f64) {
        assert!(!fuzzy_is_zero(factor));
        self.x /= factor;
        self.y /= factor;
    }
}

impl ops::Div<f64> for PointF {
    type Output = Self;

    /// Returns the `QPointF` object formed by dividing both components of the given point
    /// by the given divisor.
    fn div(self, factor: f64) -> Self {
        assert!(!fuzzy_is_zero(factor));
        Self::from(self.x / factor, self.y / factor)
    }
}

/*
cfg_if::cfg_if! {
    if #[cfg(feature = "skia")] {
        impl From<skia_safe::Point> for PointF {
            fn from(p: skia_safe::Point) -> Self {
                Self::from(f64::from(p.x), f64::from(p.y))
            }
        }

        impl From<PointF> for skia_safe::Point {
            fn from(p: PointF) -> Self {
                Self::new(p.x() as f32, p.y() as f32)
            }
        }
    }
}
*/
