// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

use super::Path2D;
use crate::base::RectF;
use crate::kernel::{PainterTrait, PathTrait, ShapeTrait};

#[derive(Debug)]
pub struct RoundRectShape {
    rect: RectF,
    radius: f64,
    path: Path2D,
    path_is_dirty: bool,
}

impl RoundRectShape {
    /// Create a general rounded rect.
    #[must_use]
    pub fn new(rect: RectF, radius: f64) -> Self {
        let path = Path2D::new();
        Self {
            rect,
            radius,
            path,
            path_is_dirty: true,
        }
    }

    fn update_path(&mut self) {
        if self.path_is_dirty {
            self.path.clear();
            self.path.add_round_rect(&self.rect, self.radius);
        }
    }
}

impl ShapeTrait for RoundRectShape {
    fn bounding_rect(&self) -> RectF {
        self.rect.clone()
    }

    fn repaint(&mut self, painter: &mut dyn PainterTrait) {
        self.update_path();
        painter.stroke(&self.path);
    }
}
