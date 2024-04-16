// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that can be found
// in the LICENSE file.

use crate::core::path_builder::PathBuilder;
use crate::core::path_types::{PathFillType, PathVerb};
use crate::core::point::Point;
use crate::core::rect::Rect;
use crate::core::scalar::Scalar;

#[derive(Debug, Clone)]
pub struct Path {
    points: Vec<Point>,
    verbs: Vec<PathVerb>,
    conic_weights: Vec<Scalar>,
    bounds: Rect,
    fill_type: PathFillType,
}

impl Path {
    #[must_use]
    pub(crate) fn new(
        points: Vec<Point>,
        verbs: Vec<PathVerb>,
        conic_weights: Vec<Scalar>,
        bounds: Rect,
        fill_type: PathFillType,
    ) -> Self {
        Self {
            points,
            verbs,
            conic_weights,
            bounds,
            fill_type,
        }
    }

    /// Clears the path and returns a `PathBuilder` that will reuse an allocated memory.
    #[must_use]
    pub fn clear(mut self) -> PathBuilder {
        self.points.clear();
        self.verbs.clear();
        self.conic_weights.clear();

        PathBuilder::from_points_verbs(self.points, self.verbs, self.conic_weights)
    }

    #[must_use]
    pub const fn fill_type(&self) -> PathFillType {
        self.fill_type
    }

    #[must_use]
    pub fn points(&self) -> &[Point] {
        &self.points
    }

    #[must_use]
    pub fn verbs(&self) -> &[PathVerb] {
        &self.verbs
    }

    #[must_use]
    pub fn conic_weights(&self) -> &[Scalar] {
        &self.conic_weights
    }
}

impl From<Path> for PathBuilder {
    fn from(path: Path) -> Self {
        Self::from_points_verbs(path.points, path.verbs, path.conic_weights)
    }
}
