// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

#![allow(clippy::cast_possible_truncation)]

use skia_safe::PaintStyle;
use std::any::Any;
use std::cell::RefCell;
use std::rc::Rc;

use super::util::{to_sk_point, to_sk_rect};
use crate::base::{PointF, RectF};
use crate::kernel::{PainterTrait, PathTrait};

#[derive(Debug)]
enum CanvasWrapper {
    Surface(skia_safe::Surface),
    SvgCanvas(Rc<RefCell<skia_safe::svg::Canvas>>),
}

#[derive(Debug)]
pub struct Painter {
    canvas: CanvasWrapper,
    paint: skia_safe::Paint,
}

impl Painter {
    #[must_use]
    pub fn from_surface(surface: skia_safe::Surface) -> Self {
        let mut paint = skia_safe::Paint::default();
        paint.set_anti_alias(true);
        let canvas = CanvasWrapper::Surface(surface);
        Self { canvas, paint }
    }

    pub fn from_svg_canvas(canvas: Rc<RefCell<skia_safe::svg::Canvas>>) -> Self {
        let mut paint = skia_safe::Paint::default();
        paint.set_anti_alias(true);
        let canvas = CanvasWrapper::SvgCanvas(canvas);
        Self { canvas, paint }
    }

    fn canvas(&mut self) -> &mut skia_safe::Canvas {
        match &mut self.canvas {
            CanvasWrapper::Surface(surface) => surface.canvas(),
            CanvasWrapper::SvgCanvas(_svg_canvas) => todo!(),
        }
    }
}

impl PainterTrait for Painter {
    #[inline]
    fn save(&mut self) {
        self.canvas().save();
    }

    #[inline]
    fn restore(&mut self) {
        self.canvas().restore();
    }

    fn clear_all(&mut self) {
        log::info!("Painter::clear_all()");
        self.canvas().clear(skia_safe::Color::WHITE);
    }

    #[inline]
    fn clip(&mut self) {
        todo!()
    }

    #[inline]
    fn fill(&mut self, path: &dyn PathTrait) {
        let path_ref = path.as_any().downcast_ref::<Path>().unwrap();
        self.paint.set_style(PaintStyle::Fill);
        match &mut self.canvas {
            CanvasWrapper::Surface(surface) => {
                surface.canvas().draw_path(path_ref.path(), &self.paint);
            }
            CanvasWrapper::SvgCanvas(_svg_canvas) => todo!(),
        }
    }

    #[inline]
    fn stroke(&mut self, path: &dyn PathTrait) {
        let path_ref = path.as_any().downcast_ref::<Path>().unwrap();
        self.paint.set_style(PaintStyle::Stroke);
        match &mut self.canvas {
            CanvasWrapper::Surface(surface) => {
                surface.canvas().draw_path(path_ref.path(), &self.paint);
            }
            CanvasWrapper::SvgCanvas(_svg_canvas) => todo!(),
        }
    }

    #[inline]
    fn rotate(&mut self, angle: f64) {
        self.canvas().rotate(angle as f32, None);
    }

    #[inline]
    fn scale(&mut self, x: f64, y: f64) {
        self.canvas().scale((x as f32, y as f32));
    }

    #[inline]
    fn translate(&mut self, x: f64, y: f64) {
        self.canvas().translate((x as f32, y as f32));
    }
}

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
        self.p.move_to(to_sk_point(point));
    }

    #[inline]
    fn line_to(&mut self, point: PointF) {
        self.p.line_to(to_sk_point(point));
    }

    fn rect(&mut self, rect: &RectF) {
        let rect: skia_safe::Rect = to_sk_rect(rect);
        self.p.add_rect(rect, None);
    }

    fn cubic_to(&mut self, p1: PointF, p2: PointF, end_point: PointF) {
        self.p
            .cubic_to(to_sk_point(p1), to_sk_point(p2), to_sk_point(end_point));
    }

    fn quad_to(&mut self, control_point: PointF, end_point: PointF) {
        self.p
            .quad_to(to_sk_point(control_point), to_sk_point(end_point));
    }

    fn arc(&mut self, center: PointF, radius: f64, start_angle: f64, end_angle: f64) {
        let rect = RectF::from_circular(center, radius);
        let rect: skia_safe::Rect = to_sk_rect(&rect);
        self.p
            .arc_to(rect, start_angle as f32, end_angle as f32, true);
    }

    fn arc_to(&mut self, p1: PointF, p2: PointF, radius: f64) {
        self.p
            .arc_to_tangent(to_sk_point(p1), to_sk_point(p2), radius as f32);
    }

    fn ellipse(
        &mut self,
        _center: PointF,
        _radius_x: f64,
        _radius_y: f64,
        _start_angle: f64,
        _end_angle: f64,
    ) {
        todo!()
    }
}
