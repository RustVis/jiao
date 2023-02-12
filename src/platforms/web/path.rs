// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

use std::any::Any;
use std::f64::consts::PI;
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

    fn add_rect(&mut self, rect: &RectF) {
        self.path2d
            .rect(rect.x(), rect.y(), rect.width(), rect.height());
    }

    fn add_round_rect(&mut self, rect: &RectF, radius: f64) {
        // TODO(Shaohua):
        //self.path2d
        //   .round_rect(rect.x(), rect.y(), rect.width(), rect.height(), radius);

        let x = rect.x();
        let y = rect.y();
        let width = rect.width();
        let height = rect.height();

        // TODO(Shaohua): Returns error.
        let _ret = self.path2d.arc(
            x + width - radius,
            y + radius,
            radius,
            -90.0_f64.to_radians(),
            0.0_f64.to_radians(),
        );
        let _ret = self.path2d.arc(
            x + width - radius,
            y + height - radius,
            radius,
            0.0_f64.to_radians(),
            90.0_f64.to_radians(),
        );
        let _ret = self.path2d.arc(
            x + radius,
            y + height - radius,
            radius,
            90.0_f64.to_radians(),
            180.0_f64.to_radians(),
        );
        let _ret = self.path2d.arc(
            x + radius,
            y + radius,
            radius,
            180.0_f64.to_radians(),
            270.0_f64.to_radians(),
        );
        self.path2d.close_path();
    }

    fn add_circle(&mut self, center: PointF, radius: f64) {
        let start_angle = 0.0;
        let end_angle = 2.0 * PI;
        let _ret = self
            .path2d
            .arc(center.x(), center.y(), radius, start_angle, end_angle);
    }

    fn add_ellipse(&mut self, rect: &RectF) {
        let center = rect.center();
        let radius_x = rect.width() / 2.0;
        let radius_y = rect.height() / 2.0;
        let rotation = 0.0;
        let start_angle = 0.0;
        let end_angle = 2.0 * PI;
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

    fn arc(&mut self, rect: &RectF, start_angle: f64, end_angle: f64) {
        // TODO(Shaohua): Returns error.
        let center = rect.center();
        if rect.width() == rect.height() {
            let radius = rect.height() / 2.0;
            let _ret = self
                .path2d
                .arc(center.x(), center.y(), radius, start_angle, end_angle);
        } else {
            let radius_x = rect.width() / 2.0;
            let radius_y = rect.height() / 2.0;
            let rotation = 0.0;
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

    fn arc_to(&mut self, p1: PointF, p2: PointF, radius: f64) {
        let _ret = self.path2d.arc_to(p1.x(), p1.y(), p2.x(), p2.y(), radius);
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
}
