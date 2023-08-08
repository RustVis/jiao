// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

use crate::core::path::Path;
use crate::core::path_types::ArcSize;
use crate::core::path_types::PathDirection;
use crate::core::path_types::{PathFillType, PathVerb};
use crate::core::point::Point;
use crate::core::rect::Rect;
use crate::core::rrect::RRect;
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

    pub fn add_path(&mut self, _path: &Path) -> &mut Self {
        self
    }

    // Add a new contour

    pub fn add_rect(
        &mut self,
        _rect: &Rect,
        _dir: PathDirection,
        _start_index: usize,
    ) -> &mut Self {
        self
    }

    pub fn add_oval(
        &mut self,
        _rect: &Rect,
        _dir: PathDirection,
        _start_index: usize,
    ) -> &mut Self {
        self
    }

    pub fn add_rrect(
        &mut self,
        _rrect: &RRect,
        _dir: PathDirection,
        _start_index: usize,
    ) -> &mut Self {
        self
    }

    pub fn add_circle(
        &mut self,
        _center_x: Scalar,
        _center_y: Scalar,
        _radius: Scalar,
        _dir: PathDirection,
    ) -> &mut Self {
        self
    }

    pub fn add_polygon(&mut self, _points: &[Point], _is_closed: bool) -> &mut Self {
        self
    }

    // Arcs
    /// Appends arc to the builder.
    ///
    /// Arc added is part of ellipse bounded by oval, from `start_angle` through `sweep_angle`.
    /// Both `start_angle` and `sweep_angle` are measured in degrees, where zero degrees
    /// is aligned with the positive x-axis, and positive sweeps extends arc clockwise.
    ///
    /// `arc_to_angle()` adds line connecting the builder's last point to initial arc point
    /// if `force_move_to` is false and the builder is not empty.
    /// Otherwise, added contour begins with first point of arc.
    /// Angles greater than -360 and less than 360 are treated modulo 360.
    ///
    /// # Parameters
    /// - `oval` - bounds of ellipse containing arc
    /// - `start_angle_deg` - starting angle of arc in degrees
    /// - `sweep_angle_deg` - sweep, in degrees. Positive is clockwise; treated modulo 360
    /// - `force_move_to` - true to start a new contour with arc
    pub fn arc_to_angle(
        &mut self,
        _oval: &Rect,
        _start_angle_deg: Scalar,
        _sweep_angle_deg: Scalar,
        _force_move_to: bool,
    ) -> &mut Self {
        self
    }

    /// Appends arc to Path, after appending line if needed.
    ///
    /// Arc is implemented by conic weighted to describe part of circle.
    /// Arc is contained by tangent from last Path point to p1, and tangent from p1 to p2.
    /// Arc is part of circle sized to radius, positioned so it touches both tangent lines.
    ///
    /// If last Path Point does not start arc, `arc_to_point()` appends connecting line to Path.
    /// The length of vector from p1 to p2 does not affect arc.
    ///
    /// Arc sweep is always less than 180 degrees. If radius is zero, or if
    /// tangents are nearly parallel, `arc_to_point()` appends line from last Path Point to p1.
    ///
    /// `arc_to_point()` appends at most one line and one conic.
    /// `arc_to_point()` implements the functionality of Post-Script arct and HTML Canvas arcTo.
    ///
    /// # Parameters
    /// - `p1` - Point common to pair of tangents
    /// - `p2` - end of second tangent
    /// - `radius` - distance from arc to circle center
    pub fn arc_to_point(&mut self, _pt1: Point, _pt2: Point, _radius: Scalar) -> &mut Self {
        self
    }

    /// Appends arc to Path.
    ///
    /// Arc is implemented by one or more conic weighted to describe part of oval
    /// with radii (r.x, r.y) rotated by `x_axis_rotate` degrees.
    /// Arc curves from last Path Point to (xy.x, xy.y), choosing one of four possible routes:
    /// clockwise or counterclockwise, and smaller or larger.
    ///
    /// Arc sweep is always less than 360 degrees. `arc_to_rotate()` appends line to xy if either
    /// radii are zero, or if last Path Point equals (xy.x, xy.y).
    /// `arc_to_rotate()` scales radii r to fit last Path Point and xy if
    /// both are greater than zero but too small to describe an arc.
    ///
    /// `arc_to_rotate()` appends up to four conic curves.
    /// `arc_to_rotate()` implements the functionality of SVG arc, although SVG sweep-flag value
    /// is opposite the integer value of sweep; SVG sweep-flag uses 1 for clockwise,
    /// while `PathDirection::CW` cast to int is zero.
    ///
    /// # Parameters
    /// - `radii` - radii on axes before x-axis rotation
    /// - `x_axis_rotate` - x-axis rotation in degrees; positive values are clockwise
    /// - `large_arc` - chooses smaller or larger arc
    /// - `sweep` - chooses clockwise or counterclockwise arc
    /// - `xy` - end point of arc
    pub fn arc_to_rotate(
        &mut self,
        _radii: Point,
        _x_axis_rotate: Scalar,
        _large_arc: ArcSize,
        _sweep: PathDirection,
        _xy: Point,
    ) -> &mut Self {
        self
    }

    /// Appends arc to the builder, as the start of new contour.
    ///
    /// Arc added is part of ellipse bounded by oval, from `start_angle` through `sweep_angle`.
    /// Both `start_angle` and `sweep_angle` are measured in degrees, where zero degrees
    /// is aligned with the positive x-axis, and positive sweeps extends arc clockwise.
    ///
    /// If `sweep_angle` <= -360, or `sweep_angle` >= 360; and `start_angle` modulo 90
    /// is nearly zero, append oval instead of arc.
    /// Otherwise, `sweep_angle` values are treated modulo 360, and arc may or may not draw
    /// depending on numeric rounding.
    ///
    /// # Parameters
    /// - `oval` - bounds of ellipse containing arc
    /// - `start_angle_deg` - starting angle of arc in degrees
    /// - `sweep_angle_deg` - sweep, in degrees. Positive is clockwise; treated modulo 360
    pub fn add_arc(
        &mut self,
        _oval: &Rect,
        _start_angle_deg: Scalar,
        _sweep_angle_deg: Scalar,
    ) -> &mut Self {
        self
    }
}
