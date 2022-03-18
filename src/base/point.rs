// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

use core::ops;
use serde::{Deserialize, Serialize};

/// The Point struct defines a point in the plane using integer precision.
///
/// A point is specified by a x coordinate and an y coordinate which can be accessed
/// using the `x()` and `y()` functions.
///
/// The `is_null()` function returns true if both x and y are set to 0.
///
/// The coordinates can be set (or altered) using the `set_x()` and `sety()` functions,
/// or alternatively the `rx()` and `ry()` functions which return references
/// to the coordinates (allowing direct manipulation).
///
/// A Point object can also be used as a vector: Addition and subtraction are defined as for vectors
/// (each component is added separately). A QPoint object can also be divided or
/// multiplied by an int or a qreal.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Point {
    x: i32,
    y: i32,
}

impl Point {
    /// Constructs a null point, i.e. with coordinates (0, 0)
    pub fn new() -> Self {
        Self::from(0, 0)
    }

    /// Constructs a point with the given coordinates `(x, y)`.
    pub fn from(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    /// Sets the x coordinate of this point to the given `x` coordinate.
    pub fn set_x(&mut self, x: i32) {
        self.x = x;
    }

    /// Returns the x coordinate of this point.
    pub fn x(&self) -> i32 {
        self.x
    }

    /// Returns a reference to the x coordinate of this point.
    pub fn rx(&mut self) -> &mut i32 {
        &mut self.x
    }

    /// Sets the y coordinate of this point to the given `y` coordinate.
    pub fn set_y(&mut self, y: i32) {
        self.y = y;
    }

    /// Returns the y coordinate of this point.
    pub fn y(&self) -> i32 {
        self.y
    }

    /// Returns a reference to the y coordinate of this point.
    pub fn ry(&mut self) -> &mut i32 {
        &mut self.y
    }

    /// Update x and y coordinates.
    pub fn set(&mut self, x: i32, y: i32) {
        self.x = x;
        self.y = y;
    }

    /// Returns the dot product of two points.
    pub fn dot_product(&self, other: Point) -> i32 {
        self.x * other.x + self.y * other.y
    }

    /// Returns true if both the x and y coordinates are set to 0, otherwise returns false.
    pub fn is_null(&self) -> bool {
        self.x == 0 && self.y == 0
    }

    /// Returns the sum of the absolute values of x() and y(),
    /// traditionally known as the "Manhattan length" of the vector from the origin to the point.
    pub fn manhattan_length(&self) -> i32 {
        self.x.abs() + self.y.abs()
    }

    /// Returns a point with x and y coordinates exchanged:
    pub fn transposed(&self) -> Self {
        Self {
            x: self.y,
            y: self.x,
        }
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
    fn mul_assign(&mut self, factor: f32) {
        self.x = (self.x as f32 * factor) as i32;
        self.y = (self.y as f32 * factor) as i32;
    }
}

impl ops::MulAssign<f64> for Point {
    fn mul_assign(&mut self, factor: f64) {
        self.x = (self.x as f64 * factor) as i32;
        self.y = (self.y as f64 * factor) as i32;
    }
}

impl ops::DivAssign<f64> for Point {
    fn div_assign(&mut self, factor: f64) {
        assert!(factor != 0.0);
        self.x = (self.x as f64 / factor) as i32;
        self.y = (self.y as f64 / factor) as i32;
    }
}

/// The PointF class defines a point in the plane using floating point precision.
///
/// A point is specified by a x coordinate and an y coordinate which can be accessed using
/// the `x()` and `y()` functions. The coordinates of the point are specified
/// using floating point numbers for accuracy.
///
/// The `is_null()` function returns true if both x and y are set to 0.0.
///
/// The coordinates can be set (or altered) using the `set_x()` and `set_y()` functions,
/// or alternatively the `rx()` and `ry()` functions which return
/// references to the coordinates (allowing direct manipulation).
#[derive(Debug, Default, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct PointF {
    x: f64,
    y: f64,
}

impl PointF {
    /// Constructs a null point, i.e. with coordinates (0, 0)
    pub fn new() -> Self {
        Self::from(0.0, 0.0)
    }

    /// Constructs a point with the given coordinates `(x, y)`.
    pub fn from(x: f64, y: f64) -> Self {
        Self { x, y }
    }

    /// Sets the x coordinate of this point to the given `x` coordinate.
    pub fn set_x(&mut self, x: f64) {
        self.x = x;
    }

    /// Returns the x coordinate of this point.
    pub fn x(&self) -> f64 {
        self.x
    }

    /// Returns a reference to the x coordinate of this point.
    pub fn rx(&mut self) -> &mut f64 {
        &mut self.x
    }

    /// Sets the y coordinate of this point to the given `y` coordinate.
    pub fn set_y(&mut self, y: f64) {
        self.y = y;
    }

    /// Returns the y coordinate of this point.
    pub fn y(&self) -> f64 {
        self.y
    }

    /// Returns a reference to the y coordinate of this point.
    pub fn ry(&mut self) -> &mut f64 {
        &mut self.y
    }

    /// Update x and y coordinates.
    pub fn set(&mut self, x: f64, y: f64) {
        self.x = x;
        self.y = y;
    }

    /// Returns the dot product of two points.
    pub fn dot_product(&self, other: PointF) -> f64 {
        self.x * other.x + self.y * other.y
    }

    /// Returns true if both the x and y coordinates are set to 0, otherwise returns false.
    pub fn is_null(&self) -> bool {
        self.x == 0.0 && self.y == 0.0
    }

    /// Returns the sum of the absolute values of x() and y(),
    /// traditionally known as the "Manhattan length" of the vector from the origin to the point.
    pub fn manhattan_length(&self) -> f64 {
        self.x.abs() + self.y.abs()
    }

    /// Returns a point with x and y coordinates exchanged:
    pub fn transposed(&self) -> Self {
        Self {
            x: self.y,
            y: self.x,
        }
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

impl ops::MulAssign<f64> for PointF {
    fn mul_assign(&mut self, factor: f64) {
        self.x *= factor;
        self.y *= factor;
    }
}

impl ops::DivAssign<f64> for PointF {
    fn div_assign(&mut self, factor: f64) {
        assert!(factor != 0.0);
        self.x /= factor;
        self.y /= factor;
    }
}
