// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

use core::ops;
use serde::{Deserialize, Serialize};

use super::vector3d::Vector3D;
use super::vector4d::Vector4D;
use crate::base::point::{Point, PointF};

/// The Vector2D class represents a vector or vertex in 2D space.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Vector2D {
    v: [f32; 2],
}

impl Default for Vector2D {
    fn default() -> Self {
        Self::new()
    }
}

impl Vector2D {
    /// Constructs a null vector, i.e. with coordinates (0, 0).
    pub fn new() -> Self {
        Self::from(0.0, 0.0)
    }

    /// Constructs a vector with coordinates (`x`, `y`).
    pub fn from(x: f32, y: f32) -> Self {
        Self { v: [x, y] }
    }

    /// Constructs a vector with x and y coordinates from a 4D vector.
    ///
    /// The z and w coordinates of vector are dropped.
    pub fn from_4d(vector: &Vector4D) -> Self {
        Self::from(vector.x(), vector.y())
    }

    /// Constructs a vector with x and y coordinates from a 3D vector.
    ///
    /// The z coordinate of vector is dropped.
    pub fn from_3d(vector: &Vector3D) -> Self {
        Self::from(vector.x(), vector.y())
    }

    /// Constructs a vector with x and y coordinates from a 2D point.
    pub fn from_pointf(point: &PointF) -> Self {
        Self::from(point.x() as f32, point.y() as f32)
    }

    /// Constructs a vector with x and y coordinates from a 2D point.
    pub fn from_point(point: &Point) -> Self {
        Self::from(point.x() as f32, point.y() as f32)
    }

    /// Returns the distance that this vertex is from a line defined by point and
    /// the unit vector direction.
    ///
    /// If `direction` is a null vector, then it does not define a line.
    /// In that case, the distance from `point` to this vertex is returned.
    pub fn distance_to_line(&self, point: &Self, direction: &Self) -> f32 {
        if direction.is_null() {
            return (self - point).length();
        }
        let p = (self - point).dot_product(direction) * direction + point;
        return (self - &p).length();
    }

    /// Returns the distance from this vertex to a point defined by the vertex point.
    pub fn distance_to_point(&self, point: &Self) -> f32 {
        (*self - point).length()
    }

    /// Returns the dot product of `self` and `vector`.
    pub fn dot_product(&self, vector: &Self) -> f32 {
        self.v[0] * vector.v[0] + self.v[1] * vector.v[1]
    }

    /// Returns true if the x and y coordinates are set to 0.0, otherwise returns false.
    pub fn is_null(&self) -> bool {
        self.x() == 0.0 && self.y() == 0.0
    }

    /// Returns the length of the vector from the origin.
    pub fn length(&self) -> f32 {
        let hypot = self.length_squared_precise();
        hypot.sqrt() as f32
    }

    /// Need some extra precision if the length is very small.
    fn length_squared_precise(&self) -> f64 {
        let x = self.v[0] as f64;
        let y = self.v[1] as f64;
        x * x + y * y
    }

    /// Returns the squared length of the vector from the origin.
    ///
    /// This is equivalent to the dot product of the vector with itself.
    pub fn length_squared(&self) -> f32 {
        self.v[0] * self.v[0] + self.v[1] * self.v[1]
    }

    /// Normalizes the currect vector in place.
    ///
    /// Nothing happens if this vector is a null vector or the length of the vector is very close to 1.
    pub fn normalize(&mut self) {
        let hypot = self.length_squared_precise();
        if hypot == 1.0 || hypot == 0.0 {
            return;
        }

        let sqrt = hypot.sqrt();
        self.v[0] = (self.v[0] as f64 / sqrt) as f32;
        self.v[1] = (self.v[1] as f64 / sqrt) as f32;
    }

    /// Returns the normalized unit vector form of this vector.
    ///
    /// If this vector is null, then a null vector is returned.
    /// If the length of the vector is very close to 1, then the vector will be returned as-is.
    /// Otherwise the normalized form of the vector of length 1 will be returned.
    pub fn normalized(&self) -> Self {
        let hypot = self.length_squared_precise();
        if hypot == 1.0 {
            return self.clone();
        } else if hypot == 0.0 {
            return Self::new();
        } else {
            let sqrt = hypot.sqrt();
            return Vector2D::from(
                (self.v[0] as f64 / sqrt) as f32,
                (self.v[1] as f64 / sqrt) as f32,
            );
        }
    }

    /// Sets the x coordinate of this point to the given x coordinate.
    pub fn set_x(&mut self, x: f32) {
        self.v[0] = x;
    }

    /// Sets the y coordinate of this point to the given y coordinate.
    pub fn set_y(&mut self, y: f32) {
        self.v[1] = y;
    }

    /// Returns the Point form of this 2D vector.
    pub fn to_point(&self) -> Point {
        Point::from(self.x().round() as i32, self.y().round() as i32)
    }

    /// Returns the PointF form of this 2D vector.
    pub fn to_pointf(&self) -> PointF {
        PointF::from(self.x() as f64, self.y() as f64)
    }

    /// Returns the 3D form of this 2D vector, with the z coordinate set to zero.
    pub fn to_vector3d(&self) -> Vector3D {
        Vector3D::from(self.v[0], self.v[1], 0.0)
    }

    /// Returns the 4D form of this 2D vector, with the z and w coordinates set to zero.
    pub fn to_vector4d(&self) -> Vector4D {
        Vector4D::from(self.v[0], self.v[1], 0.0, 0.0)
    }

    /// Returns the x coordinate of this point.
    pub fn x(&self) -> f32 {
        self.v[0]
    }

    /// Returns the y coordinate of this point.
    pub fn y(&self) -> f32 {
        self.v[1]
    }
}

impl ops::AddAssign<&Vector2D> for Vector2D {
    /// Adds the given vector to this vector.
    fn add_assign(&mut self, vector: &Vector2D) {
        self.v[0] += vector.v[0];
        self.v[1] += vector.v[1];
    }
}

impl ops::Add<&Vector2D> for Vector2D {
    type Output = Vector2D;

    /// Returns a Vector2D object that is the sum of the given vectors, `self` and `vector`;
    /// each component is added separately.
    fn add(self, vector: &Vector2D) -> Self::Output {
        Self::from(self.v[0] + vector.v[0], self.v[1] + vector.v[1])
    }
}

impl ops::Add<&Vector2D> for &Vector2D {
    type Output = Vector2D;

    /// Returns a Vector2D object that is the sum of the given vectors, `self` and `vector`;
    /// each component is added separately.
    fn add(self, vector: &Vector2D) -> Self::Output {
        Vector2D::from(self.v[0] + vector.v[0], self.v[1] + vector.v[1])
    }
}

impl ops::SubAssign<&Vector2D> for Vector2D {
    /// Subtracts the given vector from this vector.
    fn sub_assign(&mut self, vector: &Vector2D) {
        self.v[0] -= vector.v[0];
        self.v[1] -= vector.v[1];
    }
}

impl ops::Sub<&Vector2D> for Vector2D {
    type Output = Vector2D;

    /// Returns a QVector2D object that is formed by subtracting `vector` from `self`;
    /// each component is subtracted separately.
    fn sub(self, vector: &Vector2D) -> Self::Output {
        Self::from(self.v[0] - vector.v[0], self.v[1] - vector.v[1])
    }
}

impl ops::Sub<&Vector2D> for &Vector2D {
    type Output = Vector2D;

    /// Returns a QVector2D object that is formed by subtracting `vector` from `self`;
    /// each component is subtracted separately.
    fn sub(self, vector: &Vector2D) -> Self::Output {
        Vector2D::from(self.v[0] - vector.v[0], self.v[1] - vector.v[1])
    }
}

impl ops::Neg for &Vector2D {
    type Output = Vector2D;

    /// Returns a Vector2D object that is formed by changing the sign of the components of the given vector.
    ///
    /// Equivalent to Vector2D(0, 0) - vector.
    fn neg(self) -> Self::Output {
        Vector2D::from(-self.v[0], -self.v[1])
    }
}

impl ops::MulAssign<f32> for Vector2D {
    /// Multiplies this vector's coordinates by the given factor.
    fn mul_assign(&mut self, factor: f32) {
        self.v[0] *= factor;
        self.v[1] *= factor;
    }
}

impl ops::MulAssign<&Vector2D> for Vector2D {
    /// Multiplies the components of this vector by the corresponding components in vector.
    fn mul_assign(&mut self, vector: &Vector2D) {
        self.v[0] *= vector.v[0];
        self.v[1] *= vector.v[1];
    }
}

impl ops::Mul<f32> for Vector2D {
    type Output = Vector2D;

    /// Returns a copy of the given vector, multiplied by the given `factor`.
    fn mul(self, factor: f32) -> Self::Output {
        Self::from(self.v[0] * factor, self.v[1] * factor)
    }
}

impl ops::Mul<&Vector2D> for f32 {
    type Output = Vector2D;

    /// Returns a copy of the given vector, multiplied by the given `factor`.
    fn mul(self, vector: &Vector2D) -> Self::Output {
        Vector2D::from(self * vector.v[0], self * vector.v[1])
    }
}

impl ops::Mul<&Vector2D> for Vector2D {
    type Output = Vector2D;

    /// Multiplies the components of `self` by the corresponding components in `vector`.
    fn mul(self, vector: &Vector2D) -> Self::Output {
        Self::from(self.v[0] * vector.v[0], self.v[1] * vector.v[1])
    }
}

impl ops::DivAssign<f32> for Vector2D {
    /// Divides this vector's coordinates by the given `divisor`.
    fn div_assign(&mut self, divisor: f32) {
        assert!(divisor != 0.0);
        self.v[0] /= divisor;
        self.v[1] /= divisor;
    }
}

impl ops::DivAssign<&Vector2D> for Vector2D {
    /// Divides the components of this vector by the corresponding components in vector.
    fn div_assign(&mut self, vector: &Vector2D) {
        assert!(vector.v[0] != 0.0);
        assert!(vector.v[1] != 0.0);
        self.v[0] /= vector.v[0];
        self.v[1] /= vector.v[1];
    }
}

impl ops::Div<f32> for Vector2D {
    type Output = Vector2D;

    /// Returns the QVector2D object formed by dividing all three components
    /// of the given vector by the given `divisor`.
    fn div(self, divisor: f32) -> Self::Output {
        assert!(divisor != 0.0);
        Self::from(self.v[0] / divisor, self.v[1] / divisor)
    }
}

impl ops::Div<&Vector2D> for Vector2D {
    type Output = Vector2D;

    /// Returns the Vector2D object formed by dividing components of the given vector
    /// by a respective components of the given `divisor`.
    fn div(self, vector: &Vector2D) -> Self::Output {
        assert!(vector.v[0] != 0.0);
        assert!(vector.v[1] != 0.0);
        Self::from(self.v[0] / vector.v[0], self.v[1] / vector.v[1])
    }
}

impl ops::Index<usize> for Vector2D {
    type Output = f32;
    fn index(&self, index: usize) -> &Self::Output {
        assert!(index <= 1);
        &self.v[index]
    }
}

impl ops::IndexMut<usize> for Vector2D {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        assert!(index <= 1);
        &mut self.v[index]
    }
}
