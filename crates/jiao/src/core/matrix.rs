// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

use crate::core::scalar::Scalar;

#[derive(Debug, Clone, PartialEq)]
pub struct Matrix {
    mat: [Scalar; 9],
    type_mask: i32,
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
}
