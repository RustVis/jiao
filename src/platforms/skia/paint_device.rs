// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

#![allow(clippy::module_name_repetitions)]

use skia_safe::svg::Canvas;
use std::cell::{RefCell, RefMut};
use std::rc::Rc;

use super::util::to_sk_rect;
use crate::base::{RectF, Size};
use crate::painter::Painter;

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
    surface: skia_safe::Surface,
    painter: Painter,
}

impl ImagePaintDevice {
    /// Create a new image paint device.
    #[must_use]
    pub fn new(width: i32, height: i32) -> Self {
        // TODO(Shaohua): Catch errors
        let surface =
            skia_safe::Surface::new_raster_n32_premul((width, height)).expect("no surface!");
        let painter = Painter::from_surface(surface.clone());
        Self { surface, painter }
    }

    #[must_use]
    pub fn size(&self) -> Size {
        Size::from(self.surface.width(), self.surface.height())
    }

    pub fn painter(&mut self) -> &mut Painter {
        &mut self.painter
    }

    /// Encode current surface state to specific image format data.
    ///
    /// # Panics
    /// Got panic if image encoding failed.
    pub fn encode(&mut self, format: skia_safe::EncodedImageFormat) -> skia_safe::Data {
        let image = self.surface.image_snapshot();
        // TODO(Shaohua): Returns Result<>
        image.encode_to_data(format).unwrap()
    }
}

#[derive(Debug)]
pub struct SvgPaintDevice {
    canvas: Rc<RefCell<skia_safe::svg::Canvas>>,
    painter: Painter,
    rect: RectF,
}

impl SvgPaintDevice {
    /// Create a new image paint device.
    #[must_use]
    pub fn new(rect: &RectF) -> Self {
        // TODO(Shaohua): Catch errors
        let sk_rect = to_sk_rect(rect);
        let canvas = skia_safe::svg::Canvas::new(sk_rect, None);
        let canvas = Rc::new(RefCell::new(canvas));
        let painter = Painter::from_svg_canvas(canvas.clone());
        Self {
            canvas,
            painter,
            rect: rect.clone(),
        }
    }

    #[must_use]
    pub fn size(&self) -> Size {
        // TODO(Shaohua): Get size from canvas surface
        //Size::from(self.canvas.width(), self.canvas.height())
        self.rect.size().to_size()
    }

    pub fn painter(&mut self) -> &mut Painter {
        &mut self.painter
    }

    pub fn canvas(&mut self) -> RefMut<'_, Canvas> {
        self.canvas.borrow_mut()
    }
}
