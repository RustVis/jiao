// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

use super::surface::SurfaceWrapper;
use crate::kernel::generic_path::GenericPath;
use crate::kernel::PainterTrait;

// Re-export GenericPath as Path
pub type Path = GenericPath;

pub struct Painter {
    context: cairo::Context,
}

impl Painter {
    pub fn new(surface: &SurfaceWrapper) -> Self {
        let context = cairo::Context::new(&*surface).unwrap();
        Self { context }
    }
}

impl PainterTrait for Painter {
    #[inline]
    fn save(&mut self) {
        // TODO(Shaohua): Catch errors
        let _ = self.context.save();
    }

    #[inline]
    fn restore(&mut self) {
        // TODO(Shaohua): Catch errors
        let _ = self.context.restore();
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
        // TODO(Shaohua): Catch errors
        let _ = self.context.fill();
    }

    #[inline]
    fn stroke(&mut self, _path: &Path) {
        // TODO(Shaohua): catch errors
        let _ = self.context.stroke();
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
