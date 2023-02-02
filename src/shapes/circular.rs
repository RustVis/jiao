// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

use super::Path2D;
use crate::base::{PointF, RectF};
use crate::kernel::{PainterTrait, PathTrait, ShapeTrait};

const DEFAULT_END_ANGLE: f64 = std::f64::consts::TAU;

#[derive(Debug)]
pub struct CircularShape {
    center: PointF,
    radius: f64,
    start_angle: f64,
    end_angle: f64,

    path: Path2D,
    path_is_dirty: bool,
}

impl CircularShape {
    /// Create a new circular shape object.
    ///
    /// # Panics
    ///
    /// `radius` shall be non-negative.
    #[must_use]
    pub fn new(center: PointF, radius: f64) -> Self {
        assert!(radius >= 0.0);
        let path = Path2D::new();
        Self {
            center,
            radius,
            start_angle: 0.0,
            end_angle: DEFAULT_END_ANGLE,
            path,
            path_is_dirty: true,
        }
    }

    /// Get center point of the circular shape.
    #[must_use]
    pub const fn center(&self) -> PointF {
        self.center
    }

    /// Update center point of the circular shape.
    pub fn set_center(&mut self, center: PointF) {
        self.center = center;
        self.path_is_dirty = true;
    }

    /// Get radius of the circular shape.
    #[must_use]
    pub const fn radius(&self) -> f64 {
        self.radius
    }

    /// Set radius of the circular shape.
    ///
    /// # Panics
    ///
    /// `radius` shall be non-negative.
    pub fn set_radius(&mut self, radius: f64) {
        assert!(radius >= 0.0);
        self.radius = radius;
        self.path_is_dirty = true;
    }

    /// Get start angle of the circular shape.
    #[must_use]
    pub const fn start_angle(&self) -> f64 {
        self.start_angle
    }

    /// Set start angle of the circular shape.
    pub fn set_start_angle(&mut self, start_angle: f64) {
        self.start_angle = start_angle;
        self.path_is_dirty = true;
    }

    /// Get end angle of the circular shape.
    #[must_use]
    pub const fn end_angle(&self) -> f64 {
        self.end_angle
    }

    /// Set end angle of the circular shape.
    pub fn set_end_angle(&mut self, end_angle: f64) {
        self.end_angle = end_angle;
        self.path_is_dirty = true;
    }

    fn update_path(&mut self) {
        if !self.path_is_dirty {
            return;
        }
        self.path.clear();
        self.path
            .arc(self.center, self.radius, self.start_angle, self.end_angle);
        self.path_is_dirty = false;
    }
}

impl ShapeTrait for CircularShape {
    fn bounding_rect(&self) -> RectF {
        RectF::from(
            self.center.x() - self.radius,
            self.center.y() - self.radius,
            self.radius * 2.0,
            self.radius * 2.0,
        )
    }

    fn repaint(&mut self, painter: &mut dyn PainterTrait) {
        self.update_path();
        painter.stroke(&self.path);
    }
}
