// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

use super::ShapeTrait;
use crate::base::PointF;
use crate::base::{LineF, RectF};
use crate::kernel::{PainterTrait, PathTrait};
use crate::platforms::Path;

#[allow(clippy::module_name_repetitions)]
#[derive(Debug)]
pub struct LineShape {
    line: LineF,
    path: Path,
    path_is_dirty: bool,
}

impl Default for LineShape {
    fn default() -> Self {
        Self::from_points(PointF::new(), PointF::new())
    }
}

impl LineShape {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    #[must_use]
    pub fn from_f64(x1: f64, y1: f64, x2: f64, y2: f64) -> Self {
        Self::from_points(PointF::from(x1, y1), PointF::from(x2, y2))
    }

    #[must_use]
    pub fn from_points(p1: PointF, p2: PointF) -> Self {
        let path = Path::new();
        Self {
            line: LineF::from_points(p1, p2),
            path,
            path_is_dirty: true,
        }
    }

    #[must_use]
    pub const fn p1(&self) -> PointF {
        self.line.p1()
    }

    #[must_use]
    pub const fn p2(&self) -> PointF {
        self.line.p2()
    }

    pub fn set_p1(&mut self, point: PointF) {
        self.line.set_p1(point);
        self.path_is_dirty = true;
    }

    pub fn set_p2(&mut self, point: PointF) {
        self.line.set_p2(point);
        self.path_is_dirty = true;
    }

    fn update_path(&mut self) {
        if !self.path_is_dirty {
            return;
        }
        self.path.clear();
        self.path.move_to(self.p1());
        self.path.line_to(self.p2());
        self.path_is_dirty = false;
    }
}

impl ShapeTrait for LineShape {
    fn bounding_rect(&self) -> RectF {
        let rect = RectF::from_points(self.line.p1(), self.line.p2());
        rect.normalized()
    }

    fn repaint(&mut self, painter: &mut dyn PainterTrait) {
        self.update_path();
        painter.stroke(&self.path);
    }
}
