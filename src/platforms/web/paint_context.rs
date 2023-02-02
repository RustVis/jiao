// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

use web_sys::HtmlElement;

use super::paint_device::PaintDevice;
use crate::error::Error;
use crate::kernel::{PaintContextTrait, ShapeManager};

pub struct PaintContext {
    shape_manager: ShapeManager,
    paint_device: PaintDevice,
}

impl PaintContext {
    /// # Errors
    /// Returns error if failed to create paitn device.
    #[must_use]
    pub fn from_dom(dom: &HtmlElement) -> Result<Self, Error> {
        let shape_manager = ShapeManager::new();
        let paint_device = PaintDevice::new(dom)?;

        Ok(Self {
            shape_manager,
            paint_device,
        })
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
