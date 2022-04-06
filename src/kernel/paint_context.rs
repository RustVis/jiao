// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

use web_sys::HtmlElement;

use super::paint_device::PaintDevice;
use super::shape_manager::ShapeManager;

pub struct PaintContext {
    shape_manager: ShapeManager,
    paint_device: PaintDevice,
}

impl PaintContext {
    pub fn from_dom(dom: HtmlElement) -> Self {
        let shape_manager = ShapeManager::new();
        let paint_device = PaintDevice::new(dom);

        Self {
            shape_manager,
            paint_device,
        }
    }

    pub fn start(&mut self) {
        log::info!("PaintContext::start()");
    }

    pub fn update(&mut self) {
        log::info!("PaintContext::update()");
        let painter = self.paint_device.get_painter();
        self.shape_manager.update(painter);
    }

    pub fn shape_manager(&mut self) -> &mut ShapeManager {
        &mut self.shape_manager
    }
}
