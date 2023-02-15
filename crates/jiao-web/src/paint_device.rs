// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

use jiao::base::Size;
use jiao::kernel::PainterTrait;
use wasm_bindgen::JsCast;
use web_sys::{
    CanvasRenderingContext2d, Document, Element, HtmlCanvasElement, HtmlElement, Window,
};

use crate::error::Error;
use crate::painter::Painter;

#[allow(clippy::module_name_repetitions)]
pub trait PaintDeviceDelegate {
    fn on_repaint();
}

pub struct PaintDevice {
    canvas: HtmlCanvasElement,
    painter: Painter,
}

impl PaintDevice {
    /// Create a new canvas dom as child of `parent_dom`.
    ///
    /// # Errors
    /// Returns error if got web-sys type conversion errors.
    #[allow(clippy::cast_sign_loss)]
    pub fn new(parent_dom: &HtmlElement, size: &Size) -> Result<Self, Error> {
        let window: Window =
            web_sys::window().ok_or_else(|| Error::new("No window object found"))?;
        let document: Document = window
            .document()
            .ok_or_else(|| Error::new("No document object found"))?;
        let element: Element = document.create_element("canvas")?;
        let canvas: HtmlCanvasElement = element.dyn_into::<HtmlCanvasElement>()?;
        canvas.set_width(size.width() as u32);
        canvas.set_height(size.height() as u32);
        let canvas_ctx = canvas
            .get_context("2d")?
            .ok_or_else(|| Error::new("No 2d context found"))?
            .dyn_into::<CanvasRenderingContext2d>()?;
        let painter = Painter::new(canvas.clone(), canvas_ctx);
        parent_dom.append_child(&canvas)?;
        Ok(Self { canvas, painter })
    }

    pub fn bind_event(&mut self) {}

    #[allow(dead_code)]
    fn on_repaint() {
        unimplemented!()
    }

    #[must_use]
    pub fn size(&self) -> Size {
        #[allow(clippy::cast_possible_wrap)]
        let width = self.canvas.width() as i32;
        #[allow(clippy::cast_possible_wrap)]
        let height = self.canvas.height() as i32;
        Size::from(width, height)
    }

    #[must_use]
    pub const fn canvas(&self) -> &HtmlCanvasElement {
        &self.canvas
    }

    pub fn painter(&mut self) -> &mut dyn PainterTrait {
        &mut self.painter
    }
}
