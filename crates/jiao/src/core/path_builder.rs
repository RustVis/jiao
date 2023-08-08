// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

use crate::core::path::Path;
use crate::core::path_types::{PathFillType, PathVerb};
use crate::core::point::Point;
use crate::core::rect::Rect;
use crate::core::scalar::Scalar;

#[derive(Debug, Clone)]
pub struct PathBuilder {
    points: Vec<Point>,
    verbs: Vec<PathVerb>,
    conic_weights: Vec<Scalar>,
    fill_type: PathFillType,
    last_move_to_index: usize,
    move_to_required: bool,
}

impl Default for PathBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl PathBuilder {
    #[must_use]
    pub const fn new() -> Self {
        Self {
            points: Vec::new(),
            verbs: Vec::new(),
            conic_weights: Vec::new(),
            fill_type: PathFillType::Winding,
            last_move_to_index: 0,
            move_to_required: true,
        }
    }

    #[must_use]
    pub const fn from_fill_type(fill_type: PathFillType) -> Self {
        Self {
            points: Vec::new(),
            verbs: Vec::new(),
            conic_weights: Vec::new(),
            fill_type,
            last_move_to_index: 0,
            move_to_required: true,
        }
    }

    #[must_use]
    pub fn from_points_verbs(points: Vec<Point>, verbs: Vec<PathVerb>) -> Self {
        Self {
            points,
            verbs,
            conic_weights: Vec::new(),
            fill_type: PathFillType::Winding,
            last_move_to_index: 0,
            move_to_required: true,
        }
    }

    #[must_use]
    #[allow(clippy::missing_const_for_fn)]
    pub fn from_path(path: Path) -> Self {
        Self {
            points: path.points,
            verbs: path.verbs,
            conic_weights: Vec::new(),
            fill_type: path.fill_type,
            last_move_to_index: 0,
            move_to_required: true,
        }
    }

    pub fn reset(&mut self) {
        self.points.clear();
        self.verbs.clear();
        self.fill_type = PathFillType::Winding;
        self.last_move_to_index = 0;
        self.move_to_required = true;
    }

    /// The builder is unchanged after returning this path
    #[must_use]
    pub fn snapshot(&self) -> Option<Path> {
        if self.verbs.len() == 1 {
            return None;
        }

        let bounds = self.compute_bounds()?;

        Some(Path::new(
            self.points.clone(),
            self.verbs.clone(),
            bounds,
            self.fill_type,
        ))
    }

    /// The builder is reset to empty after returning this path
    #[must_use]
    pub fn detach(&mut self) -> Option<Path> {
        if self.verbs.len() == 1 {
            return None;
        }

        let bounds = self.compute_bounds()?;

        let path = Some(Path::new(
            self.points.clone(),
            self.verbs.clone(),
            bounds,
            self.fill_type,
        ));
        *self = Self::new();

        path
    }

    /// Finishes the builder and returns a `Path`.
    ///
    /// Returns `None` when `Path` is empty or has invalid bounds.
    #[must_use]
    pub fn finish(self) -> Option<Path> {
        if self.verbs.len() == 1 {
            return None;
        }

        let bounds = self.compute_bounds()?;

        Some(Path::new(self.points, self.verbs, bounds, self.fill_type))
    }

    pub fn set_fill_type(&mut self, fill_type: PathFillType) {
        self.fill_type = fill_type;
    }

    #[must_use]
    pub fn compute_bounds(&self) -> Option<Rect> {
        if self.is_empty() {
            return None;
        }
        let bounds = Rect::from_points(&self.points);
        if bounds.is_empty() {
            return None;
        }
        Some(bounds)
    }

    pub fn reserve(&mut self, additional_verbs: usize, additional_points: usize) {
        self.verbs.reserve(additional_verbs);
        self.points.reserve(additional_points);
    }

    #[must_use]
    pub const fn fill_type(&self) -> PathFillType {
        self.fill_type
    }

    #[must_use]
    pub fn len(&self) -> usize {
        self.verbs.len()
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.verbs.is_empty()
    }
}
