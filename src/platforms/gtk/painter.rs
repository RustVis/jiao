// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

use super::surface::SurfaceWrapper;
use crate::base::PointF;
use crate::kernel::{PainterTrait, PathTrait};

pub struct Painter {
    context: cairo::Context,
    surface: SurfaceWrapper,
}

impl Painter {
    pub fn new(surface: SurfaceWrapper) -> Self {
        let context = cairo::Context::new(&*surface).unwrap();
        Self { context, surface }
    }
}

impl PainterTrait for Painter {
    #[inline]
    fn save(&mut self) {
        self.context.save();
    }

    #[inline]
    fn restore(&mut self) {
        self.context.restore();
    }

    fn clear_all(&mut self) {
        todo!()
    }

    #[inline]
    fn clip(&mut self) {
        self.context.clip();
    }

    #[inline]
    fn fill(&mut self, _path: &Path) {
        self.context.fill();
    }

    #[inline]
    fn stroke(&mut self, _path: &Path) {
        self.context.stroke();
    }

    #[inline]
    fn rotate(&mut self, angle: f64) {
        self.context.rotate(angle);
    }

    #[inline]
    fn scale(&mut self, x: f64, y: f64) {
        self.context.scale(x, y);
    }

    #[inline]
    fn translate(&mut self, x: f64, y: f64) {
        self.context.translate(x, y);
    }
}

#[derive(Debug, Clone)]
pub struct Path {}

impl Path {
    pub fn new() -> Self {
        Self {}
    }
}

impl PathTrait for Path {
    #[inline]
    fn close(&mut self) {
        todo!()
    }

    #[inline]
    fn line_to(&mut self, point: PointF) {
        todo!()
    }

    #[inline]
    fn move_to(&mut self, _point: PointF) {
        todo!()
    }
}
