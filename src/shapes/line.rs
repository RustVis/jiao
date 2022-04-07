// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

use super::AbstractShape;
use crate::base::LineF;
use crate::base::PointF;
use crate::kernel::{PainterTrait, PathTrait};
use crate::platforms::Path;

pub struct LineShape {
    data: LineF,
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
            data: LineF::from_points(p1, p2),
            path,
        }
    }

    pub fn p1(&self) -> PointF {
        self.data.p1()
    }

    pub fn p2(&self) -> PointF {
        self.data.p2()
    }

    pub fn set_p1(&mut self, point: PointF) {
        self.data.set_p1(point);
        self.path.move_to(point);
    }

    pub fn set_p2(&mut self, point: PointF) {
        self.data.set_p2(point);
        self.path.line_to(point);
    }
}

impl AbstractShape for LineShape {
    fn update(&mut self, painter: &mut dyn PainterTrait) {
        log::info!(
            "LineShape::update(), p1: {:?}, p2: {:?}",
            self.p1(),
            self.p2()
        );
        painter.stroke(&self.path);
    }
}
