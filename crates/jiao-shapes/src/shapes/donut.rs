// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that can be found
// in the LICENSE file.

use jiao::base::{PointF, RectF};
use jiao::kernel::{PainterTrait, PathTrait};
use std::f64::consts::PI;

use crate::platforms::Path;
use crate::ShapeTrait;

const DEFAULT_ROUND: u32 = 36;

#[derive(Debug, Clone)]
pub struct DonutShape {
    center: PointF,
    radius: f64,
    round: u32,

    path: Path,
    path_is_dirty: bool,
}

impl DonutShape {
    #[must_use]
    pub fn new(center: PointF, radius: f64) -> Self {
        let path = Path::new();
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
            let angle = (f64::from(i) * PI) / f64::from(self.round);
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
