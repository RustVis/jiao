// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

use super::painter::Painter;
use crate::base::Size;

#[derive(Debug)]
pub enum PaintDevice {
    Image(ImagePaintDevice),
    Svg(SvgPaintDevice),
}

impl PaintDevice {
    pub fn painter(&mut self) -> &mut Painter {
        match self {
            Self::Image(image_device) => image_device.painter(),
            Self::Svg(svg_device) => svg_device.painter(),
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

    pub fn size(&self) -> Size {
        Size::from(self.surface.width(), self.surface.height())
    }

    pub fn painter(&mut self) -> &mut Painter {
        &mut self.painter
    }
}

#[derive(Debug)]
pub struct SvgPaintDevice {
    surface: cairo::SvgSurface,
    painter: Painter,
}

impl SvgPaintDevice {
    pub fn new<P: AsRef<std::path::Path>>(width: f64, height: f64, path: Option<P>) -> Self {
        // TODO(Shaohua): Catch errors
        let surface = cairo::SvgSurface::new(width, height, path).unwrap();
        let painter = Painter::new(&surface);
        Self { surface, painter }
    }

    pub fn size(&self) -> Size {
        todo!()
    }

    pub fn painter(&mut self) -> &mut Painter {
        &mut self.painter
    }

    pub fn surface(&mut self) -> &mut cairo::SvgSurface {
        &mut self.surface
    }
}
