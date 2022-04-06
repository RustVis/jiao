// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

use js_sys::Object;
use web_sys::CanvasRenderingContext2d;

use crate::base::point::PointF;

pub struct Painter {
    ctx: CanvasRenderingContext2d,
}

impl Painter {
    pub fn new(obj: CanvasRenderingContext2d) -> Self {
        Self { ctx: obj }
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

    pub fn begin_path(&mut self) {
        self.ctx.begin_path();
    }

    pub fn close_path(&mut self) {
        self.ctx.close_path();
    }
}
