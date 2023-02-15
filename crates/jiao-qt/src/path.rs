// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

use cpp_core::CppBox;
use jiao::base::{PointF, RectF};
use jiao::kernel::PathTrait;
use qt_gui::QPainterPath;
use std::any::Any;

#[derive(Debug)]
pub struct Path {
    pub path: CppBox<QPainterPath>,
}

impl Clone for Path {
    fn clone(&self) -> Self {
        let path = unsafe { QPainterPath::new_copy(&self.path) };
        Self { path }
    }
}

impl Path {
    #[must_use]
    pub fn new() -> Self {
        let path = unsafe { QPainterPath::new_0a() };
        Self { path }
    }

    pub fn path(&mut self) -> &CppBox<QPainterPath> {
        &self.path
    }
}

impl PathTrait for Path {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn clear(&mut self) {
        unsafe {
            self.path.clear();
        }
    }

    #[inline]
    fn add_path(&mut self, _other: &dyn PathTrait) {
        todo!()
        //self.path().add_path(other.path());
    }

    #[inline]
    fn close_path(&mut self) {
        unsafe {
            self.path.close_subpath();
        }
    }

    #[inline]
    fn move_to(&mut self, point: PointF) {
        unsafe {
            self.path.move_to_2a(point.x(), point.y());
        }
    }

    #[inline]
    fn line_to(&mut self, point: PointF) {
        unsafe {
            self.path.line_to_2a(point.x(), point.y());
        }
    }

    fn add_rect(&mut self, rect: &RectF) {
        unsafe {
            self.path
                .add_rect_4a(rect.x(), rect.y(), rect.width(), rect.height());
        }
    }

    fn add_round_rect(&mut self, rect: &RectF, radius: f64) {
        unsafe {
            self.path.add_rounded_rect_6a(
                rect.x(),
                rect.y(),
                rect.width(),
                rect.height(),
                radius,
                radius,
            );
        }
    }
    fn add_circle(&mut self, center: PointF, radius: f64) {
        unsafe {
            let diameter = radius * 2.0;
            self.path
                .add_ellipse_4a(center.x() - radius, center.y() - radius, diameter, diameter);
        }
    }

    fn add_ellipse(&mut self, rect: &RectF) {
        unsafe {
            self.path
                .add_ellipse_4a(rect.x(), rect.y(), rect.width(), rect.height());
        }
    }

    fn arc(&mut self, rect: &RectF, start_angle: f64, end_angle: f64) {
        // FIXME(Shaohua): Calc sweep_length.
        let sweep_length = end_angle;
        unsafe {
            self.path.arc_to_6a(
                rect.x(),
                rect.y(),
                rect.width(),
                rect.height(),
                start_angle,
                sweep_length,
            );
        }
    }

    fn arc_to(&mut self, _p1: PointF, _p2: PointF, _radius: f64) {
        // TODO(Shaohua):
        todo!()
    }

    fn cubic_to(&mut self, p1: PointF, p2: PointF, end_point: PointF) {
        unsafe {
            self.path
                .cubic_to_6a(p1.x(), p1.y(), p2.x(), p2.y(), end_point.x(), end_point.y());
        }
    }

    fn quad_to(&mut self, control_point: PointF, end_point: PointF) {
        unsafe {
            self.path.quad_to_4a(
                control_point.x(),
                control_point.y(),
                end_point.x(),
                end_point.y(),
            );
        }
    }
}
