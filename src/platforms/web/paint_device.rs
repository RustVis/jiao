// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

use wasm_bindgen::JsCast;
use web_sys::{
    CanvasRenderingContext2d, Document, Element, HtmlCanvasElement, HtmlElement, Window,
};

use super::painter::Painter;
use crate::base::Size;
use crate::error::{Error, ErrorKind};
use crate::kernel::PainterTrait;

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
    #[must_use]
    pub fn new(parent_dom: &HtmlElement) -> Result<Self, Error> {
        let window: Window =
            web_sys::window().ok_or(Error::new(ErrorKind::JsError, "No window object found"))?;
        let document: Document = window
            .document()
            .ok_or(Error::new(ErrorKind::JsError, "No document object found"))?;
        let element: Element = document.create_element("canvas")?;
        let canvas: HtmlCanvasElement = element.dyn_into::<HtmlCanvasElement>()?;
        let canvas_ctx = canvas
            .get_context("2d")?
            .ok_or(Error::new(ErrorKind::JsError, "No 2d context found"))?
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

    pub fn painter(&mut self) -> &mut dyn PainterTrait {
        &mut self.painter
    }
}
