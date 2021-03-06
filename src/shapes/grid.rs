// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

use crate::base::{PointF, RectF};
use crate::kernel::{PainterTrait, PathTrait};
use crate::platforms::Path;
use crate::shapes::ShapeTrait;
use crate::util::fuzzy_compare;

#[allow(clippy::module_name_repetitions)]
#[derive(Debug)]
pub struct GridShape {
    horizontal_step: f64,
    vertical_step: f64,
    start_point: PointF,
    viewport: RectF,
    horizontal_visible: bool,
    vertical_visible: bool,
    path_is_dirty: bool,
    path: Path,
}

impl GridShape {
    /// Create a new grid shape.
    ///
    /// # Panics
    ///
    /// Both `horizontal_step` and `vertical_step` shall be >= 0.0.
    #[must_use]
    pub fn new(horizontal_step: f64, vertical_step: f64) -> Self {
        assert!(horizontal_step >= 0.0 && vertical_step >= 0.0);
        let start_point = PointF::new();
        let viewport = RectF::from(0.0, 0.0, 100.0, 100.0);
        let path = Path::new();
        Self {
            horizontal_step,
            vertical_step,
            start_point,
            viewport,
            horizontal_visible: true,
            vertical_visible: true,
            path_is_dirty: true,
            path,
        }
    }

    /// Get horizontal step value.
    #[must_use]
    pub const fn horizontal_step(&self) -> f64 {
        self.horizontal_step
    }

    /// Update value of horizontal step.
    ///
    /// # Panics
    ///
    /// `horizontal_step` shall be >= 0.0.
    pub fn set_horizontal_step(&mut self, horizontal_step: f64) {
        assert!(horizontal_step >= 0.0);
        if !fuzzy_compare(self.horizontal_step, horizontal_step) {
            self.horizontal_step = horizontal_step;
            self.path_is_dirty = true;
        }
    }

    /// Check if horizontal step is visible.
    #[must_use]
    pub const fn horizontal_visible(&self) -> bool {
        self.horizontal_visible
    }

    /// Make horizontal step invisible.
    pub fn set_horizontal_visible(&mut self, visible: bool) {
        if self.horizontal_visible != visible {
            self.horizontal_visible = visible;
            self.path_is_dirty = true;
        }
    }

    /// Get vertical step value.
    #[must_use]
    pub const fn vertical_step(&self) -> f64 {
        self.vertical_step
    }

    /// Update value of vertical step.
    ///
    /// # Panics
    ///
    /// `vertical_step` shall be >= 0.0.
    pub fn set_vertical_step(&mut self, vertical_step: f64) {
        assert!(vertical_step >= 0.0);
        if !fuzzy_compare(self.vertical_step, vertical_step) {
            self.vertical_step = vertical_step;
            self.path_is_dirty = true;
        }
    }

    /// Check if vertical step is visible.
    #[must_use]
    pub const fn vertical_visible(&self) -> bool {
        self.vertical_visible
    }

    /// Make vertical step invisible.
    pub fn set_vertical_visible(&mut self, visible: bool) {
        if self.vertical_visible != visible {
            self.vertical_visible = visible;
            self.path_is_dirty = true;
        }
    }

    /// Get start point of grid.
    #[must_use]
    pub const fn start_point(&self) -> PointF {
        self.start_point
    }

    /// Update `start_point` of grid.
    pub fn set_start_point(&mut self, start_point: PointF) {
        self.start_point = start_point;
        self.path_is_dirty = true;
    }

    /// Get current viewport.
    #[must_use]
    pub const fn viewport(&self) -> &RectF {
        &self.viewport
    }

    /// Update viewport of grid.
    pub fn set_viewport(&mut self, viewport: RectF) {
        self.viewport = viewport;
        self.path_is_dirty = true;
    }

    fn update_path(&mut self) {
        if !self.path_is_dirty {
            return;
        }
        self.path.clear();

        self.path_is_dirty = false;
    }
}

impl ShapeTrait for GridShape {
    fn bounding_rect(&self) -> RectF {
        self.viewport.clone()
    }

    fn repaint(&mut self, painter: &mut dyn PainterTrait) {
        self.update_path();
        painter.stroke(&self.path);
    }
}
