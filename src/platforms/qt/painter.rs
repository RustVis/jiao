// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

use qt_gui::{QPainter, QPainterPath};
use std::fmt;

use crate::base::PointF;
use crate::kernel::PathTrait;
use crate::kernel::{PainterTrait, RectF};

pub struct Painter {
    painter: QPainter,
}

impl fmt::Debug for Painter {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Painter").finish()
    }
}

impl Painter {
    pub fn new() -> Self {
        let painter = QPainter::new();
        Self { painter }
    }
}

pub struct Path {
    path: QPainterPath,
}

impl fmt::Debug for Path {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Path").finish()
    }
}

impl Path {
    pub fn new() -> Self {
        let path = QPainterPath::new();
        Self { path }
    }

    pub fn path(&mut self) -> &mut QPainterPath {
        &mut self.path
    }
}

impl PathTrait for Path {
    fn clear(&mut self) {
        self.path = Path2d::new().unwrap();
    }

    #[inline]
    fn add_path(&mut self, other: &Self) {
        self.path().add_path(other.path());
    }

    #[inline]
    fn close_path(&mut self) {
        self.path.close_path();
    }

    #[inline]
    fn move_to(&mut self, point: PointF) {
        self.path.move_to(point.x(), point.y());
    }

    #[inline]
    fn line_to(&mut self, point: PointF) {
        self.path.line_to(point.x(), point.y());
    }

    fn rect(&mut self, rect: &RectF) {
        self.path
            .rect(rect.x(), rect.y(), rect.width(), rect.height());
    }

    fn cubic_to(&mut self, p1: PointF, p2: PointF, end_point: PointF) {
        self.path
            .bezier_curve_to(p1.x(), p1.y(), p2.x(), p2.y(), end_point.x(), end_point.y());
    }

    fn quad_to(&mut self, control_point: PointF, end_point: PointF) {
        self.path.quadratic_curve_to(
            control_point.x(),
            control_point.y(),
            end_point.x(),
            end_point.y(),
        );
    }

    fn arc(&mut self, center: PointF, radius: f64, start_angle: f64, end_angle: f64) {
        // TODO(Shaohua): Returns error
        let _ = self
            .path
            .arc(center.x(), center.y(), radius, start_angle, end_angle);
    }

    fn arc_to(&mut self, p1: PointF, p2: PointF, radius: f64) {
        // TODO(Shaohua): Returns error
        let _ = self.path.arc_to(p1.x(), p1.y(), p2.x(), p2.y(), radius);
    }

    fn ellipse(
        &mut self,
        center: PointF,
        radius_x: f64,
        radius_y: f64,
        rotation: f64,
        start_angle: f64,
        end_angle: f64,
    ) {
        // TODO(Shaohua): Returns error
        let _ = self.path.ellipse(
            center.x(),
            center.y(),
            radius_x,
            radius_y,
            rotation,
            start_angle,
            end_angle,
        );
    }
}
