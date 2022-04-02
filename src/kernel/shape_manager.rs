// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

use crate::shapes::abstract_shape::AbstractShape;

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
