// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

#![allow(clippy::module_name_repetitions)]

use super::painter::Painter;
use crate::base::Size;

#[derive(Debug, Clone)]
pub enum PaintDevice {
    Image(ImagePaintDevice),
    Pdf(PdfPaintDevice),
    Svg(SvgPaintDevice),
}

impl PaintDevice {
    pub fn painter(&mut self) -> &mut Painter {
        match self {
            Self::Image(image_device) => image_device.painter(),
            Self::Pdf(pdf_device) => pdf_device.painter(),
            Self::Svg(svg_device) => svg_device.painter(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct ImagePaintDevice {
    surface: cairo::ImageSurface,
    painter: Painter,
}

impl ImagePaintDevice {
    /// Create a new image painting device.
    ///
    /// # Panics
    /// Got panic if failedto create image painting device with specific format.
    #[must_use]
    pub fn new(format: cairo::Format, width: i32, height: i32) -> Self {
        // TODO(Shaohua): Catch errors
        let surface = cairo::ImageSurface::create(format, width, height).unwrap();
        let painter = Painter::new(&surface);
        Self { surface, painter }
    }

    #[must_use]
    pub fn size(&self) -> Size {
        Size::from(self.surface.width(), self.surface.height())
    }

    pub fn painter(&mut self) -> &mut Painter {
        &mut self.painter
    }

    pub fn surface(&mut self) -> &mut cairo::ImageSurface {
        &mut self.surface
    }
}

#[derive(Debug, Clone)]
pub struct PdfPaintDevice {
    surface: cairo::PdfSurface,
    painter: Painter,
}

impl PdfPaintDevice {
    /// Create a new pdf paint device with specific size.
    ///
    /// # Panics
    /// Got panic if failed to create new pdf paint device.
    pub fn new<P: AsRef<std::path::Path>>(width: f64, height: f64, path: P) -> Self {
        // TODO(Shaohua): Catch errors
        let surface = cairo::PdfSurface::new(width, height, path).unwrap();
        let painter = Painter::new(&surface);
        Self { surface, painter }
    }

    #[must_use]
    pub fn size(&self) -> Size {
        unimplemented!()
    }

    pub fn painter(&mut self) -> &mut Painter {
        &mut self.painter
    }

    pub fn surface(&mut self) -> &mut cairo::PdfSurface {
        &mut self.surface
    }
}

#[derive(Debug, Clone)]
pub struct SvgPaintDevice {
    surface: cairo::SvgSurface,
    painter: Painter,
}

impl SvgPaintDevice {
    /// Create a new svg painting device with specific size.
    ///
    /// # Panics
    /// Got panic if failed to create svg surface.
    pub fn new<P: AsRef<std::path::Path>>(width: f64, height: f64, path: P) -> Self {
        // TODO(Shaohua): Catch errors
        let surface = cairo::SvgSurface::new(width, height, Some(path)).unwrap();
        let painter = Painter::new(&surface);
        Self { surface, painter }
    }

    #[must_use]
    pub fn size(&self) -> Size {
        unimplemented!()
    }

    pub fn painter(&mut self) -> &mut Painter {
        &mut self.painter
    }

    pub fn surface(&mut self) -> &mut cairo::SvgSurface {
        &mut self.surface
    }
}
