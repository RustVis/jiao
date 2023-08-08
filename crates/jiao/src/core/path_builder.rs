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

    pub fn close(&mut self) {
        unimplemented!()
    }

    pub fn offset(&mut self, _dx: Scalar, _dy: Scalar) -> &mut Self {
        unimplemented!()
    }

    pub fn toggle_inverse_fill_type(&mut self) -> &mut Self {
        self.fill_type = self.fill_type.inverse();
        self
    }

    pub fn move_to(&mut self, x: Scalar, y: Scalar) -> &mut Self {
        self.move_to_point(Point::from_xy(x, y))
    }

    pub fn move_to_point(&mut self, _point: Point) -> &mut Self {
        self
    }

    pub fn line_to(&mut self, x: Scalar, y: Scalar) -> &mut Self {
        self.line_to_point(Point::from_xy(x, y))
    }

    pub fn line_to_point(&mut self, _point: Point) -> &mut Self {
        self
    }

    pub fn quad_to(&mut self, x1: Scalar, y1: Scalar, x2: Scalar, y2: Scalar) -> &mut Self {
        self.quad_to_point(Point::from_xy(x1, y1), Point::from_xy(x2, y2))
    }

    pub fn quad_to_point(&mut self, _pt1: Point, _pt2: Point) -> &mut Self {
        self
    }

    pub fn conic_to(
        &mut self,
        x1: Scalar,
        y1: Scalar,
        x2: Scalar,
        y2: Scalar,
        weight: Scalar,
    ) -> &mut Self {
        self.conic_to_point(Point::from_xy(x1, y1), Point::from_xy(x2, y2), weight)
    }

    pub fn conic_to_point(&mut self, _pt1: Point, _pt2: Point, _weight: Scalar) -> &mut Self {
        self
    }

    pub fn cubic_to(
        &mut self,
        x1: Scalar,
        y1: Scalar,
        x2: Scalar,
        y2: Scalar,
        x3: Scalar,
        y3: Scalar,
    ) -> &mut Self {
        self.cubic_to_point(
            Point::from_xy(x1, y1),
            Point::from_xy(x2, y2),
            Point::from_xy(x3, y3),
        )
    }

    pub fn cubic_to_point(&mut self, _pt1: Point, _pt2: Point, _pt3: Point) -> &mut Self {
        self
    }

    /// Append a series of Line.
    pub fn polyline_to(&mut self, _points: &[Point]) -> &mut Self {
        self
    }

    // Relative versions of segments, relative to the previous position.
    pub fn relative_line_to(&mut self, x: Scalar, y: Scalar) -> &mut Self {
        self.relative_line_to_point(Point::from_xy(x, y))
    }

    pub fn relative_line_to_point(&mut self, _point: Point) -> &mut Self {
        self
    }

    pub fn relative_quad_to(
        &mut self,
        x1: Scalar,
        y1: Scalar,
        x2: Scalar,
        y2: Scalar,
    ) -> &mut Self {
        self.relative_quad_to_point(Point::from_xy(x1, y1), Point::from_xy(x2, y2))
    }

    pub fn relative_quad_to_point(&mut self, _pt1: Point, _pt2: Point) -> &mut Self {
        self
    }

    pub fn relative_conic_to(
        &mut self,
        x1: Scalar,
        y1: Scalar,
        x2: Scalar,
        y2: Scalar,
        weight: Scalar,
    ) -> &mut Self {
        self.relative_conic_to_point(Point::from_xy(x1, y1), Point::from_xy(x2, y2), weight)
    }

    pub fn relative_conic_to_point(
        &mut self,
        _pt1: Point,
        _pt2: Point,
        _weight: Scalar,
    ) -> &mut Self {
        self
    }

    pub fn relative_cubic_to(
        &mut self,
        x1: Scalar,
        y1: Scalar,
        x2: Scalar,
        y2: Scalar,
        x3: Scalar,
        y3: Scalar,
    ) -> &mut Self {
        self.relative_cubic_to_point(
            Point::from_xy(x1, y1),
            Point::from_xy(x2, y2),
            Point::from_xy(x3, y3),
        )
    }

    pub fn relative_cubic_to_point(&mut self, _pt1: Point, _pt2: Point, _pt3: Point) -> &mut Self {
        self
    }
}
