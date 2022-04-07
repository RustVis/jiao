// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

use skia_safe::Surface;

use super::painter::Painter;
use crate::base::Size;

pub struct PaintDevice {
    surface: Surface,
    painter: Painter,
}

impl PaintDevice {
    pub fn new(width: i32, height: i32) -> Self {
        let surface = Surface::new_raster_n32_premul((width, height)).expect("no surface!");
        let painter = Painter::new(surface.clone());
        Self { surface, painter }
    }

    pub fn get_size(&self) -> Size {
        Size::from(self.surface.width(), self.surface.height())
    }

    pub fn get_painter(&mut self) -> &mut Painter {
        &mut self.painter
    }
}
