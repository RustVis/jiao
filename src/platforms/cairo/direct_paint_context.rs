// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License
// that can be found in the LICENSE file.

use super::direct_painter::DirectPainter;
use crate::kernel::{PaintContextTrait, ShapeManager};

pub struct DirectPaintContext<'a> {
    shape_manager: ShapeManager,
    ctx: Option<&'a cairo::Context>,
}

impl<'a> DirectPaintContext<'a> {
    pub fn new() -> Self {
        let shape_manager = ShapeManager::new();
        Self {
            shape_manager,
            ctx: None,
        }
    }

    pub fn set_cairo_context(&mut self, ctx: &'a cairo::Context) {
        self.ctx = Some(ctx);
    }
}

impl<'a> PaintContextTrait for DirectPaintContext<'a> {
    fn repaint(&mut self) {
        if let Some(ctx) = self.ctx {
            let mut painter = DirectPainter::new(ctx);
            self.shape_manager.update(&mut painter);
        } else {
            // TODO(Shaohua): Returns error
        }
    }

    fn update(&mut self) {
        log::info!("PaintContext::update()");
        self.repaint();
    }

    fn shape_manager(&mut self) -> &mut ShapeManager {
        &mut self.shape_manager
    }
}
