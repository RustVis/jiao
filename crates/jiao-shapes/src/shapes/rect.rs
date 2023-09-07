// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by LGPL-3.0 License that can be found
// in the LICENSE file.

use jiao::base::{PointF, RectF};
use jiao::kernel::{PainterTrait, PathTrait};

use crate::platforms::Path;
use crate::ShapeTrait;

#[derive(Debug, Clone)]
pub struct RectShape {
    rect: RectF,

    path: Path,
    path_is_dirty: bool,
}

impl RectShape {
    /// Create a square shape.
    #[must_use]
    pub fn new_square(width: f64, center: PointF) -> Self {
        let mut rect = RectF::new();
        rect.set_width(width);
        rect.set_height(width);
        rect.move_center(center);
        Self::from_rect(rect)
    }

    /// Create a general rect.
    #[must_use]
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
            self.path.clear();
            self.path.add_rect(&self.rect);
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
