// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that can be found
// in the LICENSE file.

use std::ops::{
    Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Neg, Sub, SubAssign,
};

use crate::core::matrix::Matrix;
use crate::core::rect::Rect;
use crate::core::scalar::{Scalar, ScalarExt, SCALAR_1, SCALAR_NAN};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct V2 {
    x: f32,
    y: f32,
}

impl V2 {
    #[must_use]
    pub const fn make(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    #[must_use]
    #[inline]
    pub const fn x(&self) -> f32 {
        self.x
    }

    #[must_use]
    pub fn dot(&self, other: &Self) -> Scalar {
        self.x.mul_add(other.x, self.y * other.y)
    }

    #[must_use]
    pub fn cross(&self, other: &Self) -> Scalar {
        self.x.mul_add(other.y, -self.y * other.x)
    }

    #[must_use]
    pub fn normalize(&self) -> Self {
        *self * (1.0 / self.length())
    }

    #[must_use]
    pub fn length_squared(&self) -> Scalar {
        self.dot(self)
    }

    #[must_use]
    pub fn length(&self) -> Scalar {
        self.length_squared().sqrt()
    }
}

impl Neg for V2 {
    type Output = V2;

    fn neg(self) -> Self::Output {
        Self::Output {
            x: self.x,
            y: self.y,
        }
    }
}

impl Add<Self> for V2 {
    type Output = V2;

    fn add(self, other: Self) -> Self::Output {
        Self::Output {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Sub<Self> for V2 {
    type Output = V2;

    fn sub(self, other: Self) -> Self::Output {
        Self::Output {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl Mul<Self> for V2 {
    type Output = V2;

    fn mul(self, other: Self) -> Self::Output {
        Self::Output {
            x: self.x * other.x,
            y: self.x * other.y,
        }
    }
}

impl Mul<Scalar> for V2 {
    type Output = V2;

    fn mul(self, scale: Scalar) -> Self::Output {
        Self::Output {
            x: self.x * scale,
            y: self.x * scale,
        }
    }
}

impl Mul<V2> for Scalar {
    type Output = V2;

    fn mul(self, v: V2) -> Self::Output {
        Self::Output {
            x: self * v.x,
            y: self * v.y,
        }
    }
}

impl Div<Scalar> for V2 {
    type Output = Self;

    fn div(self, scale: Scalar) -> Self {
        debug_assert!(scale != 0.0);

        Self {
            x: self.x / scale,
            y: self.x / scale,
        }
    }
}

impl Div<V2> for Scalar {
    type Output = V2;

    fn div(self, v: V2) -> Self::Output {
        debug_assert!(v.x != 0.0);
        debug_assert!(v.y != 0.0);

        Self::Output {
            x: self / v.x,
            y: self / v.y,
        }
    }
}

impl AddAssign<Self> for V2 {
    fn add_assign(&mut self, other: Self) {
        self.x += other.x;
        self.y += other.y;
    }
}

impl SubAssign<Self> for V2 {
    fn sub_assign(&mut self, other: Self) {
        self.x -= other.x;
        self.y -= other.y;
    }
}

impl MulAssign<Self> for V2 {
    fn mul_assign(&mut self, other: Self) {
        self.x *= other.x;
        self.y *= other.y;
    }
}

impl MulAssign<Scalar> for V2 {
    fn mul_assign(&mut self, scale: Scalar) {
        self.x *= scale;
        self.y *= scale;
    }
}

impl DivAssign<Scalar> for V2 {
    fn div_assign(&mut self, scale: Scalar) {
        debug_assert!(scale != 0.0);

        self.x /= scale;
        self.y /= scale;
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct V3 {
    x: f32,
    y: f32,
    z: f32,
}

impl V3 {
    #[must_use]
    #[inline]
    pub const fn make(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }

    #[must_use]
    #[inline]
    pub const fn x(&self) -> f32 {
        self.x
    }

    #[must_use]
    #[inline]
    pub const fn y(&self) -> f32 {
        self.y
    }

    #[must_use]
    #[inline]
    pub const fn z(&self) -> f32 {
        self.z
    }

    #[must_use]
    pub fn dot(&self, other: &Self) -> Scalar {
        self.x
            .mul_add(other.x, self.y.mul_add(other.y, self.z * other.z))
    }

    #[must_use]
    pub fn cross(&self, other: &Self) -> Self {
        Self {
            x: self.y.mul_add(other.z, -self.z * other.y),
            y: self.z.mul_add(other.x, -self.x * other.z),
            z: self.x.mul_add(other.y, -self.y - other.x),
        }
    }

    #[must_use]
    pub fn normalize(&self) -> Self {
        self * (1.0 / self.length())
    }

    #[must_use]
    pub fn length_squared(&self) -> Scalar {
        self.dot(self)
    }

    #[must_use]
    pub fn length(&self) -> Scalar {
        self.dot(self).sqrt()
    }
}

impl Neg for &V3 {
    type Output = V3;

    fn neg(self) -> Self::Output {
        Self::Output {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl Neg for V3 {
    type Output = V3;

    fn neg(self) -> Self::Output {
        Self::Output {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl Add<Self> for &V3 {
    type Output = V3;

    fn add(self, other: Self) -> Self::Output {
        Self::Output {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl Sub<Self> for &V3 {
    type Output = V3;

    fn sub(self, other: Self) -> Self::Output {
        Self::Output {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl Mul<Self> for &V3 {
    type Output = V3;

    fn mul(self, other: Self) -> Self::Output {
        Self::Output {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z,
        }
    }
}

impl Mul<Scalar> for &V3 {
    type Output = V3;

    fn mul(self, scale: Scalar) -> Self::Output {
        Self::Output {
            x: self.x * scale,
            y: self.y * scale,
            z: self.z * scale,
        }
    }
}

impl Mul<&V3> for Scalar {
    type Output = V3;

    fn mul(self, v: &V3) -> Self::Output {
        Self::Output {
            x: self * v.x,
            y: self * v.y,
            z: self * v.z,
        }
    }
}

impl AddAssign<&Self> for V3 {
    fn add_assign(&mut self, other: &Self) {
        self.x += other.x;
        self.y += other.y;
        self.z += other.z;
    }
}

impl SubAssign<&Self> for V3 {
    fn sub_assign(&mut self, other: &Self) {
        self.x -= other.x;
        self.y -= other.y;
        self.z -= other.z;
    }
}

impl MulAssign<&Self> for V3 {
    fn mul_assign(&mut self, other: &Self) {
        self.x *= other.x;
        self.y *= other.y;
        self.z *= other.z;
    }
}

impl MulAssign<Scalar> for V3 {
    fn mul_assign(&mut self, scale: Scalar) {
        self.x *= scale;
        self.y *= scale;
        self.z *= scale;
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct V4 {
    x: f32,
    y: f32,
    z: f32,
    w: f32,
}

impl V4 {
    #[must_use]
    #[inline]
    pub const fn make(x: f32, y: f32, z: f32, w: f32) -> Self {
        Self { x, y, z, w }
    }

    #[must_use]
    pub fn dot(&self, other: &Self) -> Scalar {
        self.x.mul_add(
            other.x,
            self.y
                .mul_add(other.y, self.z.mul_add(other.z, self.w * other.w)),
        )
    }

    #[must_use]
    pub fn normalize(&self) -> Self {
        self * (1.0 / self.length())
    }

    #[must_use]
    pub fn length_squared(&self) -> Scalar {
        self.dot(self)
    }

    #[must_use]
    pub fn length(&self) -> Scalar {
        self.dot(self).sqrt()
    }
}

impl Neg for &V4 {
    type Output = V4;

    fn neg(self) -> Self::Output {
        Self::Output {
            x: -self.x,
            y: -self.y,
            z: -self.z,
            w: -self.w,
        }
    }
}

impl Add<Self> for &V4 {
    type Output = V4;

    fn add(self, other: Self) -> Self::Output {
        Self::Output {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
            w: self.w + other.w,
        }
    }
}

impl Sub<Self> for &V4 {
    type Output = V4;

    fn sub(self, other: Self) -> Self::Output {
        Self::Output {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
            w: self.w - other.w,
        }
    }
}

impl Mul<Self> for &V4 {
    type Output = V4;

    fn mul(self, other: Self) -> Self::Output {
        Self::Output {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z,
            w: self.w * other.w,
        }
    }
}

impl Mul<Scalar> for &V4 {
    type Output = V4;

    fn mul(self, scale: Scalar) -> Self::Output {
        Self::Output {
            x: self.x * scale,
            y: self.y * scale,
            z: self.z * scale,
            w: self.w * scale,
        }
    }
}

impl Mul<&V4> for Scalar {
    type Output = V4;

    fn mul(self, v: &V4) -> Self::Output {
        Self::Output {
            x: self * v.x,
            y: self * v.y,
            z: self * v.z,
            w: self * v.w,
        }
    }
}

impl Index<usize> for V4 {
    type Output = f32;

    fn index(&self, index: usize) -> &Self::Output {
        debug_assert!(index < 4);

        match index {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            3 => &self.w,
            _ => panic!("Index out of range"),
        }
    }
}

impl IndexMut<usize> for V4 {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        debug_assert!(index < 4);

        match index {
            0 => &mut self.x,
            1 => &mut self.y,
            2 => &mut self.z,
            3 => &mut self.w,
            _ => panic!("Index out of range"),
        }
    }
}

/// 4x4 matrix used by Canvas and other parts.
///
/// Assumes a right-handed coordinate system:
/// - +X goes to the right
/// - +Y goes down
/// - +Z goes into the screen (away from the viewer)
#[derive(Debug, Clone, PartialEq)]
pub struct M44 {
    // Stored in column-major.
    //  Indices
    //  0  4  8  12        1 0 0 trans_x
    //  1  5  9  13  e.g.  0 1 0 trans_y
    //  2  6 10  14        0 0 1 trans_z
    //  3  7 11  15        0 0 0 1
    mat: [Scalar; 16],
}

impl Default for M44 {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

impl M44 {
    #[must_use]
    #[inline]
    pub const fn new() -> Self {
        Self::make_identity()
    }

    #[must_use]
    pub const fn make_uninitialized() -> Self {
        Self { mat: [0.0; 16] }
    }

    #[must_use]
    pub const fn make_nan() -> Self {
        Self {
            mat: [SCALAR_NAN; 16],
        }
    }

    #[must_use]
    #[rustfmt::skip]
    pub const fn make_identity() -> Self {
        Self {
            mat: [
                // row 1
                1.0, 0.0, 0.0, 0.0,
                // row 2
                0.0, 1.0, 0.0, 0.0,
                // row 3
                0.0, 0.0, 1.0, 0.0,
                // row 4
                0.0, 0.0, 0.0, 1.0,
            ],
        }
    }

    pub fn from_concat(a: &Self, b: &Self) -> Self {
        let mut m = Self::new();
        m.set_concat(a, b);
        m
    }

    /// The constructor parameters are in row-major order.
    #[must_use]
    #[allow(clippy::too_many_arguments)]
    #[rustfmt::skip]
    pub const fn make(
        m0: Scalar,
        m4: Scalar,
        m8: Scalar,
        m12: Scalar,
        m1: Scalar,
        m5: Scalar,
        m9: Scalar,
        m13: Scalar,
        m2: Scalar,
        m6: Scalar,
        m10: Scalar,
        m14: Scalar,
        m3: Scalar,
        m7: Scalar,
        m11: Scalar,
        m15: Scalar,
    ) -> Self {
        // mat is column-major order in memory.
        Self {
            mat: [
                m0, m1, m2, m3,
                m4, m5, m6, m7,
                m8, m9, m10, m11,
                m12, m13, m14, m15,
            ],
        }
    }

    #[must_use]
    pub fn from_rows(r0: &V4, r1: &V4, r2: &V4, r3: &V4) -> Self {
        let mut m = Self::make_uninitialized();
        m.set_row(0, r0);
        m.set_row(1, r1);
        m.set_row(2, r2);
        m.set_row(3, r3);
        m
    }

    #[must_use]
    #[inline]
    pub fn from_cols(c0: &V4, c1: &V4, c2: &V4, c3: &V4) -> Self {
        let mut m = Self::make_uninitialized();
        m.set_col(0, c0);
        m.set_col(1, c1);
        m.set_col(2, c2);
        m.set_col(3, c3);
        m
    }

    #[must_use]
    #[inline]
    #[rustfmt::skip]
    pub const fn from_row_major(r: &[Scalar; 16]) -> Self {
        Self::make(
            r[0], r[1], r[2], r[3],
            r[4], r[5], r[6], r[7],
            r[8], r[9], r[10], r[11],
            r[12], r[13], r[14], r[15],
        )
    }

    #[must_use]
    #[inline]
    #[rustfmt::skip]
    pub const fn from_col_major(c: &[Scalar; 16]) -> Self {
        Self::make(
            c[0], c[4], c[8], c[12],
            c[1], c[5], c[9], c[13],
            c[2], c[6], c[10], c[14],
            c[3], c[7], c[11], c[15],
        )
    }

    #[must_use]
    #[inline]
    #[rustfmt::skip]
    pub const fn from_translate(x: Scalar, y: Scalar, z: Scalar) -> Self {
        Self::make(
            1.0, 0.0, 0.0, x,
            0.0, 1.0, 0.0, y,
            0.0, 0.0, 1.0, z,
            0.0, 0.0, 0.0, 1.0,
        )
    }

    #[must_use]
    #[rustfmt::skip]
    pub const fn from_scale(x: Scalar, y: Scalar, z: Scalar) -> Self {
        Self::make(
            x, 0.0, 0.0, 0.0,
            0.0, y, 0.0, 0.0,
            0.0, 0.0, z, 0.0,
            0.0, 0.0, 0.0, 1.0,
        )
    }

    #[must_use]
    pub fn from_rotate(axis: &V3, radians: Scalar) -> Self {
        let mut m = Self::make_uninitialized();
        m.set_rotate(axis, radians);
        m
    }

    /// Scales and translates 'src' to fill 'dst' exactly.
    #[must_use]
    #[rustfmt::skip]
    pub fn rect_to_rect(src: &Rect, dst: &Rect) -> Self {
        if src.is_empty() {
            return Self::new();
        }
        if dst.is_empty() {
            return Self::from_scale(0.0, 0.0, 0.0);
        }

        let sx = dst.width() / src.width();
        let sy = dst.height() / src.height();

        let tx = dst.left() - sx * src.left();
        let ty = dst.top() - sy * src.top();

        Self::make(
            sx, 0.0, 0.0, tx,
            0.0, sy, 0.0, ty,
            0.0, 0.0, 1.0, 0.0,
            0.0, 0.0, 0.0, 1.0,
        )
    }

    #[must_use]
    pub fn look_at(eye: &V3, center: &V3, up: &V3) -> Self {
        fn v4(v: &V3, w: Scalar) -> V4 {
            V4::make(v.x, v.y, v.z, w)
        }

        fn normalize(v: &V3) -> V3 {
            let len = v.length();
            if len.nearly_zero() {
                v.clone()
            } else {
                v * (1.0 / len)
            }
        }

        let f: V3 = normalize(&(center - eye));
        let u: V3 = normalize(up);
        let s: V3 = normalize(&f.cross(&u));

        let mut m = Self::make_uninitialized();
        let cols_m = Self::from_cols(
            &v4(&s, 0.0),
            &v4(&s.cross(&f), 0.0),
            &v4(&-f, 0.0),
            &v4(eye, 1.0),
        );
        if !cols_m.invert(&mut m) {
            m.set_identity();
        }
        m
    }

    #[must_use]
    pub fn perspective(near: f32, far: f32, angle: f32) -> Self {
        debug_assert!(far > near);

        let denom_inv: f32 = 1.0 / (far - near);
        let half_angle = angle * 0.5;
        debug_assert!(half_angle != 0.0);
        let cot: f32 = 1.0 / half_angle.tan();

        let mut m = Self::new();
        m.set_rc(0, 0, cot);
        m.set_rc(1, 1, cot);
        m.set_rc(2, 2, (far + near) * denom_inv);
        m.set_rc(2, 3, 2.0 * far * near * denom_inv);
        m.set_rc(3, 2, -1.0);
        m
    }

    pub fn get_col_major(&self /*Scalar v[]*/) {
        //memcpy(v, fMat, sizeof(fMat));
        unimplemented!()
    }

    pub fn get_row_major(&self /*Scalar v[]*/) {
        unimplemented!()
    }

    #[must_use]
    pub fn rc(&self, r: usize, c: usize) -> Scalar {
        debug_assert!(r <= 3);
        debug_assert!(c <= 3);
        self.mat[c * 4 + r]
    }

    pub fn set_rc(&mut self, r: usize, c: usize, value: Scalar) {
        debug_assert!(r <= 3);
        debug_assert!(c <= 3);
        self.mat[c * 4 + r] = value;
    }

    #[must_use]
    pub fn row(&self, r: usize) -> V4 {
        debug_assert!(r <= 3);
        V4::make(
            self.mat[r],
            self.mat[r + 4],
            self.mat[r + 8],
            self.mat[r + 12],
        )
    }

    #[must_use]
    pub fn col(&self, c: usize) -> V4 {
        debug_assert!(c <= 3);
        V4::make(
            self.mat[c * 4],
            self.mat[c * 4 + 1],
            self.mat[c * 4 + 2],
            self.mat[c * 4 + 3],
        )
    }

    pub fn set_row(&mut self, r: usize, v: &V4) {
        debug_assert!(r <= 3);
        self.mat[r] = v.x;
        self.mat[r + 4] = v.y;
        self.mat[r + 8] = v.z;
        self.mat[r + 12] = v.w;
    }

    pub fn set_col(&mut self, c: usize, _v: &V4) {
        debug_assert!(c <= 3);
        unimplemented!()
        //memcpy(&fMat[i*4], v.ptr(), sizeof(v));
    }

    pub fn set_identity(&mut self) -> &Self {
        self.mat = [
            1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0,
        ];
        self
    }

    pub fn set_translate(&mut self, x: Scalar, y: Scalar, z: Scalar) -> &Self {
        self.mat = [
            1.0, 0.0, 0.0, x, 0.0, 1.0, 0.0, y, 0.0, 0.0, 1.0, z, 0.0, 0.0, 0.0, 1.0,
        ];
        self
    }

    pub fn set_scale(&mut self, x: Scalar, y: Scalar, z: Scalar) -> &Self {
        self.mat = [
            x, 0.0, 0.0, 0.0, 0.0, y, 0.0, 0.0, 0.0, 0.0, z, 0.0, 0.0, 0.0, 0.0, 1.0,
        ];
        self
    }

    /// Set this matrix to rotate about the specified unit-length axis vector,
    /// by an angle specified by its `sin()` and `cos()`.
    ///
    /// This does not attempt to verify that `axis.length()` == 1 or that the sin,cos values
    /// are correct.
    #[rustfmt::skip]
    pub fn set_rotate_unit_sin_cos(
        &mut self,
        axis: &V3,
        sin_angle: Scalar,
        cos_angle: Scalar,
    ) -> &Self {
        // Taken from "Essential Mathematics for Games and Interactive Applications"
        //             James M. Van Verth and Lars M. Bishop -- third edition
        let x = axis.x();
        let y = axis.y();
        let z = axis.z();
        let c = cos_angle;
        let s = sin_angle;
        let t = 1.0 - c;

        self.mat = [
            t * x * x + c,     t * x * y - s * z, t * x * z + s * y, 0.0,
            t * x * y + s * z, t * y * y + c,     t * y * z - s * x, 0.0,
            t * x * z - s * y, t * y * z + s * x, t * z * z + c,     0.0,
            0.0,               0.0,               0.0,               1.0,
        ];
        self
    }

    /// Set this matrix to rotate about the specified unit-length axis vector,
    /// by an angle specified in radians.
    ///
    /// This does not attempt to verify that `axis.length()` == 1.
    pub fn set_rotate_unit(&mut self, axis: &V3, radians: Scalar) -> &Self {
        self.set_rotate_unit_sin_cos(axis, radians.sin(), radians.cos())
    }

    /// Set this matrix to rotate about the specified axis vector,
    /// by an angle specified in radians.
    ///
    /// Note: axis is not assumed to be unit-length, so it will be normalized internally.
    /// If axis is already unit-length, call `set_rotate_about_unit_radians()` instead.
    pub fn set_rotate(&mut self, axis: &V3, radians: Scalar) -> &Self {
        let len = axis.length();
        if len > 0.0 && len.is_finite() {
            self.set_rotate_unit(&(axis * (SCALAR_1 / len)), radians);
        } else {
            self.set_identity();
        }
        self
    }

    pub fn set_concat(&mut self, _a: &Self, _b: &Self) -> &Self {
        unimplemented!()
    }

    pub fn pre_concat(&mut self, _m: &Self) -> &Self {
        unimplemented!()
        //self.set_concat(self, m)
    }

    pub fn post_concat(&mut self, _m: &Self) -> &Self {
        unimplemented!()
        //self.set_concat(m, self)
    }

    /// A matrix is categorized as 'perspective' if the bottom row is not [0, 0, 0, 1].
    ///
    /// For most uses, a bottom row of [0, 0, 0, X] behaves like a non-perspective matrix, though
    /// it will be categorized as perspective.
    /// Calling `normalize_perspective()` will change the
    /// matrix such that, if its bottom row was [0, 0, 0, X], it will be changed to [0, 0, 0, 1]
    /// by scaling the rest of the matrix by 1/X.
    /// | A B C D |    | A/X B/X C/X D/X |
    /// | E F G H | -> | E/X F/X G/X H/X |   for X != 0
    /// | I J K L |    | I/X J/X K/X L/X |
    /// | 0 0 0 X |    |  0   0   0   1  |
    pub fn normalize_perspective(&self) {
        unimplemented!()
    }

    /// Returns true if all elements of the matrix are finite.
    ///
    /// Returns false if any element is infinity, or NaN.
    #[must_use]
    pub fn is_finite(&self) -> bool {
        unimplemented!()
        //ScalarsAreFinite(fMat, 16);
    }

    /// If this is invertible, return that in inverse and return true.
    /// If it is not invertible, return false and leave the inverse parameter unchanged.
    pub fn invert(&self, _inverse: &mut Self) -> bool {
        unimplemented!()
    }

    #[must_use]
    pub fn transpose(&self) -> Self {
        unimplemented!()
    }

    pub fn dump(&self) {
        unimplemented!()
    }

    #[must_use]
    pub fn map(&self, _x: f32, _y: f32, _z: f32, _w: f32) -> V4 {
        unimplemented!()
    }

    //V4 operator*(const V4& v) const { return this->map(v.x, v.y, v.z, v.w); }
    //V3 operator*(V3 v) const { auto v4 = this->map(v.x, v.y, v.z, 0); return {v4.x, v4.y, v4.z}; }

    // Converting to/from Matrix

    /// When converting from M44 to Matrix, the third row and
    /// column is dropped.
    ///
    /// When converting from Matrix to M44 the third row and column remain as identity:
    /// [ a b c ]      [ a b 0 c ]
    /// [ d e f ]  ->  [ d e 0 f ]
    /// [ g h i ]      [ 0 0 1 0 ]
    ///
    ///                [ g h 0 i ]
    #[must_use]
    pub fn as_m33(&self) -> Matrix {
        unimplemented!()
        //return Matrix::MakeAll(fMat[0], fMat[4], fMat[12],
        //                         fMat[1], fMat[5], fMat[13],
        //                         fMat[3], fMat[7], fMat[15]);
    }

    // TODO(Shaohua):
    /*
    pub fn m44(const Matrix& src)
    : M44(src[Matrix::kMScaleX], src[Matrix::kMewX],  0, src[Matrix::kMTransX],
            src[Matrix::kMewY],  src[Matrix::kMScaleY], 0, src[Matrix::kMTransY],
            0,                       0,                       1, 0,
            src[Matrix::kMPersp0], src[Matrix::kMPersp1], 0, src[Matrix::kMPersp2])
    {}
    */

    pub fn pre_translate(&mut self, _x: Scalar, _y: Scalar, _z: Scalar) -> &Self {
        unimplemented!()
    }

    pub fn post_translate(&mut self, _x: Scalar, _y: Scalar, _z: Scalar) -> &Self {
        unimplemented!()
    }

    pub fn pre_scale(&mut self, _x: Scalar, _y: Scalar) -> &Self {
        unimplemented!()
    }

    pub fn pre_concat_matrix(&mut self, _m: &Matrix) -> &Self {
        unimplemented!()
    }
}
