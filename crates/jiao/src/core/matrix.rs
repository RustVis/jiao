// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

use bitflags::bitflags;

use crate::core::point::Point;
use crate::core::scalar::Scalar;

/// When we transform points through a matrix containing perspective (the bottom row is something
/// other than 0,0,1), the bruteforce math can produce confusing results (since we might divide
/// by 0, or a negative w value).
///
/// By default, methods that map rects and paths will apply perspective clipping,
/// but this can be changed by specifying kYes to those methods.
#[repr(u8)]
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum ApplyPerspectiveClip {
    /// Don't pre-clip the geometry before applying the (perspective) matrix
    No,

    /// Do pre-clip the geometry before applying the (perspective) matrix
    Yes,
}

bitflags! {
    /// `TypeMask`, enum of bit fields for mask returned by `get_type()`.
    ///
    /// Used to identify the complexity of SkMatrix, to optimize performance.
    #[derive(Debug, Clone, Copy, Eq, PartialEq)]
    pub struct TypeMask : u8 {
        /// identity SkMatrix; all bits clear
        const IDENTITY    = 0;

        /// translation SkMatrix
        const TRANSLATE   = 0x01;

        /// scale SkMatrix
        const SCALE       = 0x02;

        /// skew or rotate SkMatrix
        const AFFINE      = 0x04;

        /// perspective SkMatrix
        const PERSPECTIVE = 0x08;

        /// Set if the matrix will map a rectangle to another rectangle.
        ///
        /// This can be true if the matrix is scale-only, or rotates a multiple of
        /// 90 degrees.
        ///
        /// This bit will be set on identity matrices
        const RECT_STAYS_RECT = 0x10;

        /// Set if the perspective bit is valid even though the rest of the matrix is Unknown.
        const ONLY_PERSPECTIVE_VALID = 0x40;

        const UNKNOWN = 0x80;
    }
}

const ORABLE_MASKS: TypeMask =
    TypeMask::TRANSLATE | TypeMask::SCALE | TypeMask::AFFINE | TypeMask::PERSPECTIVE;

const ALL_MASKS: TypeMask = TypeMask::TRANSLATE
    | TypeMask::SCALE
    | TypeMask::AFFINE
    | TypeMask::PERSPECTIVE
    | TypeMask::RECT_STAYS_RECT;

/// Matrix organizes its values in row-major order.
///
/// These members correspond to each value in Matrix.
///
/// horizontal scale factor
pub const M_SCALE_X: usize = 0;

/// horizontal skew factor
pub const M_SKEW_X: usize = 1;

/// horizontal translation
pub const M_TRANS_X: usize = 2;

/// vertical skew factor
pub const M_SKEW_Y: usize = 3;

/// vertical scale factor
pub const M_SCALE_Y: usize = 4;

/// vertical translation
pub const M_TRANS_Y: usize = 5;

/// input x perspective factor
pub const M_PERSP_0: usize = 6;

/// input y perspective factor
pub const M_PERSP_1: usize = 7;

/// perspective bias
pub const M_PERSP_2: usize = 8;

/// Affine arrays are in column-major order to match the matrix used by PDF and XPS.
/// horizontal scale factor
pub const A_SCALE_X: usize = 0;

/// vertical skew factor
pub const A_SKEW_Y: usize = 1;

/// horizontal skew factor
pub const A_SKEW_X: usize = 2;

/// vertical scale factor
pub const A_SCALE_Y: usize = 3;

/// horizontal translation
pub const A_TRANS_X: usize = 4;

/// vertical translation
pub const A_TRANS_Y: usize = 5;

pub type MapXYProc = fn(mat: &Matrix, x: Scalar, y: Scalar, result: &mut Point);

pub type MapPtsProc = fn(mat: &Matrix, dst: &mut [Point], src: &[Point], count: i32);

pub const MAP_XY_PROCS: &[MapXYProc] = &[
    Matrix::identity_xy,
    Matrix::trans_xy,
    Matrix::scale_xy,
    Matrix::scale_trans_xy,
    Matrix::rot_xy,
    Matrix::rot_trans_xy,
    Matrix::rot_xy,
    Matrix::rot_trans_xy,
    // repeat the persp proc 8 times
    Matrix::persp_xy,
    Matrix::persp_xy,
    Matrix::persp_xy,
    Matrix::persp_xy,
    Matrix::persp_xy,
    Matrix::persp_xy,
    Matrix::persp_xy,
    Matrix::persp_xy,
];

pub const MAP_PTS_PROCS: &[MapPtsProc] = &[
    Matrix::identity_pts,
    Matrix::trans_pts,
    Matrix::scale_pts,
    Matrix::scale_pts,
    Matrix::affine_vpts,
    Matrix::affine_vpts,
    Matrix::affine_vpts,
    Matrix::affine_vpts,
    // repeat the persp proc 8 times
    Matrix::persp_pts,
    Matrix::persp_pts,
    Matrix::persp_pts,
    Matrix::persp_pts,
    Matrix::persp_pts,
    Matrix::persp_pts,
    Matrix::persp_pts,
    Matrix::persp_pts,
];

impl TypeMask {
    #[must_use]
    pub fn get_map_xy_proc(self) -> MapXYProc {
        debug_assert!((self & !ALL_MASKS) == TypeMask::empty());
        let index = (self & ALL_MASKS).bits() as usize;
        MAP_XY_PROCS[index]
    }

    #[must_use]
    pub fn get_map_pts_proc(self) -> MapPtsProc {
        debug_assert!((self & !ALL_MASKS) == TypeMask::empty());
        let index = (self & ALL_MASKS).bits() as usize;
        MAP_PTS_PROCS[index]
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Matrix {
    mat: [Scalar; 9],
    type_mask: TypeMask,
}

impl Matrix {
    #[must_use]
    pub const fn identity() -> Self {
        unimplemented!()
    }

    #[must_use]
    #[allow(clippy::too_many_arguments)]
    pub const fn from_i32(
        _m00: i32,
        _m01: i32,
        _m02: i32,
        _m10: i32,
        _m11: i32,
        _m12: i32,
        _m20: i32,
        _m21: i32,
        _m22: i32,
    ) -> Self {
        unimplemented!()
    }

    /// Returns a bit field describing the transformations the matrix may perform.
    ///
    /// The bit field is computed conservatively, so it may include false positives.
    /// For example, when Perspectiveis set, all other bits are set.
    ///
    /// Returns Identity, or combinations of: Translate, Scale, Affine, Perspective.
    pub fn get_type(&mut self) -> TypeMask {
        if self.type_mask.contains(TypeMask::UNKNOWN) {
            self.type_mask = self.compute_type_mask();
        }

        // only return the public masks
        self.type_mask & ORABLE_MASKS
    }
}

// Private methods
impl Matrix {
    #[rustfmt::skip]
    fn from(
        sx: Scalar, kx: Scalar, tx: Scalar,
        ky: Scalar, sy: Scalar, ty: Scalar,
        p0: Scalar, p1: Scalar, p2: Scalar,
        type_mask: TypeMask
    ) -> Self {
        Self {
            mat: [
                sx, kx, tx,
                ky, sy, ty,
                p0, p1, p2,
            ],
            type_mask,
        }
    }

    fn compute_inv(_dst: &mut [Scalar; 9], _src: &[Scalar; 9], _inv_det: f64, _is_persp: bool) {
        unimplemented!()
    }

    #[must_use]
    fn compute_type_mask(&self) -> TypeMask {
        unimplemented!()
    }

    #[must_use]
    fn compute_perspective_type_mask(&self) -> TypeMask {
        unimplemented!()
    }

    fn set_type_mask(&mut self, mask: TypeMask) {
        // allow kUnknown or a valid mask
        //SkASSERT(kUnknown_Mask == mask || (mask & kAllMasks) == mask ||
        //         ((kUnknown_Mask | kOnlyPerspectiveValid_Mask) & mask)
        //         == (kUnknown_Mask | kOnlyPerspectiveValid_Mask));
        self.type_mask = mask;
    }

    fn or_type_mask(&mut self, mask: TypeMask) {
        debug_assert!(mask.contains(ORABLE_MASKS));
        self.type_mask |= mask;
    }

    fn clear_type_mask(&mut self, mask: TypeMask) {
        // only allow a valid mask
        self.type_mask &= !mask;
    }

    fn get_perspective_type_mask_only(&mut self) -> TypeMask {
        if self.type_mask.contains(TypeMask::UNKNOWN)
            && !self.type_mask.contains(TypeMask::ONLY_PERSPECTIVE_VALID)
        {
            self.type_mask = self.compute_perspective_type_mask();
        }

        self.type_mask & ORABLE_MASKS
    }

    /// Returns true if we already know that the matrix is identity; false otherwise.
    #[must_use]
    const fn is_trivially_identity(&self) -> bool {
        if self.type_mask.contains(TypeMask::UNKNOWN) {
            false
        } else {
            self.type_mask & ORABLE_MASKS == TypeMask::empty()
        }
    }

    fn update_translate_mask(&mut self) {
        if self.mat[M_TRANS_X] != 0.0 || self.mat[M_TRANS_Y] != 0.0 {
            self.type_mask |= TypeMask::TRANSLATE;
        } else {
            self.type_mask &= !TypeMask::TRANSLATE;
        }
    }

    #[must_use]
    fn get_map_xy_proc(&self) -> MapXYProc {
        self.get_type().get_map_xy_proc()
    }

    #[must_use]
    fn get_map_pts_proc(&self) -> MapPtsProc {
        self.get_type().get_map_pts_proc()
    }

    #[must_use]
    fn invert_non_identity(&self, _inverse: &mut Self) -> bool {
        unimplemented!()
    }

    #[must_use]
    fn poly2proc(points: &[Point], matrix: &mut Self) -> bool {
        unimplemented!()
    }

    #[must_use]
    fn poly3proc(points: &[Point], matrix: &mut Self) -> bool {
        unimplemented!()
    }

    #[must_use]
    fn poly4proc(points: &[Point], matrix: &mut Self) -> bool {
        unimplemented!()
    }

    fn identity_xy(&self, x: Scalar, y: Scalar, point: &mut Point) {
        unimplemented!()
    }

    fn trans_xy(&self, x: Scalar, y: Scalar, point: &mut Point) {
        unimplemented!()
    }

    fn scale_xy(&self, x: Scalar, y: Scalar, point: &mut Point) {
        unimplemented!()
    }

    fn scale_trans_xy(&self, x: Scalar, y: Scalar, point: &mut Point) {
        unimplemented!()
    }

    fn rot_xy(&self, x: Scalar, y: Scalar, point: &mut Point) {
        unimplemented!()
    }

    fn rot_trans_xy(&self, x: Scalar, y: Scalar, point: &mut Point) {
        unimplemented!()
    }

    fn persp_xy(&self, x: Scalar, y: Scalar, point: &mut Point) {
        unimplemented!()
    }

    fn identity_pts(&self, dest: &mut [Point], src: &[Point], count: i32) {
        unimplemented!()
    }

    fn trans_pts(&self, dest: &mut [Point], src: &[Point], count: i32) {
        unimplemented!()
    }

    fn scale_pts(&self, dest: &mut [Point], src: &[Point], count: i32) {
        unimplemented!()
    }

    fn scale_trans_pts(&self, dest: &mut [Point], src: &[Point], count: i32) {
        unimplemented!()
    }

    fn persp_pts(&self, dest: &mut [Point], src: &[Point], count: i32) {
        unimplemented!()
    }

    fn affine_vpts(&self, dest: &mut [Point], src: &[Point], count: i32) {
        unimplemented!()
    }

    /// return the number of bytes written, whether or not buffer is null
    fn write_to_memory(&self, buffer: &mut [u8]) -> usize {
        unimplemented!()
    }

    /// Reads data from the buffer parameter
    ///
    /// # Parameters
    /// - `buffer` Memory to read from
    ///
    /// Returns number of bytes read (must be a multiple of 4) or 0 if there was
    /// not enough memory available
    fn read_from_memory(&mut self, buffer: &[u8]) -> usize {
        unimplemented!()
    }
}
