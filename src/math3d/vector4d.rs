// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

use core::ops;
use serde::{Deserialize, Serialize};

use super::vector2d::Vector2D;
use super::vector3d::Vector3D;
use crate::base::point::{Point, PointF};

/// The Vector4D struct represents a vector or vertex in 4D space.
pub struct Vector4D {
    v: [f32; 4],
}

impl Default for Vector4D {
    fn default() -> Self {
        Self::new()
    }
}

impl Vector4D {
    /// Constructs a null vector, i.e. with coordinates (0, 0, 0, 0).
    pub fn new() -> Self {
        Self::from(0.0, 0.0, 0.0, 0.0)
    }

    /// Constructs a vector with coordinates (`x`, `y`, `z`, `w`).
    pub fn from(x: f32, y: f32, z: f32, w: f32) -> Self {
        Self { v: [x, y, z, w] }
    }

    /// Constructs a 4D vector from the specified 3D vector.
    ///
    /// The w coordinate is set to `w`.
    pub fn from_vector3d(vector: &Vector3D) -> Self {
        Self::from(vector.x(), vector.y(), vector.z(), 0.0)
    }

    /// Constructs a 4D vector from the specified 3D vector.
    ///
    /// The w coordinate is set to `w`.
    pub fn from_vector3d_and_w(vector: &Vector3D, w: f32) -> Self {
        Self::from(vector.x(), vector.y(), vector.z(), w)
    }

    /// Constructs a 4D vector from the specified 2D vector.
    ///
    /// The z and w coordinates are set to zero.
    pub fn from_vector2d(vector: &Vector2D) -> Self {
        Self::from(vector.x(), vector.y(), 0.0, 0.0)
    }

    /// Constructs a 4D vector from the specified 2D vector.
    /// The z and w coordinates are set to `z` and `w` respectively.
    pub fn from_vector2d_and_zw(vector: &Vector2D, z: f32, w: f32) -> Self {
        Self::from(vector.x(), vector.y(), z, w)
    }

    /// Constructs a vector with x and y coordinates from a 2D point, and z and w coordinates of 0.
    pub fn from_point(point: &Point) -> Self {
        Self::from(point.x() as f32, point.y() as f32, 0.0, 0.0)
    }

    /// Constructs a vector with x and y coordinates from a 2D point, and z and w coordinates of 0.
    pub fn from_pointf(point: &PointF) -> Self {
        Self::from(point.x() as f32, point.y() as f32, 0.0, 0.0)
    }

    /// Returns the dot product of self and `vector`.
    pub fn dot_product(&self, vector: &Self) -> f32 {
        unimplemented!()
    }

    /// Returns true if the x, y, z, and w coordinates are set to 0.0, otherwise returns false.
    pub fn is_null(&self) -> bool {
        self.x() == 0.0 && self.y() == 0.0 && self.z() == 0.0 && self.w() == 0.0
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

    /// Sets the w coordinate of this point to the given w coordinate.
    pub fn set_w(&mut self, w: f32) {
        self.v[3] = w;
    }

    /// Sets the x coordinate of this point to the given x coordinate.
    pub fn set_x(&mut self, x: f32) {
        self.v[0] = x;
    }

    /// Sets the y coordinate of this point to the given y coordinate.
    pub fn set_y(&mut self, y: f32) {
        self.v[1] = y;
    }

    /// Sets the z coordinate of this point to the given z coordinate.
    pub fn set_z(&mut self, z: f32) {
        self.v[2] = z;
    }

    /// Returns the Point form of this 4D vector.
    ///
    /// The z and w coordinates are dropped.
    pub fn to_point(&self) -> Point {
        Point::from(self.x().round() as i32, self.y().round() as i32)
    }

    /// Returns the PointF form of this 4D vector.
    ///
    /// The z and w coordinates are dropped.
    pub fn to_pointf(&self) -> PointF {
        PointF::from(self.x() as f64, self.y() as f64)
    }

    /// Returns the 2D vector form of this 4D vector, dropping the z and w coordinates.
    pub fn to_vector2d(&self) -> Vector2D {
        Vector2D::from(self.x(), self.y())
    }

    /// Returns the 2D vector form of this 4D vector, dividing the x and y coordinates
    /// by the w coordinate and dropping the z coordinate.
    ///
    /// Returns a null vector if w is zero.
    pub fn to_vector2d_affine(&self) -> Vector2D {
        unimplemented!()
    }

    /// Returns the 3D vector form of this 4D vector, dropping the w coordinate.
    pub fn to_vector3d(&self) -> Vector3D {
        Vector3D::from(self.x(), self.y(), self.z())
    }

    /// Returns the 3D vector form of this 4D vector, dividing the x, y, and z coordinates
    /// by the w coordinate.
    ///
    /// Returns a null vector if w is zero.
    pub fn to_vector3d_affine(&self) -> Vector3D {
        unimplemented!()
    }

    /// Returns the w coordinate of this point.
    pub fn w(&self) -> f32 {
        self.v[3]
    }

    /// Returns the x coordinate of this point.
    pub fn x(&self) -> f32 {
        self.v[0]
    }

    /// Returns the y coordinate of this point.
    pub fn y(&self) -> f32 {
        self.v[1]
    }

    /// Returns the z coordinate of this point.
    pub fn z(&self) -> f32 {
        self.v[2]
    }
}

impl ops::AddAssign<&Vector4D> for Vector4D {
    /// Adds the given vector to this vector.
    fn add_assign(&mut self, vector: &Vector4D) {
        unimplemented!()
    }
}

impl ops::SubAssign<&Vector4D> for Vector4D {
    /// Subtracts the given vector from this vector.
    fn sub_assign(&mut self, vector: &Vector4D) {
        unimplemented!()
    }
}

impl ops::MulAssign<f32> for Vector4D {
    /// Multiplies this vector's coordinates by the given factor.
    fn mul_assign(&mut self, factor: f32) {
        unimplemented!()
    }
}

impl ops::MulAssign<&Vector4D> for Vector4D {
    /// Multiplies the components of this vector by the corresponding components in vector.
    fn mul_assign(&mut self, vector: &Vector4D) {
        unimplemented!()
    }
}

impl ops::DivAssign<f32> for Vector4D {
    /// Divides this vector's coordinates by the given divisor.
    fn div_assign(&mut self, divisor: f32) {
        unimplemented!()
    }
}

impl ops::DivAssign<&Vector4D> for Vector4D {
    /// Divides the components of this vector by the corresponding components in vector.
    fn div_assign(&mut self, divisor: &Vector4D) {
        unimplemented!()
    }
}

impl ops::Index<usize> for Vector4D {
    type Output = f32;

    /// Returns the component of the vector at index position `index` as a reference.
    ///
    /// `index` must be a valid index position in the vector (i.e., 0 <= index < 4).
    fn index(&self, index: usize) -> &Self::Output {
        assert!(index < 4);
        &self.v[index]
    }
}

impl ops::IndexMut<usize> for Vector4D {
    /// Returns the component of the vector at index position `index` as a mutable reference.
    ///
    /// `index` must be a valid index position in the vector (i.e., 0 <= index < 4).
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        assert!(index < 4);
        &mut self.v[index]
    }
}

impl ops::Add<&Vector4D> for Vector4D {
    type Output = Vector4D;

    /// Returns a Vector4D object that is the sum of the given vectors,
    /// each component is added separately.
    fn add(self, vector: &Vector4D) -> Self::Output {
        unimplemented!()
    }
}

impl ops::Add<&Vector4D> for &Vector4D {
    type Output = Vector4D;

    /// Returns a Vector4D object that is the sum of the given vectors,
    /// each component is added separately.
    fn add(self, vector: &Vector4D) -> Self::Output {
        unimplemented!()
    }
}

impl ops::Sub<&Vector4D> for Vector4D {
    type Output = Vector4D;

    /// Returns a Vector4D object that is the sub of the given vectors,
    /// each component is added separately.
    fn sub(self, vector: &Vector4D) -> Self::Output {
        unimplemented!()
    }
}

impl ops::Sub<&Vector4D> for &Vector4D {
    type Output = Vector4D;

    /// Returns a Vector4D object that is the sub of the given vectors,
    /// each component is added separately.
    fn sub(self, vector: &Vector4D) -> Self::Output {
        unimplemented!()
    }
}

impl ops::Neg for Vector4D {
    type Output = Vector4D;

    /// Returns a Vector4D object that is formed by changing the sign of all three components
    /// of the given vector.
    ///
    /// Equivalent to Vector4D(0,0,0,0) - vector.
    fn neg(self) -> Self::Output {
        unimplemented!()
    }
}

impl ops::Neg for &Vector4D {
    type Output = Vector4D;

    /// Returns a Vector4D object that is formed by changing the sign of all three components
    /// of the given vector.
    ///
    /// Equivalent to Vector4D(0,0,0,0) - vector.
    fn neg(self) -> Self::Output {
        unimplemented!()
    }
}

impl ops::Mul<&Vector4D> for f32 {
    type Output = Vector4D;

    /// Returns a copy of the given vector, multiplied by the given factor.
    fn mul(self, vector: &Vector4D) -> Self::Output {
        unimplemented!()
    }
}

impl ops::Mul<f32> for Vector4D {
    type Output = Vector4D;

    /// Returns a copy of the given vector, multiplied by the given factor.
    fn mul(self, factor: f32) -> Self::Output {
        unimplemented!()
    }
}

impl ops::Mul<f32> for &Vector4D {
    type Output = Vector4D;

    /// Returns a copy of the given vector, multiplied by the given factor.
    fn mul(self, factor: f32) -> Self::Output {
        unimplemented!()
    }
}

impl ops::Mul<&Vector4D> for Vector4D {
    type Output = Vector4D;

    /// Returns a copy of the given vector, multiplied by the given factor.
    fn mul(self, vector: &Vector4D) -> Self::Output {
        unimplemented!()
    }
}

impl ops::Mul<&Vector4D> for &Vector4D {
    type Output = Vector4D;

    /// Returns a copy of the given vector, multiplied by the given factor.
    fn mul(self, vector: &Vector4D) -> Self::Output {
        unimplemented!()
    }
}

impl ops::Div<f32> for Vector4D {
    type Output = Vector4D;

    /// Returns the Vector4D object formed by dividing all four components of
    /// the given vector by the given divisor.
    fn div(self, divisor: f32) -> Self::Output {
        unimplemented!()
    }
}

impl ops::Div<f32> for &Vector4D {
    type Output = Vector4D;

    /// Returns the Vector4D object formed by dividing all four components of
    /// the given vector by the given divisor.
    fn div(self, divisor: f32) -> Self::Output {
        unimplemented!()
    }
}

impl ops::Div<&Vector4D> for Vector4D {
    type Output = Vector4D;

    /// Returns the Vector4D object formed by dividing all four components of
    /// the given vector by the given divisor.
    fn div(self, vector: &Vector4D) -> Self::Output {
        unimplemented!()
    }
}

impl ops::Div<&Vector4D> for &Vector4D {
    type Output = Vector4D;

    /// Returns the Vector4D object formed by dividing all four components of
    /// the given vector by the given divisor.
    fn div(self, vector: &Vector4D) -> Self::Output {
        unimplemented!()
    }
}
