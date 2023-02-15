// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};

use super::path::Path;
use crate::base::PointF;
use crate::kernel::{PainterTrait, PathTrait};

pub struct Painter {
    canvas: HtmlCanvasElement,
    ctx: CanvasRenderingContext2d,
}

impl Painter {
    #[must_use]
    pub const fn new(canvas: HtmlCanvasElement, ctx: CanvasRenderingContext2d) -> Self {
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
            f64::from(self.canvas.width()),
            f64::from(self.canvas.height()),
        );
    }

    #[inline]
    fn clip(&mut self) {
        todo!()
    }

    #[inline]
    fn fill(&mut self, path: &dyn PathTrait) {
        let path_ref = path.as_any().downcast_ref::<Path>().unwrap();
        self.ctx.fill_with_path_2d(path_ref.path2d());
    }

    #[inline]
    fn stroke(&mut self, path: &dyn PathTrait) {
        let path_ref = path.as_any().downcast_ref::<Path>().unwrap();
        self.ctx.stroke_with_path(path_ref.path2d());
    }

    #[inline]
    fn rotate(&mut self, angle: f64) {
        // TODO(Shaohua): Returns error
        let _ret = self.ctx.rotate(angle);
    }

    #[inline]
    fn scale(&mut self, sx: f64, sy: f64) {
        // TODO(Shaohua): Returns error
        let _ret = self.ctx.scale(sx, sy);
    }

    #[inline]
    fn translate(&mut self, point: PointF) {
        // TODO(Shaohua): Returns error
        let _ret = self.ctx.translate(point.x(), point.y());
    }

    #[inline]
    fn draw_text(&mut self, text: &str, position: PointF) {
        // TODO(Shaohua): Returns error
        let _ret = self.ctx.fill_text(text, position.x(), position.y());
    }
}