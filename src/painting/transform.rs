// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

use core::ops;

use crate::base::axis::Axis;
use crate::base::line::{Line, LineF};
use crate::base::point::{Point, PointF};
use crate::base::rect::{Rect, RectF};

/// The Transform struct specifies 2D transformations of a coordinate system.
///
/// A transformation specifies how to translate, scale, shear, rotate or
/// project the coordinate system, and is typically used when rendering graphics.
///
/// `Transform` allows perspective transformations.
///
/// A Transform object can be built using the `set_matrix()`, `scale()`, `rotate()`,
/// `translate()` and `shear()` functions.
/// Alternatively, it can be built by applying basic matrix operations.
/// The matrix can be reset to the identity matrix (the default) using the `reset()` function.
///
/// The Transform struct supports mapping of graphic primitives: A given point,
/// line, polygon, region, or painter path can be mapped to the coordinate system
/// defined by this matrix using the `map()` function.
/// In case of a rectangle, its coordinates can be transformed using the `map_rect()` function.
/// A rectangle can also be transformed into a polygon
/// (mapped to the coordinate system defined by this matrix),
/// using the `map_to_polygon()` function.
///
/// Transform provides the `is_identity()` function which returns true if
/// the matrix is the identity matrix, and the `is_invertible()` function
/// which returns true if the matrix is non-singular (i.e. AB = BA = I).
/// The `inverted()` function returns an inverted copy of this matrix
/// if it is invertible (otherwise it returns the identity matrix),
/// and `adjoint()` returns the matrix's classical adjoint.
/// In addition, QTransform provides the `determinant()` function
/// which returns the matrix's determinant.
///
/// Finally, the Transform struct supports matrix multiplication,
/// addition and subtraction, and objects of the class can be streamed as well as compared.
#[derive(Debug, Clone, PartialEq)]
pub struct Transform {
    m11: f64,
    m12: f64,
    m13: f64,
    m21: f64,
    m22: f64,
    m23: f64,
    m31: f64, // alias of dx
    m32: f64, // alias of dy
    m33: f64,

    dirty: TransformationType,
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
#[repr(u8)]
pub enum TransformationType {
    None = 0x00,
    Translate = 0x01,
    Scale = 0x02,
    Rotate = 0x04,
    Shear = 0x08,
    Project = 0x10,
}

impl Transform {
    /// Constructs an identity matrix.
    ///
    /// All elements are set to zero except m11 and m22 (specifying the scale) and m33 which are set to 1.
    pub fn new() -> Self {
        unimplemented!()
    }

    /// Constructs a matrix with the elements, `m11`, `m12`, `m21`, `m22`, `dx` and `dy`.
    pub fn new_2d(_m11: f64, _m12: f64, _m21: f64, _m22: f64, _dx: f64, _dy: f64) {
        unimplemented!()
    }

    /// Constructs a matrix with the elements, `m11`, `m12`, `m13`, `m21`, `m22`, `m23`, `m31`, `m32`, `m33`.
    pub fn new_3d(
        _m11: f64,
        _m12: f64,
        _m13: f64,
        _m21: f64,
        _m22: f64,
        _m23: f64,
        _m31: f64,
        _m32: f64,
        _m33: f64,
    ) -> Self {
        unimplemented!()
    }

    /// Creates a matrix which corresponds to a scaling of `sx` horizontally and
    /// `sy` vertically.
    ///
    /// This is the same as `Transform::new().scale(sx, sy)` but slightly faster.
    pub fn from_scale(sx: f64, sy: f64) -> Self {
        unimplemented!()
    }

    /// Creates a matrix which corresponds to a translation of `dx` along the x axis and
    /// `dy` along the y axis.
    ///
    /// This is the same as `Transform::new().translate(dx, dy)` but slightly faster.
    pub fn from_translate(dx: f64, dy: f64) -> Self {
        unimplemented!()
    }

    /// Returns the horizontal scaling factor.
    pub fn m11(&self) -> f64 {
        self.m11
    }

    /// Returns the vertical shearing factor.
    pub fn m12(&self) -> f64 {
        self.m12
    }

    /// Returns the horizontal projection factor.
    pub fn m13(&self) -> f64 {
        self.m13
    }

    /// Returns the horizontal shearing factor.
    pub fn m21(&self) -> f64 {
        self.m21
    }

    /// Returns the vertical scaling factor.
    pub fn m22(&self) -> f64 {
        self.m22
    }

    /// Returns the vertical projection factor.
    pub fn m23(&self) -> f64 {
        self.m23
    }

    /// Returns the horizontal translation factor.
    pub fn m31(&self) -> f64 {
        self.m31
    }

    /// Returns the vertical translation factor.
    pub fn m32(&self) -> f64 {
        self.m32
    }

    /// Returns the division factor.
    pub fn m33(&self) -> f64 {
        self.m33
    }

    /// Returns the adjoint of this matrix.
    pub fn adjoint(&self) -> Self {
        unimplemented!()
    }

    /// Returns the matrix's determinant.
    pub fn determinant(&self) -> f64 {
        self.m11 * (self.m33 * self.m22 - self.m32 * self.m23)
            - self.m21 * (self.m33 * self.m12 - self.m32 * self.m13)
            + self.m31 * (self.m23 * self.m12 - self.m22 * self.m13)
    }

    /// Returns the horizontal translation factor.
    pub fn dx(&self) -> f64 {
        self.m31
    }

    /// Returns the vertical translation factor.
    pub fn dy(&self) -> f64 {
        self.m32
    }

    /// Returns an inverted copy of this matrix.
    ///
    /// If the matrix is singular (not invertible), the returned matrix is the identity matrix.
    /// Value of `invertible` is set to true if the matrix is invertible, otherwise it is set to false.
    pub fn inverted(&self, invertible: &mut bool) -> Self {
        unimplemented!()
    }

    /// Returns true if the matrix represent an affine transformation, otherwise returns false.
    pub fn is_affine(&self) -> bool {
        self.get_type() < TransformationType::Project
    }

    /// Returns true if the matrix is the identity matrix, otherwise returns false.
    pub fn is_identity(&self) -> bool {
        self.get_type() == TransformationType::None
    }

    /// Returns true if the matrix is invertible, otherwise returns false.
    pub fn is_invertible(&self) -> bool {
        self.determinant() != 0.0
    }

    /// Returns true if the matrix represents some kind of a rotating transformation,
    /// otherwise returns false.
    ///
    /// Note: A rotation transformation of 180 degrees and/or 360 degrees
    /// is treated as a scaling transformation.
    pub fn is_rotating(&self) -> bool {
        self.get_type() >= TransformationType::Rotate
    }

    /// Returns true if the matrix represents a scaling transformation, otherwise returns false.
    pub fn is_scaling(&self) -> bool {
        self.get_type() >= TransformationType::Scale
    }

    /// Returns true if the matrix represents a translating transformation, otherwise returns false.
    pub fn is_translating(&self) -> bool {
        self.get_type() >= TransformationType::Translate
    }

    /// Maps the given coordinates `x` and `y` into the coordinate system defined by this matrix.
    pub fn map(&self, x: f64, y: f64) -> (f64, f64) {
        unimplemented!()
    }

    /// Maps the given coordinates `x` and `y` into the coordinate system defined by this matrix.
    ///
    /// Note that the transformed coordinates are rounded to the nearest integer.
    pub fn map_int(&self, x: i32, y: i32) -> (i32, i32) {
        unimplemented!()
    }

    /// Creates and returns a Point object that is a copy of the given point,
    /// mapped into the coordinate system defined by this matrix.
    ///
    /// Note that the transformed coordinates are rounded to the nearest integer.
    pub fn map_point(&self, point: &Point) -> Point {
        unimplemented!()
    }

    /// Creates and returns a PointF object that is a copy of the given point,
    /// mapped into the coordinate system defined by this matrix.
    pub fn map_point_f(&self, point: &PointF) -> PointF {
        unimplemented!()
    }

    /// Creates and returns a Line object that is a copy of the given line,
    /// mapped into the coordinate system defined by this matrix.
    ///
    /// Note that the transformed coordinates are rounded to the nearest integer.
    pub fn map_line(&self, line: &Line) -> Line {
        unimplemented!()
    }

    /// Creates and returns a Line object that is a copy of the given line,
    /// mapped into the coordinate system defined by this matrix.
    pub fn map_line_f(&self, line: &LineF) -> LineF {
        unimplemented!()
    }

    // Creates and returns a Polygon object that is a copy of the given polygon,
    // mapped into the coordinate system defined by this matrix.
    //
    // Note that the transformed coordinates are rounded to the nearest integer.
    //pub fn map_polygon(&self, polygon: &Polygon) -> Polygon {
    //    unimplemented!()
    //}

    // Creates and returns a PolygonF object that is a copy of the given polygon,
    // mapped into the coordinate system defined by this matrix.
    //pub fn map_polygon_f(&self, polygon: &PolygonF) -> PolygonF {
    //    unimplemented!()
    //}

    // Creates and returns a Region object that is a copy of the given region,
    // mapped into the coordinate system defined by this matrix.
    //
    // Calling this method can be rather expensive if rotations or shearing are used.
    //pub fn map_region(&self, region: &Region) -> Region {
    //    unimplemented!()
    //}

    // Creates and returns a PainterPath object that is a copy of the given path,
    // mapped into the coordinate system defined by this matrix.
    //pub fn map_painter_path(&self, painter_path: &PainterPath) -> PainterPath {
    //    unimplemented!()
    //}

    /// Creates and returns a Rect object that is a copy of the given rectangle,
    /// mapped into the coordinate system defined by this matrix.
    ///
    /// Note that the transformed coordinates are rounded to the nearest integer.
    pub fn map_rect(&self, rect: &Rect) -> Rect {
        unimplemented!()
    }

    /// Creates and returns a RectF object that is a copy of the given rectangle,
    /// mapped into the coordinate system defined by this matrix.
    pub fn map_rect_f(&self, rect: &RectF) -> RectF {
        unimplemented!()
    }

    // Creates and returns a Polygon representation of the given rectangle,
    // mapped into the coordinate system defined by this matrix.
    //pub fn map_to_polygon(&self, rect: &Rect) -> Polygon {
    //    unimplemented!()
    //}

    // Creates a transformation matrix, that maps a four-sided polygon, `one`,
    // to another four-sided polygon, `two`.
    //
    // Returns Some if the transformation is possible; otherwise returns None.
    //
    // This is a convenience method combining `quad_to_square()` and `square_to_quad()` methods.
    // It allows the input quad to be transformed into any other quad.
    //pub fn quad_to_quad(one: &PolygonF, two: &PolygonF) -> Option<Self> {
    //    unimplemented!()
    //}

    // Creates a transformation matrix, that maps a four-sided polygon, `quad`,
    // to a unit square.
    //
    // Returns Some if the transformation is constructed or None if such a transformation does not exist.
    //pub fn quad_to_square(quad: &PolygonF) -> Option<Self> {
    //    unimplemented!()
    //}

    /// Resets the matrix to an identity matrix, i.e. all elements are set to zero,
    /// except m11 and m22 (specifying the scale) and m33 which are set to 1.
    pub fn reset(&mut self) {
        unimplemented!()
    }

    /// Rotates the coordinate system counterclockwise by the given `angle`
    /// about the z-axis.
    ///
    /// The `angle` is specified in degrees.
    pub fn rotate(&mut self, angle: f64) {
        self.rotate_with_axis(angle, Axis::Z);
    }

    /// Rotates the coordinate system counterclockwise by the given `angle`
    /// about the specified `axis`.
    ///
    /// The `angle` is specified in degrees.
    pub fn rotate_with_axis(&mut self, angle: f64, axis: Axis) {
        unimplemented!()
    }

    /// Rotates the coordinate system counterclockwise by the given `angle`
    /// about the z-axis.
    ///
    /// The `angle` is specified in radians.
    pub fn rotate_radians(&mut self, angle: f64) {
        self.rotate_radians_with_axis(angle, Axis::Z);
    }

    /// Rotates the coordinate system counterclockwise by the given `angle`
    /// about the specified `axis`.
    ///
    /// The `angle` is specified in radians.
    pub fn rotate_radians_with_axis(&mut self, angle: f64, axis: Axis) {
        unimplemented!()
    }

    /// Scales the coordinate system by `sx` horizontally and `sy` vertically.
    pub fn scale(&mut self, sx: f64, sy: f64) {
        unimplemented!()
    }

    /// Sets the matrix elements to the specified values, m11, m12, m13 m21, m22, m23 m31, m32 and m33.
    ///
    /// Note that this function replaces the previous values.
    ///
    /// Transform provides the `translate()`, `rotate()`, `scale()` and `shear()`
    /// convenience functions to manipulate the various matrix elements
    /// based on the currently defined coordinate system.
    pub fn set_matrix(
        &mut self,
        m11: f64,
        m12: f64,
        m13: f64,
        m21: f64,
        m22: f64,
        m23: f64,
        m31: f64,
        m32: f64,
        m33: f64,
    ) {
        unimplemented!()
    }

    /// Shears the coordinate system by sh horizontally and sv vertically.
    pub fn shear(&mut self, sh: f64, sv: f64) {
        unimplemented!()
    }

    // Creates a transformation matrix, trans, that maps a unit square to
    // a four-sided polygon.
    //
    // Returns Some if the transformation is constructed or None if such a transformation does not exist.
    //pub fn square_to_quad(quad: &PolygonF) -> Option<Self> {
    //    unimplemented!()
    //}

    /// Moves the coordinate system dx along the x axis and dy along the y axis.
    pub fn translate(&mut self, dx: f64, dy: f64) {
        unimplemented!()
    }

    /// Returns the transpose of this matrix.
    pub fn transposed(&self) -> Self {
        unimplemented!()
    }

    /// Returns the transformation type of this matrix.
    ///
    /// The transformation type is the highest enumeration value
    /// capturing all of the matrix's transformations.
    /// For example, if the matrix both scales and shears, the type would be Shear,
    /// because Shear has a higher enumeration value than Scale.
    ///
    /// Knowing the transformation type of a matrix is useful for optimization:
    /// you can often handle specific types more optimally than handling the generic case.
    pub fn get_type(&self) -> TransformationType {
        unimplemented!()
    }
}

impl ops::Mul<&Transform> for Transform {
    type Output = Transform;

    /// Returns the result of multiplying this matrix by the given matrix.
    ///
    /// Note that matrix multiplication is not commutative, i.e. a*b != b*a.
    fn mul(self, trans: &Transform) -> Self::Output {
        unimplemented!()
    }
}

impl ops::Mul<&Transform> for &Transform {
    type Output = Transform;

    /// Returns the result of multiplying this matrix by the given matrix.
    ///
    /// Note that matrix multiplication is not commutative, i.e. a*b != b*a.
    fn mul(self, trans: &Transform) -> Self::Output {
        unimplemented!()
    }
}

impl ops::MulAssign<&Transform> for Transform {
    /// Returns the result of multiplying this matrix by the given matrix.
    fn mul_assign(&mut self, trans: &Transform) {
        unimplemented!()
    }
}

impl ops::MulAssign<f64> for Transform {
    /// Returns the result of performing an element-wise multiplication of this matrix
    /// with the given scalar.
    fn mul_assign(&mut self, scalar: f64) {
        if scalar == 1.0 {
            return;
        }
        self.m11 *= scalar;
        self.m12 *= scalar;
        self.m13 *= scalar;
        self.m21 *= scalar;
        self.m22 *= scalar;
        self.m23 *= scalar;
        self.m31 *= scalar;
        self.m32 *= scalar;
        self.m33 *= scalar;
        if self.dirty < TransformationType::Scale {
            self.dirty = TransformationType::Scale;
        }
    }
}

impl ops::AddAssign<f64> for Transform {
    /// Returns the matrix obtained by adding the given scalar to each element of this matrix.
    fn add_assign(&mut self, scalar: f64) {
        unimplemented!()
    }
}

impl ops::SubAssign<f64> for Transform {
    /// Returns the matrix obtained by subtracting the given scalar from each element of this matrix.
    fn sub_assign(&mut self, scalar: f64) {
        unimplemented!()
    }
}

impl ops::DivAssign<f64> for Transform {
    /// Returns the result of performing an element-wise division of this matrix by the given scalar.
    fn div_assign(&mut self, scalar: f64) {
        unimplemented!()
    }
}
