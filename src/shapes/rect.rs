// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

use super::ShapeTrait;
use crate::base::RectF;
use crate::kernel::{PainterTrait, PathTrait};
use crate::platforms::Path;

#[derive(Debug, Clone)]
pub struct RectShape {
    rect: RectF,
    path: Path,
    path_is_dirty: bool,
}

impl RectShape {
    pub fn new() -> Self {
        Self::from_rect(RectF::new())
    }

    pub fn from_rect(rect: RectF) -> Self {
        let path = Path::new();
        Self {
            rect,
            path,
            path_is_dirty: true,
        }
    }

    fn update_path(&mut self) {
        if self.path_is_dirty {
            self.path = Path::new();
            self.path.rect(&self.rect);
        }
    }
}

impl ShapeTrait for RectShape {
    fn bounding_rect(&self) -> RectF {
        self.rect.clone()
    }

    fn repaint(&mut self, painter: &mut dyn PainterTrait) {
        self.update_path();
        painter.stroke(&self.path);
    }
}
