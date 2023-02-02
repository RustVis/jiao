// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

use cpp_core::CppBox;
use qt_gui::QPainterPath;
use std::any::Any;
use std::fmt;

use crate::base::{PointF, RectF};
use crate::kernel::PathTrait;

pub struct Path {
    pub(crate) path: CppBox<QPainterPath>,
}

impl fmt::Debug for Path {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Path").finish()
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

    fn rect(&mut self, rect: &RectF) {
        unsafe {
            self.path
                .add_rect_4a(rect.x(), rect.y(), rect.width(), rect.height());
        }
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

    fn arc(&mut self, center: PointF, radius: f64, start_angle: f64, end_angle: f64) {
        let rect = RectF::from_circular(center, radius);
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
        todo!()
    }

    fn ellipse(
        &mut self,
        center: PointF,
        radius_x: f64,
        radius_y: f64,
        start_angle: f64,
        end_angle: f64,
    ) {
        let rect = RectF::from_ellipse(center, radius_x, radius_y);
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
}
