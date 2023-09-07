// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that can be found
// in the LICENSE file.

use cpp_core::{CastInto, Ptr};
use qt_gui::QPaintDevice;

use super::Painter;
use crate::{PaintContextTrait, ShapeManager};

pub struct PaintContext {
    shape_manager: ShapeManager,
}

impl Default for PaintContext {
    fn default() -> Self {
        Self::new()
    }
}

impl PaintContext {
    #[must_use]
    pub fn new() -> Self {
        let shape_manager = ShapeManager::new();
        Self { shape_manager }
    }

    pub fn start(&mut self, paint_device: impl CastInto<Ptr<QPaintDevice>>) {
        log::info!("PaintContext::start()");
        let mut painter = Painter::new();
        unsafe {
            painter.painter().begin(paint_device);
            painter.set_default_hints();
        }
        self.shape_manager.update(&mut painter);
        unsafe {
            painter.painter().end();
        }
    }
}

impl PaintContextTrait for PaintContext {
    fn repaint(&mut self) {
        //self.shape_manager.update(painter);
    }

    fn update(&mut self) {
        log::info!("PaintContext::update()");
        self.repaint();
    }

    fn shape_manager(&mut self) -> &mut ShapeManager {
        &mut self.shape_manager
    }
}
