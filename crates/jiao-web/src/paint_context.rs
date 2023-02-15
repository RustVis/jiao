// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

use jiao::base::Size;
use jiao::kernel::{PaintContextTrait, ShapeManager};
use web_sys::HtmlElement;

use crate::error::Error;
use crate::paint_device::PaintDevice;

pub struct PaintContext {
    shape_manager: ShapeManager,
    paint_device: PaintDevice,
}

impl PaintContext {
    /// Create an html canvas element from parent `dom` with specific `size`.
    ///
    /// # Errors
    /// Returns error if failed to create paint device.
    pub fn from_dom(dom: &HtmlElement, size: &Size) -> Result<Self, Error> {
        let shape_manager = ShapeManager::new();
        let paint_device = PaintDevice::new(dom, size)?;

        Ok(Self {
            shape_manager,
            paint_device,
        })
    }

    pub fn paint_device(&self) -> &PaintDevice {
        &self.paint_device
    }

    pub fn start(&mut self) {
        log::info!("PaintContext::start()");
    }
}

impl PaintContextTrait for PaintContext {
    fn repaint(&mut self) {
        let painter = self.paint_device.painter();
        painter.clear_all();
        self.shape_manager.update(painter);
    }

    fn update(&mut self) {
        log::info!("PaintContext::update()");
        self.repaint();
    }

    fn shape_manager(&mut self) -> &mut ShapeManager {
        &mut self.shape_manager
    }
}
