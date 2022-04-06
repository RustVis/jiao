// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

use super::abstract_shape::AbstractShape;
use crate::base::line::LineF;
use crate::base::point::PointF;
use crate::painting::painter::Painter;

pub struct LineShape {
    data: LineF,
}

impl LineShape {
    pub fn new() -> Self {
        Self { data: LineF::new() }
    }

    pub fn from_f64(x1: f64, y1: f64, x2: f64, y2: f64) -> Self {
        Self::from_points(PointF::from(x1, y1), PointF::from(x2, y2))
    }

    pub fn from_points(p1: PointF, p2: PointF) -> Self {
        Self {
            data: LineF::from_points(p1, p2),
        }
    }

    pub fn set_p1(&mut self, point: PointF) {
        self.data.set_p1(point);
    }

    pub fn set_p2(&mut self, point: PointF) {
        self.data.set_p2(point);
    }
}

impl AbstractShape for LineShape {
    fn update(&mut self, painter: &mut Painter) {
        painter.move_to(self.data.p1());
        painter.line_to(self.data.p2());
    }
}
