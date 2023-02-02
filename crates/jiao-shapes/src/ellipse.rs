// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

use jiao::base::{PointF, RectF};
use jiao::kernel::{PainterTrait, PathTrait, ShapeTrait};

use crate::Path2D;

const DEFAULT_END_ANGLE: f64 = std::f64::consts::TAU;

#[derive(Debug)]
pub struct EllipseShape {
    center: PointF,
    radius_x: f64,
    radius_y: f64,
    start_angle: f64,
    end_angle: f64,

    path_is_dirty: bool,
    path: Path2D,
}

impl EllipseShape {
    /// Create a new ellipse shape.
    ///
    /// # Panics
    ///
    /// Both `radius_x` and `radius_y` shall be >= 0.0.
    #[must_use]
    pub fn new(center: PointF, radius_x: f64, radius_y: f64) -> Self {
        assert!(radius_x >= 0.0);
        assert!(radius_y >= 0.0);
        let path = Path2D::new();
        Self {
            center,
            radius_x,
            radius_y,
            start_angle: 0.0,
            end_angle: DEFAULT_END_ANGLE,
            path_is_dirty: true,
            path,
        }
    }

    /// Get center point of the ellipse shape.
    #[must_use]
    pub const fn center(&self) -> PointF {
        self.center
    }

    /// Set center point of the ellipse shape.
    pub fn set_center(&mut self, center: PointF) {
        self.center = center;
        self.path_is_dirty = true;
    }

    /// Get x-axis radius of the ellipse shape.
    #[must_use]
    pub const fn radius_x(&self) -> f64 {
        self.radius_x
    }

    /// Set x-axis radius of the ellipse shape.
    ///
    /// # Panics
    ///
    /// `radius_x` shall be non-negative.
    pub fn set_radius_x(&mut self, radius_x: f64) {
        assert!(radius_x >= 0.0);
        self.radius_x = radius_x;
        self.path_is_dirty = true;
    }

    /// Get y-axis radius of the ellipse shape.
    #[must_use]
    pub const fn radius_y(&self) -> f64 {
        self.radius_y
    }

    /// Set y-axis radius of the ellipse shape.
    ///
    /// # Panics
    ///
    /// `radius_y` shall be >= 0.0.
    pub fn set_radius_y(&mut self, radius_y: f64) {
        assert!(radius_y >= 0.0);
        self.radius_y = radius_y;
        self.path_is_dirty = true;
    }

    /// Get start angle of the ellipse shape.
    #[must_use]
    pub const fn start_angle(&self) -> f64 {
        self.start_angle
    }

    /// Set start angle of the ellipse shape.
    pub fn set_start_angle(&mut self, start_angle: f64) {
        self.start_angle = start_angle;
        self.path_is_dirty = true;
    }

    /// Get end angle of the ellipse shape.
    #[must_use]
    pub const fn end_angle(&self) -> f64 {
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
        self.path.clear();
        self.path.ellipse(
            self.center,
            self.radius_x,
            self.radius_y,
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
