// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

use super::ShapeTrait;
use crate::base::{PointF, RectF};
use crate::kernel::{PainterTrait, PathTrait};
use crate::platforms::Path;

const DEFAULT_END_ANGLE: f64 = std::f64::consts::TAU;

#[derive(Debug, Clone)]
pub struct EllipseShape {
    center: PointF,
    radius_x: f64,
    radius_y: f64,
    rotation: f64,
    start_angle: f64,
    end_angle: f64,

    path_is_dirty: bool,
    path: Path,
}

impl EllipseShape {
    /// Create a new ellipse shape.
    pub fn new(center: PointF, radius_x: f64, radius_y: f64) -> Self {
        let path = Path::new();
        Self {
            center,
            radius_x,
            radius_y,
            rotation: 0.0,
            start_angle: 0.0,
            end_angle: DEFAULT_END_ANGLE,
            path_is_dirty: true,
            path,
        }
    }

    /// Get center point of the ellipse shape.
    pub fn center(&self) -> PointF {
        self.center
    }

    /// Set center point of the ellipse shape.
    pub fn set_center(&mut self, center: PointF) {
        self.center = center;
        self.path_is_dirty = true;
    }

    /// Get x-axis radius of the ellipse shape.
    pub fn radius_x(&self) -> f64 {
        self.radius_x
    }

    /// Set x-axis radius of the ellipse shape.
    ///
    /// Note that radius_x shall be non-negative.
    pub fn set_radius_x(&mut self, radius_x: f64) {
        assert!(radius_x >= 0.0);
        self.radius_x = radius_x;
        self.path_is_dirty = true;
    }

    /// Get y-axis radius of the ellipse shape.
    pub fn radius_y(&self) -> f64 {
        self.radius_y
    }

    /// Set y-axis radius of the ellipse shape.
    pub fn set_radius_y(&mut self, radius_y: f64) {
        assert!(radius_y >= 0.0);
        self.radius_y = radius_y;
        self.path_is_dirty = true;
    }

    /// Get rotation of the ellipse shape.
    pub fn rotation(&self) -> f64 {
        self.rotation
    }

    /// Set rotation of the ellipse shape.
    pub fn set_rotation(&mut self, rotation: f64) {
        self.rotation = rotation;
        self.path_is_dirty = true;
    }

    /// Get start angle of the ellipse shape.
    pub fn start_angle(&self) -> f64 {
        self.start_angle
    }

    /// Set start angle of the ellipse shape.
    pub fn set_start_angle(&mut self, start_angle: f64) {
        self.start_angle = start_angle;
        self.path_is_dirty = true;
    }

    /// Get end angle of the ellipse shape.
    pub fn end_angle(&self) -> f64 {
        self.end_angle
    }

    /// Set end angle of the ellipse shape.
    pub fn set_end_angle(&mut self, end_angle: f64) {
        self.end_angle = end_angle;
        self.path_is_dirty = true;
    }

    fn update_path(&mut self) {
        if !self.path_is_dirty {
            return;
        }
        self.path = Path::new();
        self.path.ellipse(
            self.center,
            self.radius_x,
            self.radius_y,
            self.rotation,
            self.start_angle,
            self.end_angle,
        );
        self.path_is_dirty = false;
    }
}

impl ShapeTrait for EllipseShape {
    fn bounding_rect(&self) -> RectF {
        todo!()
    }

    fn repaint(&mut self, painter: &mut dyn PainterTrait) {
        self.update_path();
        painter.stroke(&self.path);
    }
}
