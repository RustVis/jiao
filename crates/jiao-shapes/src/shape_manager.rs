// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that can be found
// in the LICENSE file.

use jiao::kernel::PainterTrait;

use crate::ShapeTrait;

#[derive(Debug)]
pub struct ShapeManager {
    shapes: Vec<Box<dyn ShapeTrait>>,
}

impl Default for ShapeManager {
    fn default() -> Self {
        Self::new()
    }
}

impl ShapeManager {
    #[must_use]
    pub fn new() -> Self {
        Self { shapes: Vec::new() }
    }

    pub fn add(&mut self, shape: Box<dyn ShapeTrait>) {
        self.shapes.push(shape);
    }
}

impl ShapeManager {
    pub fn update(&mut self, painter: &mut dyn PainterTrait) {
        for shape in &mut self.shapes {
            painter.save();
            shape.repaint(painter);
            painter.restore();
        }
    }
}
