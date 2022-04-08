// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

use super::ShapeTrait;
use crate::base::RectF;
use crate::kernel::{PainterTrait, PathTrait};
use crate::platforms::Path;

#[derive(Debug, Clone)]
pub struct RectShape {
    data: RectF,
    path: Path,
}

impl RectShape {
    pub fn new() -> Self {
        Self::from_rect(RectF::new())
    }

    pub fn from_rect(rect: RectF) -> Self {
        let mut path = Path::new();
        path.rect(&rect);
        Self { data: rect, path }
    }
}

impl ShapeTrait for RectShape {
    fn update(&mut self, painter: &mut dyn PainterTrait) {
        painter.stroke(&self.path);
    }
}
