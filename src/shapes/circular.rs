// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

use super::ShapeTrait;
use crate::base::{PointF, RectF};
use crate::kernel::{PainterTrait, PathTrait};
use crate::platforms::Path;

const DEFAULT_END_ANGLE: f64 = std::f64::consts::TAU;

#[derive(Debug, Clone)]
pub struct CircularShape {
    center: PointF,
    radius: f64,
    start_angle: f64,
    end_angle: f64,

    path_is_dirty: bool,
    path: Path,
}

impl CircularShape {
    /// Create a new circular shape object.
    pub fn new(center: PointF, radius: f64) -> Self {
        let path = Path::new();
        Self {
            center,
            radius,
            start_angle: 0.0,
            end_angle: DEFAULT_END_ANGLE,
            path_is_dirty: true,
            path,
        }
    }

    /// Get center point of the circular shape.
    pub fn center(&self) -> PointF {
        self.center
    }

    /// Update center point of the circular shape.
    pub fn set_center(&mut self, center: PointF) {
        self.center = center;
        self.path_is_dirty = true;
    }

    /// Get radius of the circular shape.
    pub fn radius(&self) -> f64 {
        self.radius
    }

    /// Set radius of the circular shape.
    ///
    /// Note that radius shall be non-negative.
    pub fn set_radius(&mut self, radius: f64) {
        assert!(radius >= 0.0);
        self.radius = radius;
        self.path_is_dirty = true;
    }

    /// Get start angle of the circular shape.
    pub fn start_angle(&self) -> f64 {
        self.start_angle
    }

    /// Set start angle of the circular shape.
    pub fn set_start_angle(&mut self, start_angle: f64) {
        self.start_angle = start_angle;
        self.path_is_dirty = true;
    }

    /// Get end angle of the circular shape.
    pub fn end_angle(&self) -> f64 {
        self.end_angle
    }

    /// Set end angle of the circular shape.
    pub fn set_end_angle(&mut self, end_angle: f64) {
        self.end_angle = end_angle;
        self.path_is_dirty = true;
    }

    fn update_path(&mut self) {
        self.path = Path::new();
        self.path
            .arc(self.center, self.radius, self.start_angle, self.end_angle);
    }
}

impl ShapeTrait for CircularShape {
    fn bounding_rect(&self) -> RectF {
        todo!()
    }

    fn repaint(&mut self, painter: &mut dyn PainterTrait) {
        if self.path_is_dirty {
            self.update_path();
            self.path_is_dirty = false;
        }
        painter.stroke(&self.path);
    }
}
