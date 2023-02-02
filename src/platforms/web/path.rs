// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

use std::any::Any;
use web_sys::Path2d;

use crate::base::{PointF, RectF};
use crate::kernel::PathTrait;

#[derive(Debug, Clone)]
pub struct Path {
    path2d: Path2d,
}

impl Path {
    /// # Panics
    /// Got panic if failed to create path2d object.
    #[must_use]
    pub fn new() -> Self {
        let path2d = Path2d::new().unwrap();
        Self { path2d }
    }

    /// Get inner `Path2D` object.
    #[must_use]
    pub const fn path2d(&self) -> &Path2d {
        &self.path2d
    }
}

impl PathTrait for Path {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn clear(&mut self) {
        self.path2d = Path2d::new().unwrap();
    }

    #[inline]
    fn add_path(&mut self, other: &dyn PathTrait) {
        let other_ref = other.as_any().downcast_ref::<Self>().unwrap();
        self.path2d().add_path(other_ref.path2d());
    }

    #[inline]
    fn close_path(&mut self) {
        self.path2d.close_path();
    }

    #[inline]
    fn move_to(&mut self, point: PointF) {
        self.path2d.move_to(point.x(), point.y());
    }

    #[inline]
    fn line_to(&mut self, point: PointF) {
        self.path2d.line_to(point.x(), point.y());
    }

    fn rect(&mut self, rect: &RectF) {
        self.path2d
            .rect(rect.x(), rect.y(), rect.width(), rect.height());
    }

    fn cubic_to(&mut self, p1: PointF, p2: PointF, end_point: PointF) {
        self.path2d
            .bezier_curve_to(p1.x(), p1.y(), p2.x(), p2.y(), end_point.x(), end_point.y());
    }

    fn quad_to(&mut self, control_point: PointF, end_point: PointF) {
        self.path2d.quadratic_curve_to(
            control_point.x(),
            control_point.y(),
            end_point.x(),
            end_point.y(),
        );
    }

    fn arc(&mut self, center: PointF, radius: f64, start_angle: f64, end_angle: f64) {
        // TODO(Shaohua): Returns error
        let _ret = self
            .path2d
            .arc(center.x(), center.y(), radius, start_angle, end_angle);
    }

    fn arc_to(&mut self, p1: PointF, p2: PointF, radius: f64) {
        // TODO(Shaohua): Returns error
        let _ret = self.path2d.arc_to(p1.x(), p1.y(), p2.x(), p2.y(), radius);
    }

    fn ellipse(
        &mut self,
        center: PointF,
        radius_x: f64,
        radius_y: f64,
        start_angle: f64,
        end_angle: f64,
    ) {
        let rotation = 0.0;
        // TODO(Shaohua): Returns error
        let _ret = self.path2d.ellipse(
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
