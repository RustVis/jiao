// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

use core::f64::consts::PI;
use core::ops;

use crate::base::{LineF, PointF, RectF};
use crate::util::{fuzzy_compare, fuzzy_is_zero};

/// The `Matrix` struct specifies 2D transformations of a coordinate system.
///
/// A matrix specifies how to translate, scale, shear or rotate the
/// coordinate system, and is typically used when rendering graphics.
/// `Matrix`, in contrast to `Transform`, does not allow perspective
/// transformations.
#[derive(Debug, Clone, PartialEq)]
pub struct Matrix {
    m11: f64,
    m12: f64,

    m21: f64,
    m22: f64,

    dx: f64,
    dy: f64,
}

impl Default for Matrix {
    fn default() -> Self {
        Self::identity()
    }
}

impl Matrix {
    /// Create an identity matrix.
    #[must_use]
    pub const fn new() -> Self {
        Self::identity()
    }

    /// Constructs a matrix with the elements, `m11`, `m12`, `m21`, `m22`, `dx` and `dy`.
    #[must_use]
    pub const fn from(m11: f64, m12: f64, m21: f64, m22: f64, dx: f64, dy: f64) -> Self {
        Self {
            m11,
            m12,
            m21,
            m22,
            dx,
            dy,
        }
    }

    /// Create an identity matrix.
    #[must_use]
    pub const fn identity() -> Self {
        Self {
            m11: 1.0,
            m12: 0.0,
            m21: 0.0,
            m22: 1.0,
            dx: 0.0,
            dy: 0.0,
        }
    }

    /// Returns the horizontal scaling factor.
    #[must_use]
    #[inline]
    pub const fn m11(&self) -> f64 {
        self.m11
    }

    /// Returns the vertical shearing factor.
    #[must_use]
    #[inline]
    pub const fn m12(&self) -> f64 {
        self.m12
    }

    /// Returns the horizontal shearing factor.
    #[must_use]
    #[inline]
    pub const fn m21(&self) -> f64 {
        self.m21
    }

    /// Returns the vertical scaling factor.
    #[must_use]
    #[inline]
    pub const fn m22(&self) -> f64 {
        self.m22
    }

    /// Returns the matrix's determinant.
    #[must_use]
    pub fn determinant(&self) -> f64 {
        self.m11 * self.m22 - self.m12 * self.m21
    }

    /// Returns the horizontal translation factor.
    #[must_use]
    #[inline]
    pub const fn dx(&self) -> f64 {
        self.dx
    }

    /// Returns the vertical translation factor.
    #[must_use]
    #[inline]
    pub const fn dy(&self) -> f64 {
        self.dy
    }

    /// Returns an inverted copy of this matrix.
    ///
    /// If the matrix is singular (not invertible), the returned matrix is the identity matrix.
    ///
    /// Second value of pair is true if the matrix is invertible, otherwise it is set to false.
    #[must_use]
    pub fn inverted(&self) -> (Self, bool) {
        let dtr = self.determinant();
        if dtr == 0.0 {
            // singular matrix
            (Self::new(), false)
        } else {
            // invertible matrix
            let dinv = 1.0 / dtr;
            let matrix = Self::from(
                self.m22 * dinv,
                -self.m12 * dinv,
                -self.m21 * dinv,
                self.m11 * dinv,
                (self.m21 * self.dy - self.m22 * self.dx) * dinv,
                (self.m12 * self.dx - self.m11 * self.dy) * dinv,
            );
            (matrix, true)
        }
    }

    /// Returns true if the matrix is the identity matrix, otherwise returns false.
    #[must_use]
    pub fn is_identity(&self) -> bool {
        fuzzy_is_zero(self.m11 - 1.0)
            && fuzzy_is_zero(self.m22 - 1.0)
            && fuzzy_is_zero(self.m12)
            && fuzzy_is_zero(self.m21)
            && fuzzy_is_zero(self.dx)
            && fuzzy_is_zero(self.dy)
    }

    /// Returns true if the matrix is invertible, otherwise returns false.
    #[must_use]
    pub fn is_invertible(&self) -> bool {
        !fuzzy_is_zero(self.m11 * self.m22 - self.m12 * self.m21)
    }

    /// Maps the given coordinates x and y into the coordinate system defined by this matrix.
    ///
    /// The resulting values are put in `(tx, ty)` pair.
    ///
    /// The coordinates are transformed using the following formulas:
    /// ```text
    /// x2 = m11 * x + m21 * y + dx
    /// y2 = m22 * y + m12 * x + dy
    /// ```
    /// The point `(x, y)` is the original point, and `(x', y')` is the transformed point.
    #[must_use]
    pub fn map(&self, x: f64, y: f64) -> (f64, f64) {
        let nx = self.m11.mul_add(x, self.m21 * y) + self.dx;
        let ny = self.m12.mul_add(x, self.m22 * y) + self.dy;
        (nx, ny)
    }

    /// Overload of `map()`.
    #[must_use]
    pub fn map_point(&self, point: PointF) -> PointF {
        let (nx, ny) = self.map(point.x(), point.y());
        PointF::from(nx, ny)
    }

    /// This is an overloaded function.
    ///
    /// Creates and returns a `LineF` object that is a copy of the given line,
    /// mapped into the coordinate system defined by this matrix.
    #[must_use]
    pub fn map_line(&self, line: &LineF) -> LineF {
        let p1 = self.map_point(line.p1());
        let p2 = self.map_point(line.p2());
        LineF::from_points(p1, p2)
    }

    /// Creates and returns a `RectF` object that is a copy of the given rectangle,
    /// mapped into the coordinate system defined by this matrix.
    ///
    /// The rectangle's coordinates are transformed using the following formulas:
    /// ```text
    /// x2 = m11 * x + m21 * y + dx
    /// y2 = m22 * y + m12 * x + dy
    /// ```
    /// If rotation or shearing has been specified, this function returns the bounding rectangle.
    #[must_use]
    #[allow(clippy::similar_names)]
    pub fn map_rect(&self, rect: &RectF) -> RectF {
        if self.m12 == 0.0 && self.m21 == 0.0 {
            let mut x = self.m11.mul_add(rect.x(), self.dx);
            let mut y = self.m22.mul_add(rect.y(), self.dy);
            let mut w = self.m11 * rect.width();
            let mut h = self.m22 * rect.height();
            if w < 0.0 {
                w = -w;
                x -= w;
            }
            if h < 0.0 {
                h = -h;
                y -= h;
            }
            RectF::from(x, y, w, h)
        } else {
            let (x0, y0) = self.map(rect.x(), rect.y());
            let mut xmin = x0;
            let mut ymin = y0;
            let mut xmax = x0;
            let mut ymax = y0;
            let (x, y) = self.map(rect.x() + rect.width(), rect.y());
            xmin = xmin.min(x);
            ymin = ymin.min(y);
            xmax = xmax.max(x);
            ymax = ymax.max(y);
            let (x, y) = self.map(rect.x() + rect.width(), rect.y() + rect.height());
            xmin = xmin.min(x);
            ymin = ymin.min(y);
            xmax = xmax.max(x);
            ymax = ymax.max(y);
            let (x, y) = self.map(rect.x(), rect.y() + rect.height());
            xmin = xmin.min(x);
            ymin = ymin.min(y);
            xmax = xmax.max(x);
            ymax = ymax.max(y);
            RectF::from(xmin, ymin, xmax - xmin, ymax - ymin)
        }
    }

    /// Resets the matrix to an identity matrix.
    ///
    /// i.e. all elements are set to zero, except m11 and m22 (specifying the scale) which are set to 1.
    pub fn reset(&mut self) {
        self.m11 = 1.0;
        self.m12 = 0.0;
        self.m21 = 0.0;
        self.m22 = 1.0;
        self.dx = 0.0;
        self.dy = 0.0;
    }

    /// Rotates the coordinate system the given degrees counterclockwise.
    pub fn rotate(&mut self, degree: f64) -> &mut Self {
        // pi/180
        const DEG2RAD: f64 = PI / 180.0;
        let mut sina = 0.0;
        let mut cosa = 0.0;
        if fuzzy_compare(degree, 90.0) || fuzzy_compare(degree, -270.0) {
            sina = 1.0;
        } else if fuzzy_compare(degree, 270.0) || fuzzy_compare(degree, -90.0) {
            sina = -1.0;
        } else if fuzzy_compare(degree, 180.0) {
            cosa = -1.0;
        } else {
            // convert to radians
            let b = DEG2RAD * degree;
            // fast and convenient
            sina = b.sin();
            cosa = b.cos();
        }
        let tm11 = cosa.mul_add(self.m11, sina * self.m21);
        let tm12 = cosa.mul_add(self.m12, sina * self.m22);
        let tm21 = (-sina).mul_add(self.m11, cosa * self.m21);
        let tm22 = (-sina).mul_add(self.m12, cosa * self.m22);
        self.m11 = tm11;
        self.m12 = tm12;
        self.m21 = tm21;
        self.m22 = tm22;
        self
    }

    /// Scales the coordinate system by sx horizontally and sy vertically,
    /// and returns a reference to the matrix.
    pub fn scale(&mut self, sx: f64, sy: f64) -> &mut Self {
        self.m11 *= sx;
        self.m12 *= sx;
        self.m21 *= sy;
        self.m22 *= sy;
        self
    }

    /// Sets the matrix elements to the specified values, `m11`, `m12`, `m21`, `m22`, `dx` and `dy`.
    ///
    /// Note that this function replaces the previous values.
    ///
    /// Matrix provide the `translate()`, `rotate()`, `scale()` and `shear()` convenience functions
    /// to manipulate the various matrix elements based on the currently defined coordinate system.
    pub fn set_matrix(&mut self, m11: f64, m12: f64, m21: f64, m22: f64, dx: f64, dy: f64) {
        self.m11 = m11;
        self.m12 = m12;
        self.m21 = m21;
        self.m22 = m22;
        self.dx = dx;
        self.dy = dy;
    }

    /// Shears the coordinate system by sh horizontally and sv vertically,
    /// and returns a reference to the matrix.
    pub fn shear(&mut self, sh: f64, sv: f64) -> &mut Self {
        let tm11 = sv * self.m21;
        let tm12 = sv * self.m22;
        let tm21 = sh * self.m11;
        let tm22 = sh * self.m12;
        self.m11 += tm11;
        self.m12 += tm12;
        self.m21 += tm21;
        self.m22 += tm22;
        self
    }

    /// Moves the coordinate system `dx` along the x axis and `dy` along the y axis,
    /// and returns a reference to the matrix.
    pub fn translate(&mut self, dx: f64, dy: f64) -> &mut Self {
        self.dx += dx.mul_add(self.m11, dy * self.m21);
        self.dy += dy.mul_add(self.m22, dx * self.m12);
        self
    }

    /// The `fuzzy_compare` method is for comparing two matrices using a fuzziness factor.
    ///
    /// Returns true if `self` and `other` are equal, allowing for a small
    /// fuzziness factor for floating-point comparisons; false otherwise.
    #[must_use]
    pub fn fuzzy_compare(&self, other: &Self) -> bool {
        fuzzy_compare(self.m11, other.m11)
            && fuzzy_compare(self.m12, other.m12)
            && fuzzy_compare(self.m21, other.m21)
            && fuzzy_compare(self.m22, other.m22)
            && fuzzy_compare(self.dx, other.dx)
            && fuzzy_compare(self.dy, other.dy)
    }
}

impl ops::Mul<&Matrix> for &Matrix {
    type Output = Matrix;

    /// Returns the result of multiplying this matrix by the given matrix.
    ///
    /// Note that matrix multiplication is not commutative, i.e. a*b != b*a.
    #[allow(clippy::similar_names)]
    fn mul(self, m: &Matrix) -> Self::Output {
        let tm11 = self.m11.mul_add(m.m11, self.m12 * m.m21);
        let tm12 = self.m11.mul_add(m.m12, self.m12 * m.m22);
        let tm21 = self.m21.mul_add(m.m11, self.m22 * m.m21);
        let tm22 = self.m21.mul_add(m.m12, self.m22 * m.m22);

        let tdx = self.dx.mul_add(m.m11, self.dy * m.m21) + m.dx;
        let tdy = self.dx.mul_add(m.m12, self.dy * m.m22) + m.dy;

        Matrix::from(tm11, tm12, tm21, tm22, tdx, tdy)
    }
}

impl ops::MulAssign<&Self> for Matrix {
    /// Returns the result of multiplying this matrix by the given matrix.
    #[allow(clippy::similar_names)]
    fn mul_assign(&mut self, m: &Self) {
        let tm11 = self.m11.mul_add(m.m11, self.m12 * m.m21);
        let tm12 = self.m11.mul_add(m.m12, self.m12 * m.m22);
        let tm21 = self.m21.mul_add(m.m11, self.m22 * m.m21);
        let tm22 = self.m21.mul_add(m.m12, self.m22 * m.m22);

        let tdx = self.dx.mul_add(m.m11, self.dy * m.m21) + m.dx;
        let tdy = self.dx.mul_add(m.m12, self.dy * m.m22) + m.dy;

        self.m11 = tm11;
        self.m12 = tm12;
        self.m21 = tm21;
        self.m22 = tm22;
        self.dx = tdx;
        self.dy = tdy;
    }
}

impl ops::Mul<&Matrix> for PointF {
    type Output = Self;

    /// This is the same as matrix.map(point).
    fn mul(self, matrix: &Matrix) -> Self::Output {
        matrix.map_point(self)
    }
}

impl ops::Mul<&Matrix> for &LineF {
    type Output = LineF;

    /// This is the same as matrix.map(line).
    fn mul(self, matrix: &Matrix) -> Self::Output {
        matrix.map_line(self)
    }
}
