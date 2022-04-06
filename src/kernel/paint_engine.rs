// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

use super::paint_device::PaintDevice;
use super::shape_manager::ShapeManager;

#[derive(Debug, Clone)]
pub struct PaintEngine {}

impl PaintEngine {
    pub fn new() -> Self {
        Self {}
    }

    pub fn update(&self, shape_manager: &mut ShapeManager) {
        log::info!("PaintEngine::update()");
    }
}
