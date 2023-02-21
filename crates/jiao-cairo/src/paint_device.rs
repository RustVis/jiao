// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

#![allow(clippy::module_name_repetitions)]

use jiao::base::Size;

use crate::error::Error;
use crate::painter::Painter;

#[derive(Debug, Clone)]
pub enum PaintDevice {
    Image(ImagePaintDevice),
    Pdf(PdfPaintDevice),
    Svg(SvgPaintDevice),
}

impl PaintDevice {
    pub fn context(&mut self) -> &cairo::Context {
        self.painter().context()
    }

    pub fn painter(&self) -> &Painter {
        match self {
            Self::Image(image_device) => image_device.painter(),
            Self::Pdf(pdf_device) => pdf_device.painter(),
            Self::Svg(svg_device) => svg_device.painter(),
        }
    }

    pub fn painter_mut(&mut self) -> &mut Painter {
        match self {
            Self::Image(image_device) => image_device.painter_mut(),
            Self::Pdf(pdf_device) => pdf_device.painter_mut(),
            Self::Svg(svg_device) => svg_device.painter_mut(),
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
    /// # Errors
    /// Returns error if failed to create image painting device with specific format.
    pub fn new(format: cairo::Format, width: i32, height: i32) -> Result<Self, Error> {
        let surface = cairo::ImageSurface::create(format, width, height)?;
        let context = cairo::Context::new(&surface)?;
        let painter = Painter::new(context);
        Ok(Self { surface, painter })
    }

    #[must_use]
    pub fn size(&self) -> Size {
        Size::from(self.surface.width(), self.surface.height())
    }

    pub fn surface_mut(&mut self) -> &mut cairo::ImageSurface {
        &mut self.surface
    }

    pub fn painter(&self) -> &Painter {
        &self.painter
    }

    pub fn painter_mut(&mut self) -> &mut Painter {
        &mut self.painter
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
    /// # Errors
    /// Returns error if failed to create new pdf paint device.
    pub fn new<P: AsRef<std::path::Path>>(width: f64, height: f64, path: P) -> Result<Self, Error> {
        let surface = cairo::PdfSurface::new(width, height, path)?;
        let context = cairo::Context::new(&surface)?;
        let painter = Painter::new(context);
        Ok(Self { surface, painter })
    }

    #[must_use]
    pub fn size(&self) -> Size {
        unimplemented!()
    }

    pub fn surface_mut(&mut self) -> &mut cairo::PdfSurface {
        &mut self.surface
    }

    pub fn painter(&self) -> &Painter {
        &self.painter
    }

    pub fn painter_mut(&mut self) -> &mut Painter {
        &mut self.painter
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
    /// # Errors
    /// Returns error if failed to create svg surface.
    pub fn new<P: AsRef<std::path::Path>>(width: f64, height: f64, path: P) -> Result<Self, Error> {
        let surface = cairo::SvgSurface::new(width, height, Some(path))?;
        let context = cairo::Context::new(&surface)?;
        let painter = Painter::new(context);
        Ok(Self { surface, painter })
    }

    #[must_use]
    pub fn size(&self) -> Size {
        unimplemented!()
    }

    pub fn surface_mut(&mut self) -> &mut cairo::SvgSurface {
        &mut self.surface
    }

    pub fn painter(&self) -> &Painter {
        &self.painter
    }

    pub fn painter_mut(&mut self) -> &mut Painter {
        &mut self.painter
    }
}
