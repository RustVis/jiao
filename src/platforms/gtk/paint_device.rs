// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

use super::painter::Painter;
use crate::base::Size;

#[derive(Debug)]
pub enum PaintDevice {
    Image(ImagePaintDevice),
}

impl PaintDevice {
    pub fn get_painter(&mut self) -> &mut Painter {
        match self {
            Self::Image(image_device) => image_device.get_painter(),
        }
    }
}

#[derive(Debug)]
pub struct ImagePaintDevice {
    surface: cairo::ImageSurface,
    painter: Painter,
}

impl ImagePaintDevice {
    pub fn new(format: cairo::Format, width: i32, height: i32) -> Self {
        // TODO(Shaohua): Catch errors
        let surface = cairo::ImageSurface::create(format, width, height).unwrap();
        let painter = Painter::new(&surface);
        Self { surface, painter }
    }

    pub fn get_size(&self) -> Size {
        Size::from(self.surface.width(), self.surface.height())
    }

    pub fn get_painter(&mut self) -> &mut Painter {
        &mut self.painter
    }
}
