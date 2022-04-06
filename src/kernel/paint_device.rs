// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

use wasm_bindgen::JsCast;
use web_sys::{
    CanvasRenderingContext2d, Document, Element, HtmlCanvasElement, HtmlElement, Window,
};

use crate::base::Size;
use crate::painting::Painter;

pub trait PaintDeviceDelegate {
    fn on_repaint();
}

pub struct PaintDevice {
    canvas: HtmlCanvasElement,
    dom: HtmlElement,
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
        Self {
            canvas,
            dom,
            painter,
        }
    }

    pub fn bind_event(&mut self) {}

    fn on_repaint(&mut self) {
        unimplemented!()
    }

    pub fn get_size(&self) -> Size {
        Size::from(self.canvas.width() as i32, self.canvas.height() as i32)
    }

    pub fn get_painter(&mut self) -> &mut Painter {
        &mut self.painter
    }
}
