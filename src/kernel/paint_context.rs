// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

use web_sys::HtmlElement;

use super::paint_device::{CanvasPaintDevice, PaintDevice};
use super::shape_manager::ShapeManager;

pub struct PaintContext {
    shape_manager: ShapeManager,
    paint_device: Box<dyn PaintDevice>,
}

impl PaintContext {
    pub fn from_dom(dom: HtmlElement) -> Self {
        let shape_manager = ShapeManager::new();
        let paint_device = CanvasPaintDevice::new(dom);
        let paint_device = Box::new(paint_device);

        Self {
            shape_manager,
            paint_device,
        }
    }
}
