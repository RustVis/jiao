// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

use crate::core::path::Path;
use crate::core::path_types::PathFillType;

#[derive(Debug, Clone)]
pub struct PathBuilder {
    fill_type: PathFillType,
}

impl PathBuilder {
    pub fn snapshot(&mut self) -> Path {
        unimplemented!()
    }

    pub fn set_fill_type(&mut self, fill_type: PathFillType) {
        self.fill_type = fill_type;
    }

    #[must_use]
    pub const fn fill_type(&self) -> PathFillType {
        self.fill_type
    }
}
