// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

use core::ops;
use serde::{Deserialize, Serialize};

use super::vector2d::Vector2D;
use crate::base::point::{Point, PointF};

/// The Vector3D struct represents a vector or vertex in 3D space.
///
/// Vectors are one of the main building blocks of 3D representation and drawing.
/// They consist of three coordinates, traditionally called x, y, and z.
///
/// The Vector3D struct can also be used to represent vertices in 3D space.
/// We therefore do not need to provide a separate vertex class.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Vector3D {
    v: [f32; 3],
}

impl Default for Vector3D {
    fn default() -> Self {
        Self::new()
    }
}

impl Vector3D {
    /// Constructs a null vector, i.e. with coordinates (0, 0, 0).
    pub fn new() -> Self {
        Self::from(0.0, 0.0, 0.0)
    }

    /// Constructs a vector with coordinates (`x`, `y`, `z`).
    pub fn from(x: f32, y: f32, z: f32) -> Self {
        Self { v: [x, y, z] }
    }

    // Constructs a 3D vector from the specified 4D vector. The w coordinate is dropped.
    //pub fn from_vector4d(vector: Vector4D) -> Self {
    //}

    /// Constructs a 3D vector from the specified 2D vector.
    ///
    /// The z coordinate is set to `z`.
    pub fn from_vector2d_and_z(vector: Vector2D, z: f32) -> Self {
        Self::from(vector.x(), vector.y(), z)
    }

    /// Constructs a 3D vector from the specified 2D vector.
    ///
    /// The z coordinate is set to zero.
    pub fn from_vector2d(vector: Vector2D) -> Self {
        Self::from(vector.x(), vector.y(), 0.0)
    }

    /// Constructs a vector with x and y coordinates from a 2D point, and a z coordinate of 0.
    pub fn from_pointf(point: &PointF) -> Self {
        Self::from(point.x() as f32, point.y() as f32, 0.0)
    }

    /// Constructs a vector with x and y coordinates from a 2D point, and a z coordinate of 0.
    pub fn from_point(point: &Point) -> Self {
        Self::from(point.x() as f32, point.y() as f32, 0.0)
    }

    /// Returns the cross-product of vectors self and `vector`,
    /// which corresponds to the normal vector of a plane defined by self and `vector`.
    pub fn cross_product(&self, vector: &Self) -> Self {
        unimplemented!()
    }

    /// Returns the distance that this vertex is from a line defined by point and the unit vector direction.
    ///
    /// If `direction` is a null vector, then it does not define a line.
    /// In that case, the distance from `point` to this vertex is returned.
    pub fn distance_to_line(&self, point: &Self, direction: &Self) -> f32 {
        unimplemented!()
    }

    /// Returns the distance from this vertex to a plane defined by the vertex plane and
    /// a normal unit vector.
    ///
    /// The `normal` parameter is assumed to have been normalized to a unit vector.
    /// The return value will be negative if the vertex is below the `plane`,
    /// or zero if it is on the `plane`.
    pub fn distance_to_plane(&self, plane: &Self, normal: &Self) -> f32 {
        unimplemented!()
    }

    /// Returns the distance from this vertex to a plane defined by the vertices `plane1`,
    /// `plane2` and `plane3`.
    ///
    /// The return value will be negative if the vertex is below the plane,
    /// or zero if it is on the plane.
    ///
    /// The two vectors that define the plane are plane2 - plane1 and plane3 - plane1.
    pub fn distance_to_planes(&self, plane1: &Self, plane2: &Self, plane3: &Self) -> f32 {
        unimplemented!()
    }

    /// Returns the distance from this vertex to a point defined by the vertex `point`.
    pub fn distance_to_point(&self, point: &Self) -> f32 {
        unimplemented!()
    }

    /// Returns the dot product of self and `vector`.
    pub fn dot_product(&self, vector: &Self) -> f32 {
        unimplemented!()
    }

    /// Returns true if the x, y, and z coordinates are set to 0.0, otherwise returns false.
    pub fn is_null(&self) -> bool {
        unimplemented!()
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

    /// Returns the normal vector of a plane defined by vectors self and `vector`,
    /// normalized to be a unit vector.
    ///
    /// Use `cross_product()` to compute the cross-product of vectors if
    /// you do not need the result to be normalized to a unit vector.
    pub fn normal(&self, vector: &Self) -> Self {
        unimplemented!()
    }

    /// Returns the normal vector of a plane defined by vectors v2 - self and
    /// v3 - self, normalized to be a unit vector.
    ///
    /// Use `cross_product()` to compute the cross-product of v2 - self and v3 - self
    /// if you do not need the result to be normalized to a unit vector.
    pub fn normal3(&self, v2: &Self, v3: &Self) -> Self {
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
    /// If the length of the vector is very close to 1, then the vector
    /// will be returned as-is.
    /// Otherwise the normalized form of the vector of length 1 will be returned.
    pub fn normalized(&self) -> Self {
        unimplemented!()
    }

    // Returns the window coordinates of this vector initially in object/model coordinates
    // using the model view matrix modelView, the projection matrix projection and
    // the viewport dimensions viewport.
    //
    // When transforming from clip to normalized space, a division by the w component
    // on the vector components takes place.
    // To prevent dividing by 0 if w equals to 0, it is set to 1.
    //
    // Note: the returned y coordinates are in OpenGL orientation.
    // OpenGL expects the bottom to be 0.
    //pub fn project(&self, model_view: &Matrix4x4, projection: &Matrix4x4, viewport: &Rect) -> Self {
    //    unimplemented!()
    //}

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

    /// Returns the Point form of this 3D vector.
    ///
    /// The z coordinate is dropped.
    pub fn to_point(&self) -> Point {
        unimplemented!()
    }

    /// Returns the PointF form of this 3D vector.
    ///
    /// The z coordinate is dropped.
    pub fn to_pointf(&self) -> PointF {
        unimplemented!()
    }

    /// Returns the 2D vector form of this 3D vector, dropping the z coordinate.
    pub fn to_vector2d(&self) -> Vector2D {
        unimplemented!()
    }

    // Returns the 4D form of this 3D vector, with the w coordinate set to zero.
    //pub fn to_vector4d(&self) -> Vector4D {
    //    unimplemented!()
    //}

    // Returns the object/model coordinates of this vector initially in window coordinates
    // using the model view matrix modelView, the projection matrix projection and
    // the viewport dimensions viewport.
    //
    // When transforming from clip to normalized space, a division by the w component
    // of the vector components takes place.
    // To prevent dividing by 0 if w equals to 0, it is set to 1.
    //
    // Note: y coordinates in viewport should use OpenGL orientation.
    // OpenGL expects the bottom to be 0.
    //pub fn unproject(model_view: &QMatrix4x4, projection: &QMatrix4x4, viewport: &Rect) -> Self {
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

    /// Returns the z coordinate of this point.
    pub fn z(&self) -> f32 {
        self.v[2]
    }
}

impl ops::AddAssign<&Vector3D> for Vector3D {
    /// Adds the given vector to this vector.
    fn add_assign(&mut self, vector: &Self) {
        unimplemented!()
    }
}

impl ops::SubAssign<&Vector3D> for Vector3D {
    /// Subtracts the given vector from this vector.
    fn sub_assign(&mut self, vector: &Self) {
        unimplemented!()
    }
}

impl ops::MulAssign<f32> for Vector3D {
    /// Multiplies this vector's coordinates by the given factor.
    fn mul_assign(&mut self, factor: f32) {
        unimplemented!()
    }
}

impl ops::MulAssign<&Vector3D> for Vector3D {
    /// Multiplies the components of this vector by the corresponding components in vector.
    ///
    /// Note: this is not the same as the `cross_product()` of this vector and vector.
    fn mul_assign(&mut self, vector: &Self) {
        unimplemented!()
    }
}

impl ops::DivAssign<f32> for Vector3D {
    /// Divides this vector's coordinates by the given divisor.
    fn div_assign(&mut self, divisor: f32) {
        unimplemented!()
    }
}

impl ops::DivAssign<&Vector3D> for Vector3D {
    /// Divides the components of this vector by the corresponding components in vector.
    fn div_assign(&mut self, vector: &Self) {
        unimplemented!()
    }
}

impl ops::Index<usize> for Vector3D {
    type Output = f32;

    /// Returns the component of the vector at index position `index` as a reference.
    ///
    /// `index` must be a valid index position in the vector (i.e., 0 <= index < 3).
    fn index(&self, index: usize) -> &Self::Output {
        unimplemented!()
    }
}

impl ops::IndexMut<usize> for Vector3D {
    /// Returns the component of the vector at index position `index` as a mutable reference.
    ///
    /// `index` must be a valid index position in the vector (i.e., 0 <= index < 3).
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        unimplemented!()
    }
}

impl ops::Add<&Vector3D> for Vector3D {
    type Output = Vector3D;

    /// Returns a Vector3D object that is the sum of the given vectors,
    /// each component is added separately.
    fn add(self, vector: &Vector3D) -> Self::Output {
        unimplemented!()
    }
}

impl ops::Add<&Vector3D> for &Vector3D {
    type Output = Vector3D;

    /// Returns a Vector3D object that is the sum of the given vectors,
    /// each component is added separately.
    fn add(self, vector: &Vector3D) -> Self::Output {
        unimplemented!()
    }
}

impl ops::Sub<&Vector3D> for Vector3D {
    type Output = Vector3D;

    /// Returns a Vector3D object that is formed by subtracting vector from self;
    /// each component is subtracted separately.
    fn sub(self, vector: &Vector3D) -> Self::Output {
        unimplemented!()
    }
}

impl ops::Sub<&Vector3D> for &Vector3D {
    type Output = Vector3D;

    /// Returns a Vector3D object that is formed by subtracting vector from self;
    /// each component is subtracted separately.
    fn sub(self, vector: &Vector3D) -> Self::Output {
        unimplemented!()
    }
}

impl ops::Neg for &Vector3D {
    type Output = Vector3D;

    /// Returns a Vector3D object that is formed by changing the sign of all three components
    /// of the given vector.
    ///
    /// Equivalent to Vector3D(0, 0, 0) - vector.
    fn neg(self) -> Self::Output {
        unimplemented!()
    }
}

impl ops::Mul<&Vector3D> for f32 {
    type Output = Vector3D;

    /// Returns a copy of the given vector, multiplied by the given factor.
    fn mul(self, vector: &Vector3D) -> Self::Output {
        unimplemented!()
    }
}

impl ops::Mul<f32> for Vector3D {
    type Output = Vector3D;

    /// Returns a copy of the given vector, multiplied by the given factor.
    fn mul(self, factor: f32) -> Self::Output {
        unimplemented!()
    }
}

impl ops::Mul<f32> for &Vector3D {
    type Output = Vector3D;

    /// Returns a copy of the given vector, multiplied by the given factor.
    fn mul(self, factor: f32) -> Self::Output {
        unimplemented!()
    }
}

impl ops::Mul<&Vector3D> for Vector3D {
    type Output = Vector3D;

    /// Multiplies the components of self by the corresponding components in vector.
    ///
    /// Note: this is not the same as the `cross_product()` of self and vector.
    fn mul(self, vector: &Vector3D) -> Self::Output {
        unimplemented!()
    }
}

impl ops::Mul<&Vector3D> for &Vector3D {
    type Output = Vector3D;

    /// Multiplies the components of self by the corresponding components in vector.
    ///
    /// Note: this is not the same as the `cross_product()` of self and vector.
    fn mul(self, vector: &Vector3D) -> Self::Output {
        unimplemented!()
    }
}

impl ops::Div<f32> for Vector3D {
    type Output = Vector3D;

    /// Returns the Vector3D object formed by dividing all three components
    /// of the given vector by the given divisor.
    fn div(self, divisor: f32) -> Self::Output {
        unimplemented!()
    }
}

impl ops::Div<f32> for &Vector3D {
    type Output = Vector3D;

    /// Returns the Vector3D object formed by dividing all three components
    /// of the given vector by the given divisor.
    fn div(self, divisor: f32) -> Self::Output {
        unimplemented!()
    }
}

impl ops::Div<&Vector3D> for Vector3D {
    type Output = Vector3D;

    /// Returns the Vector3D object formed by dividing components of the given vector
    /// by a respective components of the given divisor.
    fn div(self, vector: &Vector3D) -> Self::Output {
        unimplemented!()
    }
}

impl ops::Div<&Vector3D> for &Vector3D {
    type Output = Vector3D;

    /// Returns the Vector3D object formed by dividing components of the given vector
    /// by a respective components of the given divisor.
    fn div(self, vector: &Vector3D) -> Self::Output {
        unimplemented!()
    }
}
