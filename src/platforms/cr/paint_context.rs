// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

use super::paint_device::PaintDevice;
use crate::kernel::{PaintContextTrait, PainterTrait, ShapeManager};

pub struct PaintContext {
    shape_manager: ShapeManager,
    paint_device: PaintDevice,
}

impl PaintContext {
    pub fn new(paint_device: PaintDevice) -> Self {
        let shape_manager = ShapeManager::new();
        Self {
            shape_manager,
            paint_device,
        }
    }

    pub fn start(&mut self) {
        log::info!("PaintContext::start()");
    }

    pub fn paint_device(&mut self) -> &mut PaintDevice {
        &mut self.paint_device
    }
}

impl PaintContextTrait for PaintContext {
    fn repaint(&mut self) {
        let painter = self.paint_device.painter();
        // TODO(Shaohua): Reset color.
        // painter.clear_all();
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
