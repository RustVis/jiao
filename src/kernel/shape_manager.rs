// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

use crate::event::{KeyEvent, MouseEvent, ResizeEvent};
use crate::painting::Painter;
use crate::shapes::AbstractShape;

pub struct ShapeManager {
    shapes: Vec<Box<dyn AbstractShape>>,
}

impl ShapeManager {
    pub fn new() -> Self {
        Self { shapes: Vec::new() }
    }

    pub fn add(&mut self, shape: Box<dyn AbstractShape>) {
        self.shapes.push(shape);
    }
}

impl ShapeManager {
    pub fn update(&mut self, painter: &mut Painter) {
        log::info!("ShapeManager::update()");
        for shape in self.shapes.iter_mut() {
            shape.update(painter);
        }
    }

    pub fn mouse_press_event(&mut self, _mouse_event: &MouseEvent) {}

    pub fn mouse_release_event(&mut self, _mouse_event: &MouseEvent) {}

    pub fn mouse_move_event(&mut self, _mouse_event: &MouseEvent) {}

    pub fn mouse_double_click_event(&mut self, _mouse_event: &MouseEvent) {}

    pub fn resize_event(&mut self, resize_event: &ResizeEvent) {}

    pub fn key_press_event(&mut self, key_event: &KeyEvent) {}
}