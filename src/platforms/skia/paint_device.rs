// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

use skia_safe::Surface;

use super::painter::Painter;
use crate::base::Size;

#[derive(Debug, Clone)]
pub enum PaintDevice {
    Image(ImagePaintDevice),
}

impl PaintDevice {
    pub fn painter(&mut self) -> &mut Painter {
        match self {
            Self::Image(image_device) => image_device.painter(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct ImagePaintDevice {
    surface: Surface,
    painter: Painter,
}

impl ImagePaintDevice {
    /// Create a new image paint device.
    pub fn new(width: i32, height: i32) -> Self {
        // TODO(Shaohua): Catch errors
        let surface = Surface::new_raster_n32_premul((width, height)).expect("no surface!");
        let painter = Painter::new(surface.clone());
        Self { surface, painter }
    }

    pub fn size(&self) -> Size {
        Size::from(self.surface.width(), self.surface.height())
    }

    pub fn painter(&mut self) -> &mut Painter {
        &mut self.painter
    }

    /// Encode current surface state to specific image format data.
    pub fn encode(&mut self, format: skia_safe::EncodedImageFormat) -> skia_safe::Data {
        let image = self.surface.image_snapshot();
        image.encode_to_data(format).unwrap()
    }
}
