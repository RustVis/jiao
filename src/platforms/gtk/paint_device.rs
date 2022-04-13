// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

use super::painter::Painter;
use super::surface::SurfaceWrapper;
use crate::base::Size;

pub struct PaintDevice {
    surface: SurfaceWrapper,
    painter: Painter,
}

impl PaintDevice {
    pub fn new_image(width: i32, height: i32) -> Self {
        // TODO(Shaohua): Catch errors
        let surface = cairo::ImageSurface::create(cairo::Format::ARgb32, width, height).unwrap();
        let surface = SurfaceWrapper::Image(surface);
        let painter = Painter::new(surface.clone());
        Self { surface, painter }
    }

    pub fn get_size(&self) -> Size {
        match self.surface {
            SurfaceWrapper::Image(surface) => Size::from(surface.width(), surface.height()),
        }
    }

    pub fn get_painter(&mut self) -> &mut Painter {
        &mut self.painter
    }
}
