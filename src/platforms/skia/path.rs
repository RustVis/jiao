// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

#![allow(clippy::cast_possible_truncation)]

use std::any::Any;

use crate::base::{PointF, RectF};
use crate::kernel::PathTrait;

#[derive(Debug, Clone)]
pub struct Path {
    p: skia_safe::Path,
}

impl Path {
    #[must_use]
    pub fn new() -> Self {
        let p = skia_safe::Path::new();
        Self { p }
    }

    #[must_use]
    pub const fn path(&self) -> &skia_safe::Path {
        &self.p
    }
}

impl Default for Path {
    fn default() -> Self {
        Self::new()
    }
}

impl PathTrait for Path {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn clear(&mut self) {
        // TODO(Shaohua):
    }

    fn add_path(&mut self, other: &dyn PathTrait) {
        let other_ref = other.as_any().downcast_ref::<Self>().unwrap();
        self.p.add_path(&other_ref.p, (0.0, 0.0), None);
    }

    fn close_path(&mut self) {
        self.p.close();
    }

    #[inline]
    fn move_to(&mut self, point: PointF) {
        self.p.move_to(point);
    }

    #[inline]
    fn line_to(&mut self, point: PointF) {
        self.p.line_to(point);
    }

    fn add_rect(&mut self, rect: &RectF) {
        let sk_rect: skia_safe::Rect = rect.into();
        self.p.add_rect(sk_rect, None);
    }

    fn add_round_rect(&mut self, rect: &RectF, radius: f64) {
        let sk_rect: skia_safe::Rect = rect.into();
        let radius = radius as f32;
        self.p.add_round_rect(sk_rect, (radius, radius), None);
    }

    fn add_circle(&mut self, center: PointF, radius: f64) {
        let radius = radius as f32;
        self.p.add_circle(center, radius, None);
    }

    fn add_ellipse(&mut self, rect: &RectF) {
        let sk_rect: skia_safe::Rect = rect.into();
        self.p.add_oval(sk_rect, None);
    }

    fn arc(&mut self, rect: &RectF, start_angle: f64, end_angle: f64) {
        let sk_rect: skia_safe::Rect = rect.into();
        let start_angle = start_angle as f32;
        let end_angle = end_angle as f32;
        self.p.arc_to(sk_rect, start_angle, end_angle, true);
    }

    fn arc_to(&mut self, p1: PointF, p2: PointF, radius: f64) {
        let radius = radius as f32;
        self.p.arc_to_tangent(p1, p2, radius);
    }

    fn cubic_to(&mut self, p1: PointF, p2: PointF, end_point: PointF) {
        self.p.cubic_to(p1, p2, end_point);
    }

    fn quad_to(&mut self, control_point: PointF, end_point: PointF) {
        self.p.quad_to(control_point, end_point);
    }
}
