// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that can be found
// in the LICENSE file.

use crate::core::matrix::Matrix;

/// These values match the orientation [exif2](www.exif.org/Exif2-2.PDF).
#[repr(u8)]
#[derive(Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
pub enum EncodedOrigin {
    /// Default
    TopLeft = 1,

    /// Reflected across y-axis
    TopRight = 2,

    /// Rotated 180
    BottomRight = 3,

    /// Reflected across x-axis
    BottomLeft = 4,

    /// Reflected across x-axis, Rotated 90 CCW
    LeftTop = 5,

    /// Rotated 90 CW
    RightTop = 6,

    /// Reflected across x-axis, Rotated 90 CW
    RightBottom = 7,

    /// Rotated 90 CCW
    LeftBottom = 8,
}

impl Default for EncodedOrigin {
    fn default() -> Self {
        Self::TopLeft
    }
}

impl EncodedOrigin {
    /// Given an encoded origin and the width and height of the source data, returns a matrix
    /// that transforms the source rectangle with upper left corner at [0, 0] and origin to a correctly
    /// oriented destination rectangle of [0, 0, w, h].
    #[must_use]
    pub const fn to_matrix(self, w: i32, h: i32) -> Matrix {
        match self {
            Self::TopLeft => Matrix::identity(),
            Self::TopRight => Matrix::from_i32(-1, 0, w, 0, 1, 0, 0, 0, 1),
            Self::BottomRight => Matrix::from_i32(-1, 0, w, 0, -1, h, 0, 0, 1),
            Self::BottomLeft => Matrix::from_i32(1, 0, 0, 0, -1, h, 0, 0, 1),
            Self::LeftTop => Matrix::from_i32(0, 1, 0, 1, 0, 0, 0, 0, 1),
            Self::RightTop => Matrix::from_i32(0, -1, w, 1, 0, 0, 0, 0, 1),
            Self::RightBottom => Matrix::from_i32(0, -1, w, -1, 0, h, 0, 0, 1),
            Self::LeftBottom => Matrix::from_i32(0, 1, 0, -1, 0, h, 0, 0, 1),
        }
    }

    /// Return true if the encoded origin includes a 90 degree rotation, in which case the width
    /// and height of the source data are swapped relative to a correctly oriented destination.
    #[must_use]
    pub fn swaps_width_height(self) -> bool {
        self >= Self::LeftTop
    }
}
