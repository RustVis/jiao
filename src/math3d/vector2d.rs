// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

use core::ops;
use serde::{Deserialize, Serialize};

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

    // Constructs a vector with x and y coordinates from a 3D vector.
    //
    // The z and w coordinates of vector are dropped.
    //pub fn from_4d(vector: &Vector4D) -> Self {
    //    unimplemented!()
    //}
    // Constructs a vector with x and y coordinates from a 3D vector.
    //
    // The z coordinate of vector is dropped.
    //pub fn from_3d(vector: &Vector3D) -> Self {
    //}

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
        unimplemented!()
    }

    /// Returns the distance from this vertex to a point defined by the vertex point.
    pub fn distance_to_point(&self, point: &Self) -> f32 {
        unimplemented!()
    }

    /// Returns the dot product of v1 and v2.
    pub fn dot_product(&self, other: &Self) -> f32 {
        unimplemented!()
    }

    /// Returns true if the x and y coordinates are set to 0.0, otherwise returns false.
    pub fn is_null(&self) -> bool {
        self.x() == 0.0 && self.y() == 0.0
    }

    /// Returns the length of the vector from the origin.
    pub fn length(&self) -> f32 {
        unimplemented!()
    }

    /// Returns the squared length of the vector from the origin.
    ///
    /// This is equivalent to the dot product of the vector with itself.
    pub fn length_squared(&self) -> f32 {
        unimplemented!()
    }

    /// Normalizes the currect vector in place.
    ///
    /// Nothing happens if this vector is a null vector or the length of the vector is very close to 1.
    pub fn normalize(&mut self) {
        unimplemented!()
    }

    /// Returns the normalized unit vector form of this vector.
    ///
    /// If this vector is null, then a null vector is returned.
    /// If the length of the vector is very close to 1, then the vector will be returned as-is.
    /// Otherwise the normalized form of the vector of length 1 will be returned.
    pub fn normalized(&self) -> Self {
        unimplemented!()
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
        unimplemented!()
    }

    /// Returns the PointF form of this 2D vector.
    pub fn to_pointf(&self) -> PointF {
        unimplemented!()
    }

    // Returns the 3D form of this 2D vector, with the z coordinate set to zero.
    //pub fn to_vector3d(&self) -> Vector3D {
    //    unimplemented!()
    //}

    // Returns the 4D form of this 2D vector, with the z and w coordinates set to zero.
    //pub fn to_vector4d(&self) -> Vector4D {
    //    unimplemented!()
    //}

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
    /// Adds the given vector to this vector and returns a reference to this vector.
    fn add_assign(&mut self, vector: &Vector2D) {
        unimplemented!()
    }
}

impl ops::Add<&Vector2D> for Vector2D {
    type Output = Vector2D;

    /// Returns a Vector2D object that is the sum of the given vectors, `self` and `vector`;
    /// each component is added separately.
    fn add(self, vector: &Vector2D) -> Self::Output {
        unimplemented!()
    }
}

impl ops::SubAssign<&Vector2D> for Vector2D {
    /// Subtracts the given vector from this vector and returns a reference to this vector.
    fn sub_assign(&mut self, vector: &Vector2D) {
        unimplemented!()
    }
}

impl ops::Sub<&Vector2D> for Vector2D {
    type Output = Vector2D;

    /// Returns a QVector2D object that is formed by subtracting `vector` from `self`;
    /// each component is subtracted separately.
    fn sub(self, vector: &Vector2D) -> Self::Output {
        unimplemented!()
    }
}

impl ops::MulAssign<f32> for Vector2D {
    /// Multiplies this vector's coordinates by the given factor, and returns a reference to this vector.
    fn mul_assign(&mut self, factor: f32) {
        unimplemented!()
    }
}

impl ops::MulAssign<&Vector2D> for Vector2D {
    /// Multiplies the components of this vector by the corresponding components in vector.
    fn mul_assign(&mut self, vector: &Vector2D) {
        unimplemented!()
    }
}

impl ops::Mul<f32> for Vector2D {
    type Output = Vector2D;

    /// Returns a copy of the given vector, multiplied by the given `factor`.
    fn mul(self, factor: f32) -> Self::Output {
        unimplemented!()
    }
}

impl ops::Mul<Vector2D> for f32 {
    type Output = Vector2D;

    /// Returns a copy of the given vector, multiplied by the given `factor`.
    fn mul(self, vector: Vector2D) -> Self::Output {
        unimplemented!()
    }
}

impl ops::Mul<&Vector2D> for Vector2D {
    type Output = Vector2D;

    /// Multiplies the components of `self` by the corresponding components in `vector`.
    fn mul(self, vector: &Vector2D) -> Self::Output {
        unimplemented!()
    }
}

impl ops::DivAssign<f32> for Vector2D {
    /// Divides this vector's coordinates by the given `divisor`, and returns a reference to this vector.
    fn div_assign(&mut self, divisor: f32) {
        unimplemented!()
    }
}

impl ops::DivAssign<&Vector2D> for Vector2D {
    /// Divides the components of this vector by the corresponding components in vector.
    fn div_assign(&mut self, vector: &Vector2D) {
        unimplemented!()
    }
}

impl ops::Div<f32> for Vector2D {
    type Output = Vector2D;

    /// Returns the QVector2D object formed by dividing all three components
    /// of the given vector by the given `divisor`.
    fn div(self, divisor: f32) -> Self::Output {
        unimplemented!()
    }
}

impl ops::Div<&Vector2D> for Vector2D {
    type Output = Vector2D;

    /// Returns the Vector2D object formed by dividing components of the given vector
    /// by a respective components of the given `divisor`.
    fn div(self, divisor: &Vector2D) -> Self::Output {
        unimplemented!()
    }
}

impl ops::Index<usize> for Vector2D {
    type Output = f32;
    fn index(&self, index: usize) -> &Self::Output {
        assert!(index >= 0 && index <= 1);
        &self.v[index]
    }
}

impl ops::IndexMut<usize> for Vector2D {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        assert!(index >= 0 && index <= 1);
        &mut self.v[index]
    }
}
