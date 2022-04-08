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
        self.ctx.fill_with_path_2d(path.path());
    }

    #[inline]
    fn stroke(&mut self, path: &Path) {
        self.ctx.stroke_with_path(path.path());
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
    p: Path2d,
}

impl Path {
    pub fn new() -> Self {
        // TODO(Shaohua): Add error type.
        let p = Path2d::new().unwrap();
        Self { p }
    }

    pub fn path(&self) -> &Path2d {
        &self.p
    }
}

impl PathTrait for Path {
    #[inline]
    fn add_path(&mut self, other: &Self) {
        self.path().add_path(other.path());
    }

    #[inline]
    fn close_path(&mut self) {
        self.p.close_path();
    }

    #[inline]
    fn move_to(&mut self, point: PointF) {
        self.p.move_to(point.x(), point.y());
    }

    #[inline]
    fn line_to(&mut self, point: PointF) {
        self.p.line_to(point.x(), point.y());
    }

    fn rect_f64(&mut self, x: f64, y: f64, width: f64, height: f64) {
        self.p.rect(x, y, width, height);
    }
}
