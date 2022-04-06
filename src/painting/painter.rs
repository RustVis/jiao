// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

use js_sys::Object;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};

use crate::base::point::PointF;

pub struct Painter {
    canvas: HtmlCanvasElement,
    ctx: CanvasRenderingContext2d,
}

impl Painter {
    pub fn new(canvas: HtmlCanvasElement, ctx: CanvasRenderingContext2d) -> Self {
        Self { canvas, ctx }
    }
}

impl Painter {
    pub fn move_to(&mut self, point: PointF) {
        self.ctx.move_to(point.x(), point.y());
    }

    pub fn line_to(&mut self, point: PointF) {
        self.ctx.line_to(point.x(), point.y());
    }

    pub fn stroke(&mut self) {
        self.ctx.stroke();
    }

    pub fn clear_all(&mut self) {
        log::info!("Painter::clear_all()");
        self.ctx.clear_rect(
            0.0,
            0.0,
            self.canvas.width() as f64,
            self.canvas.height() as f64,
        );
    }

    pub fn begin_path(&mut self) {
        log::info!("Painter::begin_path()");
        self.ctx.begin_path();
    }

    pub fn save(&mut self) {
        self.ctx.save();
    }

    pub fn restore(&mut self) {
        self.ctx.restore();
    }

    pub fn close_path(&mut self) {
        self.ctx.close_path();
    }
}
