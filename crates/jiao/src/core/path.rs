// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

use crate::core::path_types::PathFillType;

#[derive(Debug, Clone)]
pub struct Path {
    fill_type: PathFillType,
    pub uuid: String,
}

impl Path {
    #[must_use]
    pub const fn fill_type(&self) -> PathFillType {
        self.fill_type
    }
}
