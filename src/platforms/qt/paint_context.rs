// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

use cpp_core::{CastInto, Ptr};
use qt_gui::QPaintDevice;

use super::painter::Painter;
use crate::kernel::{PaintContextTrait, PainterTrait, ShapeManager};

pub struct PaintContext {
    shape_manager: ShapeManager,
    painter: Painter,
}

impl PaintContext {
    pub fn new() -> Self {
        let shape_manager = ShapeManager::new();
        let painter = Painter::new();
        Self {
            shape_manager,
            painter,
        }
    }

    pub fn start(&mut self, paint_device: impl CastInto<Ptr<QPaintDevice>>) {
        log::info!("PaintContext::start()");
        unsafe {
            self.painter.painter().begin(paint_device);
        }
        unsafe {
            self.painter.painter().end();
        }
    }
}

impl PaintContextTrait for PaintContext {
    fn repaint(&mut self) {
        todo!()
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
