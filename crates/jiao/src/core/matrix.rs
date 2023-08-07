// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

use crate::core::scalar::Scalar;

#[derive(Debug, Clone, PartialEq)]
pub struct Matrix {
    mat: [Scalar; 9],
    type_mask: i32,
}
