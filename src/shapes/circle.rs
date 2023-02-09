// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

use super::Path2D;
use crate::base::{PointF, RectF};
use crate::kernel::{PainterTrait, PathTrait, ShapeTrait};

#[derive(Debug)]
pub struct CircleShape {
    center: PointF,
    radius: f64,

    path: Path2D,
    path_is_dirty: bool,
}

impl CircleShape {
    /// Create a new circle shape object.
    #[must_use]
    pub fn new(center: PointF, radius: f64) -> Self {
        debug_assert!(radius > 0.0);
        let path = Path2D::new();
        Self {
            center,
            radius,
            path,
            path_is_dirty: true,
        }
    }

    /// Get center point of the circle shape.
    #[must_use]
    pub const fn center(&self) -> PointF {
        self.center
    }

    /// Update center point of the circle shape.
    pub fn set_center(&mut self, center: PointF) {
        self.center = center;
        self.path_is_dirty = true;
    }

    /// Get radius of the circle shape.
    #[must_use]
    pub const fn radius(&self) -> f64 {
        self.radius
    }

    /// Set radius of the circle shape.
    pub fn set_radius(&mut self, radius: f64) {
        debug_assert!(radius > 0.0);
        self.radius = radius;
        self.path_is_dirty = true;
    }

    fn update_path(&mut self) {
        if !self.path_is_dirty {
            return;
        }
        self.path.clear();
        self.path.add_circle(self.center, self.radius);
        self.path_is_dirty = false;
    }
}

impl ShapeTrait for CircleShape {
    fn bounding_rect(&self) -> RectF {
        RectF::from_circle(self.center, self.radius)
    }

    fn repaint(&mut self, painter: &mut dyn PainterTrait) {
        self.update_path();
        painter.stroke(&self.path);
    }
}
