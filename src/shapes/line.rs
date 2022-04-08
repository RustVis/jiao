// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

use super::ShapeTrait;
use crate::base::PointF;
use crate::base::{LineF, RectF};
use crate::kernel::{PainterTrait, PathTrait};
use crate::platforms::Path;

#[derive(Debug, Clone)]
pub struct LineShape {
    line: LineF,
    path: Path,
}

impl LineShape {
    pub fn new() -> Self {
        Self::from_points(PointF::new(), PointF::new())
    }

    pub fn from_f64(x1: f64, y1: f64, x2: f64, y2: f64) -> Self {
        Self::from_points(PointF::from(x1, y1), PointF::from(x2, y2))
    }

    pub fn from_points(p1: PointF, p2: PointF) -> Self {
        let mut path = Path::new();
        path.move_to(p1);
        path.line_to(p2);
        Self {
            line: LineF::from_points(p1, p2),
            path,
        }
    }

    pub fn p1(&self) -> PointF {
        self.line.p1()
    }

    pub fn p2(&self) -> PointF {
        self.line.p2()
    }

    pub fn set_p1(&mut self, point: PointF) {
        self.line.set_p1(point);
        self.path.move_to(point);
    }

    pub fn set_p2(&mut self, point: PointF) {
        self.line.set_p2(point);
        self.path.line_to(point);
    }
}

impl ShapeTrait for LineShape {
    fn bounding_rect(&self) -> RectF {
        let rect = RectF::from_points(self.line.p1(), self.line.p2());
        rect.normalized()
    }

    fn repaint(&mut self, painter: &mut dyn PainterTrait) {
        log::info!(
            "LineShape::update(), p1: {:?}, p2: {:?}",
            self.p1(),
            self.p2()
        );
        painter.stroke(&self.path);
    }
}
