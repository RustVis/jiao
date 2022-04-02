// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

use super::abstract_shape::AbstractShape;
use crate::base::line::LineF;
use crate::base::point::PointF;

pub struct LineShape {
    data: LineF,
}

impl LineShape {
    pub fn new() -> Self {
        Self { data: LineF::new() }
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
    fn update(&mut self) {}
}
