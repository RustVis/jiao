// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};

use crate::base::PointF;
use crate::kernel::PainterTrait;

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
    fn save(&mut self) {
        self.ctx.save();
    }

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

    fn clip(&mut self) {
        todo!()
    }

    fn fill(&mut self) {
        todo!()
    }

    fn stroke(&mut self) {
        self.ctx.stroke();
    }

    fn rotate(&mut self, _angle: f64) {
        todo!()
    }

    fn scale(&mut self, _x: f64, _y: f64) {
        todo!()
    }

    fn translate(&mut self, _x: f64, _y: f64) {
        todo!()
    }

    fn begin_path(&mut self) {
        log::info!("Painter::begin_path()");
        self.ctx.begin_path();
    }

    fn close_path(&mut self) {
        self.ctx.close_path();
    }

    fn line_to(&mut self, point: PointF) {
        self.ctx.line_to(point.x(), point.y());
    }

    fn move_to(&mut self, point: PointF) {
        self.ctx.move_to(point.x(), point.y());
    }
}
