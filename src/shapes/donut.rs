// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

use std::f64::consts::PI;

use super::Path2D;
use crate::base::{PointF, RectF};
use crate::kernel::{PainterTrait, PathTrait, ShapeTrait};

const DEFAULT_ROUND: u32 = 36;

#[derive(Debug, Clone)]
pub struct DonutShape {
    center: PointF,
    radius: f64,
    round: u32,

    path: Path2D,
    path_is_dirty: bool,
}

impl DonutShape {
    #[must_use]
    pub fn new(center: PointF, radius: f64) -> Self {
        let path = Path2D::new();
        Self {
            center,
            radius,
            round: DEFAULT_ROUND,
            path,
            path_is_dirty: true,
        }
    }

    fn update_path(&mut self, painter: &mut dyn PainterTrait) {
        if !self.path_is_dirty {
            return;
        }

        self.path.clear();
        painter.translate(self.center);
        self.path.add_circle(PointF::new(), self.radius);
        painter.stroke(&self.path);

        for i in 0..self.round {
            let angle = (i as f64 * PI) / self.round as f64;
            painter.save();
            painter.rotate(angle);
            painter.scale(0.3, 1.0);
            painter.stroke(&self.path);
            painter.restore();
        }

        self.path_is_dirty = false;
    }
}

impl ShapeTrait for DonutShape {
    fn bounding_rect(&self) -> RectF {
        let rect = RectF::from_circle(self.center, self.radius);
        rect.normalized()
    }

    fn repaint(&mut self, painter: &mut dyn PainterTrait) {
        self.update_path(painter);
    }
}
