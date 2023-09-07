// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by LGPL-3.0 License that can be found
// in the LICENSE file.

use super::paint_device::PaintDevice;
use crate::PaintContextTrait;

pub struct PaintContext {
    shape_manager: ShapeManager,
    paint_device: PaintDevice,
}

impl PaintContext {
    #[must_use]
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
        let _ret = painter.context.save();
        painter.context.set_antialias(cairo::Antialias::Best);

        self.shape_manager.update(painter);

        let _ret = painter.context.restore();
    }

    fn update(&mut self) {
        log::info!("PaintContext::update()");
        self.repaint();
    }

    fn shape_manager(&mut self) -> &mut ShapeManager {
        &mut self.shape_manager
    }
}
