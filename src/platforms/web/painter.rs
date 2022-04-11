// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement, Path2d};

use crate::base::PointF;
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

    fn rect_f64(&mut self, x: f64, y: f64, width: f64, height: f64) {
        self.path2d.rect(x, y, width, height);
    }

    fn cubic_to_f64(&mut self, cp1x: f64, cp1y: f64, cp2x: f64, cp2y: f64, x: f64, y: f64) {
        self.path2d.bezier_curve_to(cp1x, cp1y, cp2x, cp2y, x, y);
    }

    fn quad_to_f64(&mut self, cpx: f64, cpy: f64, x: f64, y: f64) {
        self.path2d.quadratic_curve_to(cpx, cpy, x, y);
    }

    fn arc_f64(&mut self, x: f64, y: f64, radius: f64, start_angle: f64, end_angle: f64) {
        // TODO(Shaohua): Returns error
        let _ = self.path2d.arc(x, y, radius, start_angle, end_angle);
    }

    fn arc_to_f64(&mut self, x1: f64, y1: f64, x2: f64, y2: f64, radius: f64) {
        // TODO(Shaohua): Returns error
        let _ = self.path2d.arc_to(x1, y1, x2, y2, radius);
    }

    fn ellipse_f64(
        &mut self,
        center_x: f64,
        center_y: f64,
        radius_x: f64,
        radius_y: f64,
        rotation: f64,
        start_angle: f64,
        end_angle: f64,
    ) {
        // TODO(Shaohua): Returns error
        let _ = self.path2d.ellipse(
            center_x,
            center_y,
            radius_x,
            radius_y,
            rotation,
            start_angle,
            end_angle,
        );
    }
}
