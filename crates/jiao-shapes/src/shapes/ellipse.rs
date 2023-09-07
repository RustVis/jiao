// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by LGPL-3.0 License that can be found
// in the LICENSE file.

use jiao::base::{PointF, RectF};
use jiao::kernel::{PainterTrait, PathTrait};

use crate::platforms::Path;
use crate::ShapeTrait;

#[derive(Debug, Clone)]
pub struct EllipseShape {
    center: PointF,
    radius_x: f64,
    radius_y: f64,

    path_is_dirty: bool,
    path: Path,
}

impl EllipseShape {
    /// Create a new ellipse shape.
    #[must_use]
    pub fn new(center: PointF, radius_x: f64, radius_y: f64) -> Self {
        debug_assert!(radius_x > 0.0);
        debug_assert!(radius_y > 0.0);
        let path = Path::new();
        Self {
            center,
            radius_x,
            radius_y,
            path_is_dirty: true,
            path,
        }
    }

    #[must_use]
    pub fn from_rect(rect: &RectF) -> Self {
        let center = rect.center();
        let radius_x = center.x() - rect.left();
        let radius_y = center.y() - rect.top();
        Self::new(center, radius_x, radius_y)
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
    pub fn set_radius_x(&mut self, radius_x: f64) {
        debug_assert!(radius_x > 0.0);
        self.radius_x = radius_x;
        self.path_is_dirty = true;
    }

    /// Get y-axis radius of the ellipse shape.
    #[must_use]
    pub const fn radius_y(&self) -> f64 {
        self.radius_y
    }

    /// Set y-axis radius of the ellipse shape.
    pub fn set_radius_y(&mut self, radius_y: f64) {
        debug_assert!(radius_y > 0.0);
        self.radius_y = radius_y;
        self.path_is_dirty = true;
    }

    fn update_path(&mut self) {
        if !self.path_is_dirty {
            return;
        }
        self.path.clear();
        let rect = RectF::from_ellipse(self.center, self.radius_x, self.radius_y);
        self.path.add_ellipse(&rect);
        self.path_is_dirty = false;
    }
}

impl ShapeTrait for EllipseShape {
    fn bounding_rect(&self) -> RectF {
        RectF::from_ellipse(self.center, self.radius_x, self.radius_y)
    }

    fn repaint(&mut self, painter: &mut dyn PainterTrait) {
        self.update_path();
        painter.stroke(&self.path);
    }
}
