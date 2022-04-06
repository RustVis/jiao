// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

use core::cmp;
use core::f64::consts::PI;
use core::ops;

use crate::base::{Axis, Line, LineF, Point, PointF, Rect, RectF};

// pi/180
const DEG_TO_RAD: f64 = PI / 180.0;
const INV_DIST_TO_PLANE: f64 = 1.0 / 1024.0;

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
#[derive(Debug, Clone)]
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
    type_: TransformationType,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
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
        Self {
            m11: 1.0,
            m12: 0.0,
            m13: 0.0,
            m21: 0.0,
            m22: 1.0,
            m23: 0.0,
            m31: 0.0,
            m32: 0.0,
            m33: 1.0,

            dirty: TransformationType::None,
            type_: TransformationType::None,
        }
    }

    /// Constructs a matrix with the elements, `m11`, `m12`, `m21`, `m22`, `dx` and `dy`.
    pub fn new_2d(m11: f64, m12: f64, m21: f64, m22: f64, dx: f64, dy: f64) -> Self {
        Self {
            m11,
            m12,
            m13: 0.0,
            m21,
            m22,
            m23: 0.0,
            m31: dx,
            m32: dy,
            m33: 1.0,

            dirty: TransformationType::Shear,
            type_: TransformationType::None,
        }
    }

    /// Constructs a matrix with the elements, `m11`, `m12`, `m13`, `m21`, `m22`, `m23`, `m31`, `m32`, `m33`.
    pub fn new_3d(
        m11: f64,
        m12: f64,
        m13: f64,
        m21: f64,
        m22: f64,
        m23: f64,
        m31: f64,
        m32: f64,
        m33: f64,
    ) -> Self {
        Self {
            m11,
            m12,
            m13,
            m21,
            m22,
            m23,
            m31,
            m32,
            m33,

            dirty: TransformationType::Project,
            type_: TransformationType::None,
        }
    }

    /// Creates a matrix which corresponds to a scaling of `sx` horizontally and
    /// `sy` vertically.
    ///
    /// This is the same as `Transform::new().scale(sx, sy)` but slightly faster.
    pub fn from_scale(sx: f64, sy: f64) -> Self {
        let mut transform = Self::new_3d(sx, 0.0, 0.0, 0.0, sy, 0.0, 0.0, 0.0, 1.0);
        if sx == 1.0 && sy == 1.0 {
            transform.type_ = TransformationType::None;
        } else {
            transform.type_ = TransformationType::Scale;
        }
        transform.dirty = TransformationType::None;
        return transform;
    }

    /// Creates a matrix which corresponds to a translation of `dx` along the x axis and
    /// `dy` along the y axis.
    ///
    /// This is the same as `Transform::new().translate(dx, dy)` but slightly faster.
    pub fn from_translate(dx: f64, dy: f64) -> Self {
        let mut transform = Self::new_3d(1.0, 0.0, 0.0, 0.0, 1.0, 0.0, dx, dy, 1.0);
        if dx == 0.0 && dy == 0.0 {
            transform.type_ = TransformationType::None;
        } else {
            transform.type_ = TransformationType::Translate;
        }
        transform.dirty = TransformationType::None;
        return transform;
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
        let h11 = self.m22 * self.m33 - self.m23 * self.m32;
        let h21 = self.m23 * self.m31 - self.m21 * self.m33;
        let h31 = self.m21 * self.m32 - self.m22 * self.m31;
        let h12 = self.m13 * self.m32 - self.m12 * self.m33;
        let h22 = self.m11 * self.m33 - self.m13 * self.m31;
        let h32 = self.m12 * self.m31 - self.m11 * self.m32;
        let h13 = self.m12 * self.m23 - self.m13 * self.m22;
        let h23 = self.m13 * self.m21 - self.m11 * self.m23;
        let h33 = self.m11 * self.m22 - self.m12 * self.m21;

        Self::new_3d(h11, h12, h13, h21, h22, h23, h31, h32, h33)
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
        let mut invert = Self::new();
        let mut inv = true;

        match self.get_type() {
            TransformationType::None => {
                // ignore.
            }
            TransformationType::Translate => {
                invert.m31 = -self.m31;
                invert.m32 = -self.m32;
            }
            TransformationType::Scale => {
                inv = self.m11 != 0.0;
                inv &= self.m22 != 0.0;
                if inv {
                    invert.m11 = 1. / self.m11;
                    invert.m22 = 1. / self.m22;
                    invert.m31 = -self.m31 * invert.m11;
                    invert.m32 = -self.m32 * invert.m22;
                }
            }
            TransformationType::Rotate | TransformationType::Shear => {
                // TODO(Shaohua): implements inverted()
                //invert.affine = affine.inverted(&inv);
            }
            _ => {
                // general case
                let det = self.determinant();
                inv = det != 0.0;
                if inv {
                    invert = self.adjoint() / det;
                }
            }
        }

        *invertible = inv;

        if inv {
            // inverting doesn't change the type
            invert.type_ = self.type_;
            invert.dirty = self.dirty;
        }

        return invert;
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
        self.map_helper(x, y)
    }

    /// Maps the given coordinates `x` and `y` into the coordinate system defined by this matrix.
    ///
    /// Note that the transformed coordinates are rounded to the nearest integer.
    pub fn map_int(&self, x: i32, y: i32) -> (i32, i32) {
        let (fx, fy) = self.map_helper(x as f64, y as f64);
        (fx.round() as i32, fy.round() as i32)
    }

    /// Creates and returns a Point object that is a copy of the given point,
    /// mapped into the coordinate system defined by this matrix.
    ///
    /// Note that the transformed coordinates are rounded to the nearest integer.
    pub fn map_point(&self, point: &Point) -> Point {
        let fx = point.x() as f64;
        let fy = point.y() as f64;
        let mut x;
        let mut y;
        let t = self.get_type();
        match t {
            TransformationType::None => {
                x = fx;
                y = fy;
            }
            TransformationType::Translate => {
                x = fx + self.m31;
                y = fy + self.m32;
            }
            TransformationType::Scale => {
                x = self.m11 * fx + self.m31;
                y = self.m22 * fy + self.m32;
            }
            TransformationType::Rotate
            | TransformationType::Shear
            | TransformationType::Project => {
                x = self.m11 * fx + self.m21 * fy + self.m31;
                y = self.m12 * fx + self.m22 * fy + self.m32;
                if t == TransformationType::Project {
                    let w = 1.0 / (self.m13 * fx + self.m23 * fy + self.m33);
                    x *= w;
                    y *= w;
                }
            }
        }

        Point::from(x.round() as i32, y.round() as i32)
    }

    /// Creates and returns a PointF object that is a copy of the given point,
    /// mapped into the coordinate system defined by this matrix.
    pub fn map_point_f(&self, point: &PointF) -> PointF {
        let fx = point.x();
        let fy = point.y();
        let mut x;
        let mut y;

        let t = self.get_type();
        match t {
            TransformationType::None => {
                x = fx;
                y = fy;
            }
            TransformationType::Translate => {
                x = fx + self.m31;
                y = fy + self.m32;
            }
            TransformationType::Scale => {
                x = self.m11 * fx + self.m31;
                y = self.m22 * fy + self.m32;
            }
            TransformationType::Rotate
            | TransformationType::Shear
            | TransformationType::Project => {
                x = self.m11 * fx + self.m21 * fy + self.m31;
                y = self.m12 * fx + self.m22 * fy + self.m32;
                if t == TransformationType::Project {
                    let w = 1. / (self.m13 * fx + self.m23 * fy + self.m33);
                    x *= w;
                    y *= w;
                }
            }
        }
        PointF::from(x, y)
    }

    /// Creates and returns a Line object that is a copy of the given line,
    /// mapped into the coordinate system defined by this matrix.
    ///
    /// Note that the transformed coordinates are rounded to the nearest integer.
    pub fn map_line(&self, line: &Line) -> Line {
        let fx1 = line.x1() as f64;
        let fy1 = line.y1() as f64;
        let fx2 = line.x2() as f64;
        let fy2 = line.y2() as f64;

        let mut x1;
        let mut y1;
        let mut x2;
        let mut y2;

        let t = self.get_type();
        match t {
            TransformationType::None => {
                x1 = fx1;
                y1 = fy1;
                x2 = fx2;
                y2 = fy2;
            }
            TransformationType::Translate => {
                x1 = fx1 + self.m31;
                y1 = fy1 + self.m32;
                x2 = fx2 + self.m31;
                y2 = fy2 + self.m32;
            }
            TransformationType::Scale => {
                x1 = self.m11 * fx1 + self.m31;
                y1 = self.m22 * fy1 + self.m32;
                x2 = self.m11 * fx2 + self.m31;
                y2 = self.m22 * fy2 + self.m32;
            }
            TransformationType::Rotate
            | TransformationType::Shear
            | TransformationType::Project => {
                x1 = self.m11 * fx1 + self.m21 * fy1 + self.m31;
                y1 = self.m12 * fx1 + self.m22 * fy1 + self.m32;
                x2 = self.m11 * fx2 + self.m21 * fy2 + self.m31;
                y2 = self.m12 * fx2 + self.m22 * fy2 + self.m32;
                if t == TransformationType::Project {
                    let mut w = 1.0 / (self.m13 * fx1 + self.m23 * fy1 + self.m33);
                    x1 *= w;
                    y1 *= w;
                    w = 1.0 / (self.m13 * fx2 + self.m23 * fy2 + self.m33);
                    x2 *= w;
                    y2 *= w;
                }
            }
        }

        Line::from(
            x1.round() as i32,
            y1.round() as i32,
            x2.round() as i32,
            y2.round() as i32,
        )
    }

    /// Creates and returns a Line object that is a copy of the given line,
    /// mapped into the coordinate system defined by this matrix.
    pub fn map_line_f(&self, line: &LineF) -> LineF {
        let fx1 = line.x1();
        let fy1 = line.y1();
        let fx2 = line.x2();
        let fy2 = line.y2();

        let mut x1;
        let mut y1;
        let mut x2;
        let mut y2;

        let t = self.get_type();
        match t {
            TransformationType::None => {
                x1 = fx1;
                y1 = fy1;
                x2 = fx2;
                y2 = fy2;
            }
            TransformationType::Translate => {
                x1 = fx1 + self.m31;
                y1 = fy1 + self.m32;
                x2 = fx2 + self.m31;
                y2 = fy2 + self.m32;
            }
            TransformationType::Scale => {
                x1 = self.m11 * fx1 + self.m31;
                y1 = self.m22 * fy1 + self.m32;
                x2 = self.m11 * fx2 + self.m31;
                y2 = self.m22 * fy2 + self.m32;
            }
            TransformationType::Rotate
            | TransformationType::Shear
            | TransformationType::Project => {
                x1 = self.m11 * fx1 + self.m21 * fy1 + self.m31;
                y1 = self.m12 * fx1 + self.m22 * fy1 + self.m32;
                x2 = self.m11 * fx2 + self.m21 * fy2 + self.m31;
                y2 = self.m12 * fx2 + self.m22 * fy2 + self.m32;
                if t == TransformationType::Project {
                    let mut w = 1.0 / (self.m13 * fx1 + self.m23 * fy1 + self.m33);
                    x1 *= w;
                    y1 *= w;
                    w = 1.0 / (self.m13 * fx2 + self.m23 * fy2 + self.m33);
                    x2 *= w;
                    y2 *= w;
                }
            }
        }
        LineF::from(x1, y1, x2, y2)
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
    pub fn map_rect(&self, _rect: &Rect) -> Rect {
        unimplemented!()
    }

    /// Creates and returns a RectF object that is a copy of the given rectangle,
    /// mapped into the coordinate system defined by this matrix.
    pub fn map_rect_f(&self, _rect: &RectF) -> RectF {
        unimplemented!()
    }

    fn map_helper(&self, x: f64, y: f64) -> (f64, f64) {
        let t = self.get_type();
        let mut nx;
        let mut ny;
        match t {
            TransformationType::None => {
                nx = x;
                ny = y;
            }
            TransformationType::Translate => {
                nx = x + self.m31;
                ny = y + self.m32;
            }
            TransformationType::Scale => {
                nx = self.m11 * x + self.m31;
                ny = self.m22 * y + self.m32;
            }
            TransformationType::Rotate
            | TransformationType::Shear
            | TransformationType::Project => {
                nx = self.m11 * x + self.m21 * y + self.m31;
                ny = self.m12 * x + self.m22 * y + self.m32;
                if t == TransformationType::Project {
                    let mut w = self.m13 * x + self.m23 * y + self.m33;
                    const NEAR_CLIP: f64 = 0.000001;
                    if w < NEAR_CLIP {
                        w = NEAR_CLIP;
                    }
                    w = 1.0 / w;
                    nx *= w;
                    ny *= w;
                }
            }
        }

        (nx, ny)
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
        self.m11 = 1.0;
        self.m12 = 0.0;
        self.m13 = 0.0;
        self.m21 = 0.0;
        self.m22 = 1.0;
        self.m23 = 0.0;
        self.m31 = 0.0;
        self.m32 = 0.0;
        self.m33 = 1.0;
        self.type_ = TransformationType::None;
        self.dirty = TransformationType::None;
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
        if angle == 0.0 {
            return;
        }

        let mut sina = 0.0;
        let mut cosa = 0.0;
        if angle == 90.0 || angle == -270.0 {
            sina = 1.0;
        } else if angle == 270.0 || angle == -90.0 {
            sina = -1.0;
        } else if angle == 180.0 {
            cosa = -1.0;
        } else {
            // convert to radians
            let b = DEG_TO_RAD * angle;
            // fast and convenient
            sina = b.sin();
            cosa = b.cos();
        }

        if axis == Axis::Z {
            match self.get_type() {
                TransformationType::None | TransformationType::Translate => {
                    self.m11 = cosa;
                    self.m12 = sina;
                    self.m21 = -sina;
                    self.m22 = cosa;
                }
                TransformationType::Scale => {
                    let tm11 = cosa * self.m11;
                    let tm12 = sina * self.m22;
                    let tm21 = -sina * self.m11;
                    let tm22 = cosa * self.m22;
                    self.m11 = tm11;
                    self.m12 = tm12;
                    self.m21 = tm21;
                    self.m22 = tm22;
                }
                TransformationType::Project => {
                    let tm13 = cosa * self.m13 + sina * self.m23;
                    let tm23 = -sina * self.m13 + cosa * self.m23;
                    self.m13 = tm13;
                    self.m23 = tm23;
                    let tm11 = cosa * self.m11 + sina * self.m21;
                    let tm12 = cosa * self.m12 + sina * self.m22;
                    let tm21 = -sina * self.m11 + cosa * self.m21;
                    let tm22 = -sina * self.m12 + cosa * self.m22;
                    self.m11 = tm11;
                    self.m12 = tm12;
                    self.m21 = tm21;
                    self.m22 = tm22;
                }

                TransformationType::Rotate | TransformationType::Shear => {
                    let tm11 = cosa * self.m11 + sina * self.m21;
                    let tm12 = cosa * self.m12 + sina * self.m22;
                    let tm21 = -sina * self.m11 + cosa * self.m21;
                    let tm22 = -sina * self.m12 + cosa * self.m22;
                    self.m11 = tm11;
                    self.m12 = tm12;
                    self.m21 = tm21;
                    self.m22 = tm22;
                }
            }

            if self.dirty < TransformationType::Rotate {
                self.dirty = TransformationType::Rotate;
            }
        } else {
            let mut result = Self::new();
            if axis == Axis::Y {
                result.m11 = cosa;
                result.m13 = -sina * INV_DIST_TO_PLANE;
            } else {
                result.m22 = cosa;
                result.m23 = -sina * INV_DIST_TO_PLANE;
            }
            result.type_ = TransformationType::Project;
            *self = result * self;
        }
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
        let sina = angle.sin();
        let cosa = angle.cos();

        if axis == Axis::Z {
            match self.get_type() {
                TransformationType::None | TransformationType::Translate => {
                    self.m11 = cosa;
                    self.m12 = sina;
                    self.m21 = -sina;
                    self.m22 = cosa;
                }
                TransformationType::Scale => {
                    let tm11 = cosa * self.m11;
                    let tm12 = sina * self.m22;
                    let tm21 = -sina * self.m11;
                    let tm22 = cosa * self.m22;
                    self.m11 = tm11;
                    self.m12 = tm12;
                    self.m21 = tm21;
                    self.m22 = tm22;
                }
                TransformationType::Project => {
                    let tm13 = cosa * self.m13 + sina * self.m23;
                    let tm23 = -sina * self.m13 + cosa * self.m23;
                    self.m13 = tm13;
                    self.m23 = tm23;
                    let tm11 = cosa * self.m11 + sina * self.m21;
                    let tm12 = cosa * self.m12 + sina * self.m22;
                    let tm21 = -sina * self.m11 + cosa * self.m21;
                    let tm22 = -sina * self.m12 + cosa * self.m22;
                    self.m11 = tm11;
                    self.m12 = tm12;
                    self.m21 = tm21;
                    self.m22 = tm22;
                }
                TransformationType::Rotate | TransformationType::Shear => {
                    let tm11 = cosa * self.m11 + sina * self.m21;
                    let tm12 = cosa * self.m12 + sina * self.m22;
                    let tm21 = -sina * self.m11 + cosa * self.m21;
                    let tm22 = -sina * self.m12 + cosa * self.m22;
                    self.m11 = tm11;
                    self.m12 = tm12;
                    self.m21 = tm21;
                    self.m22 = tm22;
                }
            }
            if self.dirty < TransformationType::Rotate {
                self.dirty = TransformationType::Rotate;
            }
        } else {
            let mut result = Self::new();
            if axis == Axis::Y {
                result.m11 = cosa;
                result.m13 = -sina * INV_DIST_TO_PLANE;
            } else {
                result.m22 = cosa;
                result.m23 = -sina * INV_DIST_TO_PLANE;
            }
            result.type_ = TransformationType::Project;
            *self = result * self;
        }
    }

    /// Scales the coordinate system by `sx` horizontally and `sy` vertically.
    pub fn scale(&mut self, sx: f64, sy: f64) {
        if sx == 1.0 && sy == 1.0 {
            return;
        }

        match self.get_type() {
            TransformationType::None | TransformationType::Translate => {
                self.m11 = sx;
                self.m22 = sy;
            }

            TransformationType::Project => {
                self.m13 *= sx;
                self.m23 *= sy;
                self.m12 *= sx;
                self.m21 *= sy;
                self.m11 *= sx;
                self.m22 *= sy;
            }
            TransformationType::Rotate | TransformationType::Shear => {
                self.m12 *= sx;
                self.m21 *= sy;
                self.m11 *= sx;
                self.m22 *= sy;
            }

            TransformationType::Scale => {
                self.m11 *= sx;
                self.m22 *= sy;
            }
        }

        if self.dirty < TransformationType::Scale {
            self.dirty = TransformationType::Scale;
        }
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
        self.m11 = m11;
        self.m12 = m12;
        self.m13 = m13;
        self.m21 = m21;
        self.m22 = m22;
        self.m23 = m23;
        self.m31 = m31;
        self.m32 = m32;
        self.m33 = m33;
        self.type_ = TransformationType::None;
        self.dirty = TransformationType::Project;
    }

    /// Shears the coordinate system by sh horizontally and sv vertically.
    pub fn shear(&mut self, sh: f64, sv: f64) {
        if sh == 0.0 && sv == 0.0 {
            return;
        }

        match self.get_type() {
            TransformationType::None | TransformationType::Translate => {
                self.m12 = sv;
                self.m21 = sh;
            }
            TransformationType::Scale => {
                self.m12 = sv * self.m22;
                self.m21 = sh * self.m11;
            }
            TransformationType::Project => {
                let tm13 = sv * self.m23;
                let tm23 = sh * self.m13;
                self.m13 += tm13;
                self.m23 += tm23;

                let tm11 = sv * self.m21;
                let tm22 = sh * self.m12;
                let tm12 = sv * self.m22;
                let tm21 = sh * self.m11;
                self.m11 += tm11;
                self.m12 += tm12;
                self.m21 += tm21;
                self.m22 += tm22;
            }
            TransformationType::Rotate | TransformationType::Shear => {
                let tm11 = sv * self.m21;
                let tm22 = sh * self.m12;
                let tm12 = sv * self.m22;
                let tm21 = sh * self.m11;
                self.m11 += tm11;
                self.m12 += tm12;
                self.m21 += tm21;
                self.m22 += tm22;
            }
        }
        if self.dirty < TransformationType::Shear {
            self.dirty = TransformationType::Shear;
        }
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
        if dx == 0.0 && dy == 0.0 {
            return;
        }

        match self.get_type() {
            TransformationType::None => {
                self.m31 -= dx;
                self.m32 -= dy;
            }
            TransformationType::Translate => {
                self.m31 += dx;
                self.m32 += dy;
            }
            TransformationType::Scale => {
                self.m31 += dx * self.m11;
                self.m32 += dy * self.m22;
            }
            TransformationType::Project => {
                self.m33 += dx * self.m13 + dy * self.m23;
                self.m31 += dx * self.m11 + dy * self.m21;
                self.m32 += dy * self.m22 + dx * self.m12;
            }
            TransformationType::Shear | TransformationType::Rotate => {
                self.m31 += dx * self.m11 + dy * self.m21;
                self.m32 += dy * self.m22 + dx * self.m12;
            }
        }

        if self.dirty < TransformationType::Translate {
            self.dirty = TransformationType::Translate;
        }
    }

    /// Returns the transpose of this matrix.
    pub fn transposed(&self) -> Self {
        Self::new_3d(
            self.m11, self.m21, self.m31, self.m12, self.m22, self.m32, self.m13, self.m23,
            self.m33,
        )
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
        if self.dirty == TransformationType::None || self.dirty < self.type_ {
            return self.type_;
        }

        if self.dirty == TransformationType::Project {
            if self.m13 != 0.0 || self.m23 != 0.0 || (self.m33 - 1.0) != 0.0 {
                return TransformationType::Project;
            }
        }

        if self.dirty == TransformationType::Shear || self.dirty == TransformationType::Rotate {
            if self.m12 != 0.0 || self.m21 != 0.0 {
                let dot = self.m11 * self.m12 * self.m21 * self.m22;
                if dot == 0.0 {
                    return TransformationType::Rotate;
                } else {
                    return TransformationType::Shear;
                }
            }
        }

        if self.dirty == TransformationType::Scale {
            if (self.m11 - 1.0) != 0.0 || (self.m22 - 1.0) != 0.0 {
                return TransformationType::Scale;
            }
        }

        if self.dirty == TransformationType::Translate {
            if self.m31 != 0.0 || self.m32 != 0.0 {
                return TransformationType::Translate;
            }
        }

        assert_eq!(self.dirty, TransformationType::None);
        return TransformationType::None;
    }
}

impl ops::Mul<&Transform> for Transform {
    type Output = Transform;

    /// Returns the result of multiplying this matrix by the given matrix.
    ///
    /// Note that matrix multiplication is not commutative, i.e. a*b != b*a.
    fn mul(self, other: &Transform) -> Self::Output {
        let other_type = other.get_type();
        if other_type == TransformationType::None {
            return self;
        }

        let this_type = self.get_type();
        if this_type == TransformationType::None {
            return other.clone();
        }

        let mut t = Self::new();
        let new_type = this_type.max(other_type);
        match new_type {
            TransformationType::None => {
                // ignore
            }
            TransformationType::Translate => {
                t.m31 = self.m31 + other.m31;
                t.m32 += self.m32 + other.m32;
            }
            TransformationType::Scale => {
                let m11 = self.m11 * other.m11;
                let m22 = self.m22 * other.m22;

                let m31 = self.m31 * other.m11 + other.m31;
                let m32 = self.m32 * other.m22 + other.m32;

                t.m11 = m11;
                t.m22 = m22;
                t.m31 = m31;
                t.m32 = m32;
            }
            TransformationType::Rotate | TransformationType::Shear => {
                let m11 = self.m11 * other.m11 + self.m12 * other.m21;
                let m12 = self.m11 * other.m12 + self.m12 * other.m22;

                let m21 = self.m21 * other.m11 + self.m22 * other.m21;
                let m22 = self.m21 * other.m12 + self.m22 * other.m22;

                let m31 = self.m31 * other.m11 + self.m32 * other.m21 + other.m31;
                let m32 = self.m31 * other.m12 + self.m32 * other.m22 + other.m32;

                t.m11 = m11;
                t.m12 = m12;
                t.m21 = m21;
                t.m22 = m22;
                t.m31 = m31;
                t.m32 = m32;
            }
            TransformationType::Project => {
                let m11 = self.m11 * other.m11 + self.m12 * other.m21 + self.m13 * other.m31;
                let m12 = self.m11 * other.m12 + self.m12 * other.m22 + self.m13 * other.m32;
                let m13 = self.m11 * other.m13 + self.m12 * other.m23 + self.m13 * other.m33;

                let m21 = self.m21 * other.m11 + self.m22 * other.m21 + self.m23 * other.m31;
                let m22 = self.m21 * other.m12 + self.m22 * other.m22 + self.m23 * other.m32;
                let m23 = self.m21 * other.m13 + self.m22 * other.m23 + self.m23 * other.m33;

                let m31 = self.m31 * other.m11 + self.m32 * other.m21 + self.m33 * other.m31;
                let m32 = self.m31 * other.m12 + self.m32 * other.m22 + self.m33 * other.m32;
                let m33 = self.m31 * other.m13 + self.m32 * other.m23 + self.m33 * other.m33;

                t.m11 = m11;
                t.m12 = m12;
                t.m13 = m13;
                t.m21 = m21;
                t.m22 = m22;
                t.m23 = m23;
                t.m31 = m31;
                t.m32 = m32;
                t.m33 = m33;
            }
        }

        t.dirty = new_type;
        t.type_ = new_type;

        return t;
    }
}

impl ops::Div<f64> for Transform {
    type Output = Transform;

    fn div(mut self, scalar: f64) -> Self::Output {
        if scalar == 0.0 {
            return self;
        }
        let scalar = 1.0 / scalar;
        self *= scalar;
        self
    }
}

impl ops::MulAssign<&Transform> for Transform {
    /// Returns the result of multiplying this matrix by the given matrix.
    fn mul_assign(&mut self, other: &Transform) {
        let other_type = other.get_type();
        if other_type == TransformationType::None {
            return;
        }

        let this_type = self.get_type();
        if this_type == TransformationType::None {
            *self = other.clone();
            return;
        }

        let new_type = this_type.max(other_type);
        match new_type {
            TransformationType::None => {
                // ignore
            }
            TransformationType::Translate => {
                self.m31 += other.m31;
                self.m32 += other.m32;
            }
            TransformationType::Scale => {
                let m11 = self.m11 * other.m11;
                let m22 = self.m22 * other.m22;

                let m31 = self.m31 * other.m11 + other.m31;
                let m32 = self.m32 * other.m22 + other.m32;

                self.m11 = m11;
                self.m22 = m22;
                self.m31 = m31;
                self.m32 = m32;
            }
            TransformationType::Rotate | TransformationType::Shear => {
                let m11 = self.m11 * other.m11 + self.m12 * other.m21;
                let m12 = self.m11 * other.m12 + self.m12 * other.m22;

                let m21 = self.m21 * other.m11 + self.m22 * other.m21;
                let m22 = self.m21 * other.m12 + self.m22 * other.m22;

                let m31 = self.m31 * other.m11 + self.m32 * other.m21 + other.m31;
                let m32 = self.m31 * other.m12 + self.m32 * other.m22 + other.m32;

                self.m11 = m11;
                self.m12 = m12;
                self.m21 = m21;
                self.m22 = m22;
                self.m31 = m31;
                self.m32 = m32;
            }
            TransformationType::Project => {
                let m11 = self.m11 * other.m11 + self.m12 * other.m21 + self.m13 * other.m31;
                let m12 = self.m11 * other.m12 + self.m12 * other.m22 + self.m13 * other.m32;
                let m13 = self.m11 * other.m13 + self.m12 * other.m23 + self.m13 * other.m33;

                let m21 = self.m21 * other.m11 + self.m22 * other.m21 + self.m23 * other.m31;
                let m22 = self.m21 * other.m12 + self.m22 * other.m22 + self.m23 * other.m32;
                let m23 = self.m21 * other.m13 + self.m22 * other.m23 + self.m23 * other.m33;

                let m31 = self.m31 * other.m11 + self.m32 * other.m21 + self.m33 * other.m31;
                let m32 = self.m31 * other.m12 + self.m32 * other.m22 + self.m33 * other.m32;
                let m33 = self.m31 * other.m13 + self.m32 * other.m23 + self.m33 * other.m33;

                self.m11 = m11;
                self.m12 = m12;
                self.m13 = m13;
                self.m21 = m21;
                self.m22 = m22;
                self.m23 = m23;
                self.m31 = m31;
                self.m32 = m32;
                self.m33 = m33;
            }
        }

        self.dirty = new_type;
        self.type_ = new_type;
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
        if scalar == 0.0 {
            return;
        }
        self.m11 += scalar;
        self.m12 += scalar;
        self.m13 += scalar;
        self.m21 += scalar;
        self.m22 += scalar;
        self.m23 += scalar;
        self.m31 += scalar;
        self.m32 += scalar;
        self.m33 += scalar;
        self.dirty = TransformationType::Project;
    }
}

impl ops::SubAssign<f64> for Transform {
    /// Returns the matrix obtained by subtracting the given scalar from each element of this matrix.
    fn sub_assign(&mut self, scalar: f64) {
        if scalar == 0.0 {
            return;
        }
        self.m11 -= scalar;
        self.m12 -= scalar;
        self.m13 -= scalar;
        self.m21 -= scalar;
        self.m22 -= scalar;
        self.m23 -= scalar;
        self.m31 -= scalar;
        self.m32 -= scalar;
        self.m33 -= scalar;
        self.dirty = TransformationType::Project;
    }
}

impl ops::DivAssign<f64> for Transform {
    /// Returns the result of performing an element-wise division of this matrix by the given scalar.
    fn div_assign(&mut self, scalar: f64) {
        if scalar == 0.0 {
            return;
        }
        let scalar = 1.0 / scalar;
        *self *= scalar;
    }
}

impl cmp::PartialEq for Transform {
    /// Returns true if this matrix is equal to the given matrix, otherwise returns false.
    fn eq(&self, other: &Self) -> bool {
        self.m11 == other.m11
            && self.m12 == other.m12
            && self.m21 == other.m21
            && self.m22 == other.m22
            && self.m31 == other.m31
            && self.m32 == other.m32
            && self.m13 == other.m13
            && self.m23 == other.m23
            && self.m33 == other.m33
    }
}
