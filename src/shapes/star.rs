// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

use crate::base::RectF;
use crate::kernel::{PainterTrait, PathTrait};
use crate::platforms::Path;
use crate::shapes::ShapeTrait;
use crate::util::fuzzy_compare;

const VERTEX_MIN: usize = 3;
const VERTEX_MAX: usize = 99;

#[allow(clippy::module_name_repetitions)]
#[derive(Debug)]
pub struct StarShape {
    corners: usize,
    corner_radius: f64,
    path: Path,
    path_is_dirty: bool,
}

impl StarShape {
    /// Create a new star shape object with specified `corners`.
    ///
    /// Corner radius is set to 0.0.
    ///
    /// # Panics
    ///
    /// Note that corners shall be in range 3..90.
    #[must_use]
    pub fn new(corners: usize) -> Self {
        assert!(corners >= VERTEX_MIN && corners <= VERTEX_MAX);
        let path = Path::new();
        Self {
            corners,
            corner_radius: 0.0,
            path,
            path_is_dirty: true,
        }
    }

    /// Create a five-corner star shape.
    #[must_use]
    pub fn new_star() -> Self {
        Self::new(5)
    }

    /// Get current number of corners.
    #[must_use]
    pub const fn corners(&self) -> usize {
        self.corners
    }

    /// Set number of corners.
    ///
    /// # Panics
    ///
    /// Note that corners shall be in range 3..90.
    pub fn set_corners(&mut self, corners: usize) {
        assert!(corners >= VERTEX_MIN && corners <= VERTEX_MAX);
        if self.corners != corners {
            self.path_is_dirty = true;
            self.corners = corners;
        }
    }

    /// Get current corner radius.
    #[must_use]
    pub const fn corner_radius(&self) -> f64 {
        self.corner_radius
    }

    /// Set corner radius.
    ///
    /// # Panics
    ///
    /// Note that radius shall be a non-negative number.
    pub fn set_corner_radius(&mut self, radius: f64) {
        assert!(radius >= 0.0);
        if !fuzzy_compare(self.corner_radius, radius) {
            self.path_is_dirty = true;
            self.corner_radius = radius;
        }
    }

    fn update_path(&mut self) {
        if !self.path_is_dirty {
            return;
        }
        self.path.clear();
        // TODO(Shaohua): draw star shape.
        self.path_is_dirty = false;
    }
}

impl ShapeTrait for StarShape {
    fn bounding_rect(&self) -> RectF {
        todo!()
    }

    fn repaint(&mut self, painter: &mut dyn PainterTrait) {
        self.update_path();
        painter.stroke(&self.path);
    }
}
