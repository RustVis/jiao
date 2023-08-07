// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

use std::ops::{
    Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Neg, Sub, SubAssign,
};

use crate::core::scalar::Scalar;

#[derive(Debug, Clone, PartialEq)]
pub struct V2 {
    x: f32,
    y: f32,
}

impl V2 {
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
        self * (1.0 / self.length())
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

impl Neg for &V2 {
    type Output = V2;

    fn neg(self) -> Self::Output {
        Self::Output {
            x: self.x,
            y: self.y,
        }
    }
}

impl Add<Self> for &V2 {
    type Output = V2;

    fn add(self, other: Self) -> Self::Output {
        Self::Output {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Sub<Self> for &V2 {
    type Output = V2;

    fn sub(self, other: Self) -> Self::Output {
        Self::Output {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl Mul<Self> for &V2 {
    type Output = V2;

    fn mul(self, other: Self) -> Self::Output {
        Self::Output {
            x: self.x * other.x,
            y: self.x * other.y,
        }
    }
}

impl Mul<Scalar> for &V2 {
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

impl AddAssign<&Self> for V2 {
    fn add_assign(&mut self, other: &Self) {
        self.x += other.x;
        self.y += other.y;
    }
}

impl SubAssign<&Self> for V2 {
    fn sub_assign(&mut self, other: &Self) {
        self.x -= other.x;
        self.y -= other.y;
    }
}

impl MulAssign<&Self> for V2 {
    fn mul_assign(&mut self, other: &Self) {
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
    fn default() -> Self {
        Self {
            mat: [
               1.0, 0.0, 0.0, 0,
               0.0, 1.0, 0.0, 0,
               0.0, 0.0, 1.0, 0,
               0.0, 0.0, 0.0, 1,
            ],
        }
    }
}

impl M44 {
    #[must_use]
    pub const fn make_uninitialized() -> Self {
        Self {
            mat: [
               0, 0, 0, 0,
               0, 0, 0, 0,
               0, 0, 0, 0,
               0, 0, 0, 0,
            ],
        }
    }

    #[must_use]
    pub const fn make_nan() -> Self {
        Self {
            mat: [
               0, 0, 0, 0,
               0, 0, 0, 0,
               0, 0, 0, 0,
               0, 0, 0, 0,
            ],
        }
    }

    #[must_use]
    pub const fn make() -> Self {
        Self::default()
    }


    SkM44(const SkM44& a, const SkM44& b) {
        this->setConcat(a, b);
    }

    enum NaN_Constructor {
        kNaN_Constructor
    };
    constexpr SkM44(NaN_Constructor)
        : fMat{SK_ScalarNaN, SK_ScalarNaN, SK_ScalarNaN, SK_ScalarNaN,
               SK_ScalarNaN, SK_ScalarNaN, SK_ScalarNaN, SK_ScalarNaN,
               SK_ScalarNaN, SK_ScalarNaN, SK_ScalarNaN, SK_ScalarNaN,
               SK_ScalarNaN, SK_ScalarNaN, SK_ScalarNaN, SK_ScalarNaN}
    {}

    /**
     *  The constructor parameters are in row-major order.
     */
    constexpr SkM44(SkScalar m0, SkScalar m4, SkScalar m8,  SkScalar m12,
                    SkScalar m1, SkScalar m5, SkScalar m9,  SkScalar m13,
                    SkScalar m2, SkScalar m6, SkScalar m10, SkScalar m14,
                    SkScalar m3, SkScalar m7, SkScalar m11, SkScalar m15)
        // fMat is column-major order in memory.
        : fMat{m0,  m1,  m2,  m3,
               m4,  m5,  m6,  m7,
               m8,  m9,  m10, m11,
               m12, m13, m14, m15}
    {}

    static SkM44 Rows(const SkV4& r0, const SkV4& r1, const SkV4& r2, const SkV4& r3) {
        SkM44 m(kUninitialized_Constructor);
        m.setRow(0, r0);
        m.setRow(1, r1);
        m.setRow(2, r2);
        m.setRow(3, r3);
        return m;
    }
    static SkM44 Cols(const SkV4& c0, const SkV4& c1, const SkV4& c2, const SkV4& c3) {
        SkM44 m(kUninitialized_Constructor);
        m.setCol(0, c0);
        m.setCol(1, c1);
        m.setCol(2, c2);
        m.setCol(3, c3);
        return m;
    }

    static SkM44 RowMajor(const SkScalar r[16]) {
        return SkM44(r[ 0], r[ 1], r[ 2], r[ 3],
                     r[ 4], r[ 5], r[ 6], r[ 7],
                     r[ 8], r[ 9], r[10], r[11],
                     r[12], r[13], r[14], r[15]);
    }
    static SkM44 ColMajor(const SkScalar c[16]) {
        return SkM44(c[0], c[4], c[ 8], c[12],
                     c[1], c[5], c[ 9], c[13],
                     c[2], c[6], c[10], c[14],
                     c[3], c[7], c[11], c[15]);
    }

    static SkM44 Translate(SkScalar x, SkScalar y, SkScalar z = 0) {
        return SkM44(1, 0, 0, x,
                     0, 1, 0, y,
                     0, 0, 1, z,
                     0, 0, 0, 1);
    }

    static SkM44 Scale(SkScalar x, SkScalar y, SkScalar z = 1) {
        return SkM44(x, 0, 0, 0,
                     0, y, 0, 0,
                     0, 0, z, 0,
                     0, 0, 0, 1);
    }

    static SkM44 Rotate(SkV3 axis, SkScalar radians) {
        SkM44 m(kUninitialized_Constructor);
        m.setRotate(axis, radians);
        return m;
    }

    // Scales and translates 'src' to fill 'dst' exactly.
    static SkM44 RectToRect(const SkRect& src, const SkRect& dst);

    static SkM44 LookAt(const SkV3& eye, const SkV3& center, const SkV3& up);
    static SkM44 Perspective(float near, float far, float angle);

    bool operator==(const SkM44& other) const;
    bool operator!=(const SkM44& other) const {
        return !(other == *this);
    }

    void getColMajor(SkScalar v[]) const {
        memcpy(v, fMat, sizeof(fMat));
    }
    void getRowMajor(SkScalar v[]) const;

    SkScalar rc(int r, int c) const {
        SkASSERT(r >= 0 && r <= 3);
        SkASSERT(c >= 0 && c <= 3);
        return fMat[c*4 + r];
    }
    void setRC(int r, int c, SkScalar value) {
        SkASSERT(r >= 0 && r <= 3);
        SkASSERT(c >= 0 && c <= 3);
        fMat[c*4 + r] = value;
    }

    SkV4 row(int i) const {
        SkASSERT(i >= 0 && i <= 3);
        return {fMat[i + 0], fMat[i + 4], fMat[i + 8], fMat[i + 12]};
    }
    SkV4 col(int i) const {
        SkASSERT(i >= 0 && i <= 3);
        return {fMat[i*4 + 0], fMat[i*4 + 1], fMat[i*4 + 2], fMat[i*4 + 3]};
    }

    void setRow(int i, const SkV4& v) {
        SkASSERT(i >= 0 && i <= 3);
        fMat[i + 0]  = v.x;
        fMat[i + 4]  = v.y;
        fMat[i + 8]  = v.z;
        fMat[i + 12] = v.w;
    }
    void setCol(int i, const SkV4& v) {
        SkASSERT(i >= 0 && i <= 3);
        memcpy(&fMat[i*4], v.ptr(), sizeof(v));
    }

    SkM44& setIdentity() {
        *this = { 1, 0, 0, 0,
                  0, 1, 0, 0,
                  0, 0, 1, 0,
                  0, 0, 0, 1 };
        return *this;
    }

    SkM44& setTranslate(SkScalar x, SkScalar y, SkScalar z = 0) {
        *this = { 1, 0, 0, x,
                  0, 1, 0, y,
                  0, 0, 1, z,
                  0, 0, 0, 1 };
        return *this;
    }

    SkM44& setScale(SkScalar x, SkScalar y, SkScalar z = 1) {
        *this = { x, 0, 0, 0,
                  0, y, 0, 0,
                  0, 0, z, 0,
                  0, 0, 0, 1 };
        return *this;
    }

    /**
     *  Set this matrix to rotate about the specified unit-length axis vector,
     *  by an angle specified by its sin() and cos().
     *
     *  This does not attempt to verify that axis.length() == 1 or that the sin,cos values
     *  are correct.
     */
    SkM44& setRotateUnitSinCos(SkV3 axis, SkScalar sinAngle, SkScalar cosAngle);

    /**
     *  Set this matrix to rotate about the specified unit-length axis vector,
     *  by an angle specified in radians.
     *
     *  This does not attempt to verify that axis.length() == 1.
     */
    SkM44& setRotateUnit(SkV3 axis, SkScalar radians) {
        return this->setRotateUnitSinCos(axis, SkScalarSin(radians), SkScalarCos(radians));
    }

    /**
     *  Set this matrix to rotate about the specified axis vector,
     *  by an angle specified in radians.
     *
     *  Note: axis is not assumed to be unit-length, so it will be normalized internally.
     *        If axis is already unit-length, call setRotateAboutUnitRadians() instead.
     */
    SkM44& setRotate(SkV3 axis, SkScalar radians);

    SkM44& setConcat(const SkM44& a, const SkM44& b);

    friend SkM44 operator*(const SkM44& a, const SkM44& b) {
        return SkM44(a, b);
    }

    SkM44& preConcat(const SkM44& m) {
        return this->setConcat(*this, m);
    }

    SkM44& postConcat(const SkM44& m) {
        return this->setConcat(m, *this);
    }

    /**
     *  A matrix is categorized as 'perspective' if the bottom row is not [0, 0, 0, 1].
     *  For most uses, a bottom row of [0, 0, 0, X] behaves like a non-perspective matrix, though
     *  it will be categorized as perspective. Calling normalizePerspective() will change the
     *  matrix such that, if its bottom row was [0, 0, 0, X], it will be changed to [0, 0, 0, 1]
     *  by scaling the rest of the matrix by 1/X.
     *
     *  | A B C D |    | A/X B/X C/X D/X |
     *  | E F G H | -> | E/X F/X G/X H/X |   for X != 0
     *  | I J K L |    | I/X J/X K/X L/X |
     *  | 0 0 0 X |    |  0   0   0   1  |
     */
    void normalizePerspective();

    /** Returns true if all elements of the matrix are finite. Returns false if any
        element is infinity, or NaN.

        @return  true if matrix has only finite elements
    */
    bool isFinite() const { return SkScalarsAreFinite(fMat, 16); }

    /** If this is invertible, return that in inverse and return true. If it is
     *  not invertible, return false and leave the inverse parameter unchanged.
     */
    [[nodiscard]] bool invert(SkM44* inverse) const;

    [[nodiscard]] SkM44 transpose() const;

    void dump() const;

    ////////////

    SkV4 map(float x, float y, float z, float w) const;
    SkV4 operator*(const SkV4& v) const {
        return this->map(v.x, v.y, v.z, v.w);
    }
    SkV3 operator*(SkV3 v) const {
        auto v4 = this->map(v.x, v.y, v.z, 0);
        return {v4.x, v4.y, v4.z};
    }
    ////////////////////// Converting to/from SkMatrix

    /* When converting from SkM44 to SkMatrix, the third row and
     * column is dropped.  When converting from SkMatrix to SkM44
     * the third row and column remain as identity:
     * [ a b c ]      [ a b 0 c ]
     * [ d e f ]  ->  [ d e 0 f ]
     * [ g h i ]      [ 0 0 1 0 ]
     *                [ g h 0 i ]
     */
    SkMatrix asM33() const {
        return SkMatrix::MakeAll(fMat[0], fMat[4], fMat[12],
                                 fMat[1], fMat[5], fMat[13],
                                 fMat[3], fMat[7], fMat[15]);
    }

    explicit SkM44(const SkMatrix& src)
    : SkM44(src[SkMatrix::kMScaleX], src[SkMatrix::kMSkewX],  0, src[SkMatrix::kMTransX],
            src[SkMatrix::kMSkewY],  src[SkMatrix::kMScaleY], 0, src[SkMatrix::kMTransY],
            0,                       0,                       1, 0,
            src[SkMatrix::kMPersp0], src[SkMatrix::kMPersp1], 0, src[SkMatrix::kMPersp2])
    {}

    SkM44& preTranslate(SkScalar x, SkScalar y, SkScalar z = 0);
    SkM44& postTranslate(SkScalar x, SkScalar y, SkScalar z = 0);

    SkM44& preScale(SkScalar x, SkScalar y);
    SkM44& preScale(SkScalar x, SkScalar y, SkScalar z);
    SkM44& preConcat(const SkMatrix&);

private:

    friend class SkMatrixPriv;
}

