// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

use skia_safe::PaintStyle;

use crate::base::{PointF, RectF};
use crate::kernel::{PainterTrait, PathTrait};

pub struct Painter {
    paint: skia_safe::Paint,
    surface: skia_safe::Surface,
}

impl Painter {
    pub fn new(surface: skia_safe::Surface) -> Self {
        let mut paint = skia_safe::Paint::default();
        paint.set_anti_alias(true);

        Self { paint, surface }
    }

    fn canvas(&mut self) -> &mut skia_safe::Canvas {
        self.surface.canvas()
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
    fn fill(&mut self, path: &Path) {
        self.paint.set_style(PaintStyle::Fill);
        self.surface.canvas().draw_path(path.path(), &self.paint);
    }

    #[inline]
    fn stroke(&mut self, path: &Path) {
        self.paint.set_style(PaintStyle::Stroke);
        self.surface.canvas().draw_path(path.path(), &self.paint);
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
    pub fn new() -> Self {
        let p = skia_safe::Path::new();
        Self { p }
    }

    pub fn path(&self) -> &skia_safe::Path {
        &self.p
    }
}

impl PathTrait for Path {
    fn clear(&mut self) {
        // TODO(Shaohua):
    }

    fn add_path(&mut self, other: &Self) {
        self.p.add_path(&other.p, (0.0, 0.0), None);
    }

    fn close_path(&mut self) {
        self.p.close();
    }

    #[inline]
    fn move_to(&mut self, point: PointF) {
        self.p.move_to((point.x() as f32, point.y() as f32));
    }

    #[inline]
    fn line_to(&mut self, point: PointF) {
        self.p.line_to((point.x() as f32, point.y() as f32));
    }

    fn rect_f64(&mut self, x: f64, y: f64, width: f64, height: f64) {
        let rect = RectF::from(x, y, width, height);
        self.p.add_rect(
            skia_safe::Rect::new(
                rect.left() as f32,
                rect.top() as f32,
                rect.right() as f32,
                rect.bottom() as f32,
            ),
            None,
        );
    }

    fn cubic_to_f64(&mut self, cp1x: f64, cp1y: f64, cp2x: f64, cp2y: f64, x: f64, y: f64) {
        self.p.cubic_to(
            (cp1x as f32, cp1y as f32),
            (cp2x as f32, cp2y as f32),
            (x as f32, y as f32),
        );
    }

    fn quad_to_f64(&mut self, cpx: f64, cpy: f64, x: f64, y: f64) {
        self.p
            .quad_to((cpx as f32, cpy as f32), (x as f32, y as f32));
    }

    fn arc_f64(&mut self, x: f64, y: f64, radius: f64, start_angle: f64, end_angle: f64) {
        let mut rect = RectF::new();
        rect.move_center(&PointF::from(x, y));
        rect.set_width(radius * 2.0);
        rect.set_height(radius * 2.0);
        self.p.arc_to(
            skia_safe::Rect::new(
                rect.left() as f32,
                rect.top() as f32,
                rect.right() as f32,
                rect.bottom() as f32,
            ),
            start_angle as f32,
            end_angle as f32,
            true,
        );
    }

    fn arc_to_f64(&mut self, x1: f64, y1: f64, x2: f64, y2: f64, radius: f64) {
        self.p.arc_to_tangent(
            (x1 as f32, y1 as f32),
            (x2 as f32, y2 as f32),
            radius as f32,
        );
    }

    fn ellipse_f64(
        &mut self,
        _center_x: f64,
        _center_y: f64,
        _radius_x: f64,
        _radius_y: f64,
        _rotation: f64,
        _start_angle: f64,
        _end_angle: f64,
    ) {
        todo!()
    }
}
