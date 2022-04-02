// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

use wasm_bindgen::JsCast;
use web_sys::{Document, Element, HtmlCanvasElement, HtmlElement, Window};

pub trait PaintDeviceDelegate {
    fn on_repaint();
}

pub trait PaintDevice {}

pub struct CanvasPaintDevice {
    canvas: HtmlCanvasElement,
    dom: HtmlElement,
}

impl CanvasPaintDevice {
    pub fn new(dom: HtmlElement) -> Self {
        let window: Window = web_sys::window().unwrap();
        let document: Document = window.document().unwrap();
        let element: Element = document.create_element("canvas").unwrap();
        let canvas: HtmlCanvasElement = element.dyn_into::<HtmlCanvasElement>().unwrap();
        dom.append_child(&canvas).unwrap();
        Self { canvas, dom }
    }

    pub fn bind_event(&mut self) {}

    fn on_repaint(&mut self) {
        //
    }
}

impl PaintDevice for CanvasPaintDevice {}
