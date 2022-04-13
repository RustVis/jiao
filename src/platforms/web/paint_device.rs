// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

use wasm_bindgen::JsCast;
use web_sys::{
    CanvasRenderingContext2d, Document, Element, HtmlCanvasElement, HtmlElement, Window,
};

use super::painter::Painter;
use crate::base::Size;
use crate::kernel::PainterTrait;

pub trait PaintDeviceDelegate {
    fn on_repaint();
}

pub struct PaintDevice {
    canvas: HtmlCanvasElement,
    painter: Painter,
}

impl PaintDevice {
    pub fn new(dom: HtmlElement) -> Self {
        let window: Window = web_sys::window().unwrap();
        let document: Document = window.document().unwrap();
        let element: Element = document.create_element("canvas").unwrap();
        let canvas: HtmlCanvasElement = element.dyn_into::<HtmlCanvasElement>().unwrap();
        let canvas_ctx = canvas
            .get_context("2d")
            .unwrap()
            .unwrap()
            .dyn_into::<CanvasRenderingContext2d>()
            .unwrap();
        let painter = Painter::new(canvas.clone(), canvas_ctx);
        dom.append_child(&canvas).unwrap();
        Self { canvas, painter }
    }

    pub fn bind_event(&mut self) {}

    #[allow(dead_code)]
    fn on_repaint(&mut self) {
        unimplemented!()
    }

    pub fn size(&self) -> Size {
        Size::from(self.canvas.width() as i32, self.canvas.height() as i32)
    }

    pub fn painter(&mut self) -> &mut dyn PainterTrait {
        &mut self.painter
    }
}
