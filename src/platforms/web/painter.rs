// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement, Path2d};

use crate::base::{PointF, RectF};
use crate::kernel::{PainterTrait, PathTrait};

pub struct Painter {
    canvas: HtmlCanvasElement,
    ctx: CanvasRenderingContext2d,
}

impl Painter {
    pub fn new(canvas: HtmlCanvasElement, ctx: CanvasRenderingContext2d) -> Self {
        Self { canvas, ctx }
    }
}

impl PainterTrait for Painter {
    #[inline]
    fn save(&mut self) {
        self.ctx.save();
    }

    #[inline]
    fn restore(&mut self) {
        self.ctx.restore();
    }

    fn clear_all(&mut self) {
        log::info!("Painter::clear_all()");
        self.ctx.clear_rect(
            0.0,
            0.0,
            self.canvas.width() as f64,
            self.canvas.height() as f64,
        );
    }

    #[inline]
    fn clip(&mut self) {
        todo!()
    }

    #[inline]
    fn fill(&mut self, path: &Path) {
        self.ctx.fill_with_path_2d(path.path2d());
    }

    #[inline]
    fn stroke(&mut self, path: &Path) {
        self.ctx.stroke_with_path(path.path2d());
    }

    #[inline]
    fn rotate(&mut self, angle: f64) {
        // TODO(Shaohua): Returns error
        let _ = self.ctx.rotate(angle);
    }

    #[inline]
    fn scale(&mut self, x: f64, y: f64) {
        // TODO(Shaohua): Returns error
        let _ = self.ctx.scale(x, y);
    }

    #[inline]
    fn translate(&mut self, x: f64, y: f64) {
        // TODO(Shaohua): Returns error
        let _ = self.ctx.translate(x, y);
    }
}

#[derive(Debug, Clone)]
pub struct Path {
    path2d: Path2d,
}

impl Path {
    pub fn new() -> Self {
        // TODO(Shaohua): Add error type.
        let path2d = Path2d::new().unwrap();
        Self { path2d }
    }

    /// Get inner Path2D object.
    pub fn path2d(&self) -> &Path2d {
        &self.path2d
    }
}

impl PathTrait for Path {
    fn clear(&mut self) {
        self.path2d = Path2d::new().unwrap();
    }

    #[inline]
    fn add_path(&mut self, other: &Self) {
        self.path2d().add_path(other.path2d());
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
        let _ = self
            .path2d
            .arc(center.x(), center.y(), radius, start_angle, end_angle);
    }

    fn arc_to(&mut self, p1: PointF, p2: PointF, radius: f64) {
        // TODO(Shaohua): Returns error
        let _ = self.path2d.arc_to(p1.x(), p1.y(), p2.x(), p2.y(), radius);
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
        let _ = self.path2d.ellipse(
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
