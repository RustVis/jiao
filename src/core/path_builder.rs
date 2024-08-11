// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that can be found
// in the LICENSE file.

use crate::core::path::Path;
use crate::core::path_builder_priv::PointIter;
use crate::core::path_types::{PathFillType, PathVerb};
use crate::core::path_types::ArcSize;
use crate::core::path_types::PathDirection;
use crate::core::path_types::PathSegmentMask;
use crate::core::point::Point;
use crate::core::rect::Rect;
use crate::core::rrect::RRect;
use crate::core::scalar::{Scalar, SCALAR_ROOT_2_OVER_2};

#[derive(Debug, Clone)]
pub struct PathBuilder {
    points: Vec<Point>,
    verbs: Vec<PathVerb>,
    conic_weights: Vec<Scalar>,
    fill_type: PathFillType,

    // Internal states:
    segment_mask: PathSegmentMask,
    last_move_to_index: usize,
    needs_move_verb: bool,

    // TODO(Shaohua): Remove
    is_a: IsA,
    // tracks direction iff fIsA is not unknown
    is_a_start: usize,
    // tracks direction iff fIsA is not unknown
    is_a_ccw: bool,
}

#[repr(u8)]
#[derive(Debug, Default, Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
enum IsA {
    /// we only have 0 or more moves
    #[default]
    JustMoves,

    /// we have verbs other than just move
    MoreThanMoves,

    /// we are 0 or more moves followed by an oval
    Oval,

    /// we are 0 or more moves followed by a rrect
    RRect,
}

impl Default for PathBuilder {
    #[inline]
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

            segment_mask: PathSegmentMask::empty(),
            last_move_to_index: usize::MAX,
            needs_move_verb: true,

            is_a: IsA::JustMoves,
            is_a_start: usize::MAX,
            is_a_ccw: false,
        }
    }

    #[must_use]
    pub const fn from_fill_type(fill_type: PathFillType) -> Self {
        Self {
            points: Vec::new(),
            verbs: Vec::new(),
            conic_weights: Vec::new(),
            fill_type,

            segment_mask: PathSegmentMask::empty(),
            last_move_to_index: usize::MAX,
            needs_move_verb: true,

            is_a: IsA::JustMoves,
            is_a_start: usize::MAX,
            is_a_ccw: false,
        }
    }

    #[must_use]
    pub fn from_points_verbs(
        points: Vec<Point>,
        verbs: Vec<PathVerb>,
        conic_weights: Vec<Scalar>,
    ) -> Self {
        Self {
            points,
            verbs,
            conic_weights,
            fill_type: PathFillType::Winding,

            segment_mask: PathSegmentMask::empty(),
            last_move_to_index: usize::MAX,
            needs_move_verb: true,

            is_a: IsA::JustMoves,
            is_a_start: usize::MAX,
            is_a_ccw: false,
        }
    }

    #[must_use]
    #[inline]
    #[allow(clippy::missing_const_for_fn)]
    pub fn from_path(path: Path) -> Self {
        path.into()
    }

    /// Creates a new `Path` from `Rect`.
    ///
    /// Never fails since `Rect` is always valid.
    ///
    /// Segments are created clockwise: `TopLeft -> TopRight -> BottomRight -> BottomLeft`
    ///
    /// The contour is closed.
    #[must_use]
    pub fn from_rect(rect: &Rect) -> Option<Path> {
        let mut pb = Self::new();
        pb.add_rect(rect);
        pb.finish()
    }

    /// Creates a new `Path` from a circle.
    ///
    /// Segments are created clockwise.
    #[must_use]
    pub fn from_circle(cx: f32, cy: f32, radius: f32) -> Option<Path> {
        let mut pb = Self::new();
        pb.add_circle(cx, cy, radius);
        pb.finish()
    }

    /// Creates a new `Path` from an oval.
    ///
    /// Segments are created clockwise.
    #[must_use]
    pub fn from_oval(oval: &Rect) -> Option<Path> {
        let mut pb = Self::new();
        pb.add_oval(oval);
        pb.finish()
    }

    pub fn reset(&mut self) -> &mut Self {
        self.points.clear();
        self.verbs.clear();
        self.conic_weights.clear();
        self.fill_type = PathFillType::Winding;

        self.segment_mask = PathSegmentMask::empty();
        self.last_move_to_index = usize::MAX;
        self.needs_move_verb = true;

        self.is_a = IsA::JustMoves;
        self.is_a_start = usize::MAX;
        self.is_a_ccw = false;

        self
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
            self.conic_weights.clone(),
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
            self.conic_weights.clone(),
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

        Some(Path::new(
            self.points,
            self.verbs,
            self.conic_weights,
            bounds,
            self.fill_type,
        ))
    }

    #[inline]
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

    #[inline]
    pub fn reserve(&mut self, additional_verbs: usize, additional_points: usize) {
        self.verbs.reserve(additional_verbs);
        self.points.reserve(additional_points);
    }

    #[must_use]
    #[inline]
    pub const fn fill_type(&self) -> PathFillType {
        self.fill_type
    }

    #[must_use]
    #[inline]
    pub fn len(&self) -> usize {
        self.verbs.len()
    }

    #[must_use]
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.verbs.is_empty()
    }

    pub fn offset(&mut self, _dx: Scalar, _dy: Scalar) -> &mut Self {
        unimplemented!()
    }

    #[inline]
    pub fn toggle_inverse_fill_type(&mut self) -> &mut Self {
        self.fill_type = self.fill_type.inverse();
        self
    }

    pub fn close(&mut self) -> &mut Self {
        if !self.verbs.is_empty() {
            self.ensure_move();

            self.verbs.push(PathVerb::Close);

            // last_move_to_index stays where it is -- the previous moveTo
            self.needs_move_verb = true;
        }

        self
    }

    #[inline]
    pub fn move_to(&mut self, x: Scalar, y: Scalar) -> &mut Self {
        self.move_to_point(Point::from_xy(x, y))
    }

    /// Adds beginning of a contour.
    ///
    /// Multiple continuous `MoveTo` segments are not allowed.
    /// If the previous segment was also `MoveTo`, it will be overwritten with the current one.

    pub fn move_to_point(&mut self, point: Point) -> &mut Self {
        if self.verbs.last() == Some(&PathVerb::Move) {
            let last_idx = self.points.len() - 1;
            self.points[last_idx] = point;
        } else {
            self.points.push(point);
            self.verbs.push(PathVerb::Move);

            self.last_move_to_index = self.points.len();
            self.needs_move_verb = false;
        }

        self
    }

    #[inline]
    pub fn line_to(&mut self, x: Scalar, y: Scalar) -> &mut Self {
        self.line_to_point(Point::from_xy(x, y))
    }

    pub fn line_to_point(&mut self, point: Point) -> &mut Self {
        self.ensure_move();

        self.points.push(point);
        self.verbs.push(PathVerb::Line);

        self.segment_mask |= PathSegmentMask::Line;

        self
    }

    #[inline]
    pub fn quad_to(&mut self, x1: Scalar, y1: Scalar, x2: Scalar, y2: Scalar) -> &mut Self {
        self.quad_to_point(Point::from_xy(x1, y1), Point::from_xy(x2, y2))
    }

    pub fn quad_to_point(&mut self, pt1: Point, pt2: Point) -> &mut Self {
        self.ensure_move();

        self.points.push(pt1);
        self.points.push(pt2);
        self.verbs.push(PathVerb::Quad);

        self.segment_mask |= PathSegmentMask::Quad;

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

    pub fn conic_to_point(&mut self, pt1: Point, pt2: Point, weight: Scalar) -> &mut Self {
        self.ensure_move();

        self.points.push(pt1);
        self.points.push(pt2);
        self.verbs.push(PathVerb::Conic);
        self.conic_weights.push(weight);

        self.segment_mask |= PathSegmentMask::Conic;

        self
    }

    #[inline]
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

    pub fn cubic_to_point(&mut self, pt1: Point, pt2: Point, pt3: Point) -> &mut Self {
        self.ensure_move();

        self.points.push(pt1);
        self.points.push(pt2);
        self.points.push(pt3);
        self.verbs.push(PathVerb::Cubic);

        self.segment_mask |= PathSegmentMask::Cubic;

        self
    }

    /// Append a series of Line.
    #[inline]
    pub fn polyline_to(&mut self, _points: &[Point]) -> &mut Self {
        self
    }

    /// Relative versions of segments, relative to the previous position.
    #[inline]
    pub fn relative_line_to(&mut self, x: Scalar, y: Scalar) -> &mut Self {
        self.relative_line_to_point(Point::from_xy(x, y))
    }

    pub fn relative_line_to_point(&mut self, point: Point) -> &mut Self {
        self.ensure_move();

        match self.points.last().copied() {
            Some(last_pt) => self.line_to_point(last_pt + point),
            None => self.line_to_point(point),
        }
    }

    #[inline]
    pub fn relative_quad_to(
        &mut self,
        x1: Scalar,
        y1: Scalar,
        x2: Scalar,
        y2: Scalar,
    ) -> &mut Self {
        self.relative_quad_to_point(Point::from_xy(x1, y1), Point::from_xy(x2, y2))
    }

    pub fn relative_quad_to_point(&mut self, pt1: Point, pt2: Point) -> &mut Self {
        self.ensure_move();

        match self.points.last().copied() {
            Some(last_pt) => self.quad_to_point(last_pt + pt1, last_pt + pt2),
            None => self.quad_to_point(pt1, pt2),
        }
    }

    #[inline]
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

    pub fn relative_conic_to_point(&mut self, pt1: Point, pt2: Point, weight: Scalar) -> &mut Self {
        self.ensure_move();

        match self.points.last().copied() {
            Some(last_pt) => self.conic_to_point(last_pt + pt1, last_pt + pt2, weight),
            None => self.conic_to_point(pt1, pt2, weight),
        }
    }

    #[inline]
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

    pub fn relative_cubic_to_point(&mut self, pt1: Point, pt2: Point, pt3: Point) -> &mut Self {
        self.ensure_move();

        match self.points.last().copied() {
            Some(last_pt) => self.cubic_to_point(last_pt + pt1, last_pt + pt2, last_pt + pt3),
            None => self.cubic_to_point(pt1, pt2, pt3),
        }
    }

    pub fn add_path(&mut self, path: &Path) -> &mut Self {
        self.last_move_to_index = self.points.len();

        self.verbs.extend_from_slice(path.verbs());
        self.points.extend_from_slice(path.points());
        self.conic_weights.extend_from_slice(path.conic_weights());

        self
    }

    /// Add a new rect contour.
    ///
    /// The contour is closed and has a clock-wise direction.
    #[inline]
    pub fn add_rect(&mut self, rect: &Rect) -> &mut Self {
        self.add_rect_detail(rect, PathDirection::Cw, 0)
    }

    pub fn add_rect_detail(
        &mut self,
        rect: &Rect,
        dir: PathDirection,
        start_index: usize,
    ) -> &mut Self {
        // moveTo + 3 lines
        const POINTS: usize = 4;
        // moveTo + 3 lines + close
        const VERBS: usize = 5;
        self.reserve(POINTS, VERBS);

        let mut iter = PointIter::new_rect(rect, dir, start_index);

        self.move_to_point(iter.current());
        self.line_to_point(iter.next());
        self.line_to_point(iter.next());
        self.line_to_point(iter.next());
        self.close()
    }

    /// Adds an oval contour bounded by the provided rectangle.
    ///
    /// The contour is closed and has a clock-wise direction.
    #[inline]
    pub fn add_oval(&mut self, oval: &Rect) -> &mut Self {
        self.add_oval_detail(oval, PathDirection::Cw, 0)
    }

    pub fn add_oval_detail(
        &mut self,
        oval: &Rect,
        dir: PathDirection,
        start_index: usize,
    ) -> &mut Self {
        // moveTo + 4 conics(2 pts each)
        const POINTS: usize = 9;
        // moveTo + 4 conics + close
        const VERBS: usize = 6;
        self.reserve(POINTS, VERBS);

        let prev_isa = self.is_a;

        let mut oval_iter = PointIter::new_oval(oval, dir, start_index);
        let rect_index = match dir {
            PathDirection::Cw => start_index,
            PathDirection::Ccw => start_index + 1,
        };
        let mut rect_iter = PointIter::new_rect(oval, dir, rect_index);

        // The corner iterator pts are tracking "behind" the oval/radii pts.

        self.move_to_point(oval_iter.current());
        for _i in 0..4 {
            self.conic_to_point(rect_iter.next(), oval_iter.next(), SCALAR_ROOT_2_OVER_2);
        }
        self.close();
        if prev_isa == IsA::JustMoves {
            self.is_a = IsA::Oval;
            self.is_a_ccw = dir == PathDirection::Ccw;
            self.is_a_start = start_index % 4;
        }
        self
    }

    #[inline]
    pub fn add_rrect(&mut self, rrect: &RRect) -> &mut Self {
        self.add_rrect_detail(rrect, PathDirection::Cw, 0)
    }

    pub fn add_rrect_detail(
        &mut self,
        _rrect: &RRect,
        _dir: PathDirection,
        _start_index: usize,
    ) -> &mut Self {
        // TODO(Shaohua):
        self
    }

    /// Add a circular contour.
    ///
    /// Segments are created clockwise.
    #[inline]
    pub fn add_circle(&mut self, center_x: Scalar, center_y: Scalar, radius: Scalar) -> &mut Self {
        self.add_circle_detail(center_x, center_y, radius, PathDirection::Cw)
    }

    pub fn add_circle_detail(
        &mut self,
        _center_x: Scalar,
        _center_y: Scalar,
        _radius: Scalar,
        _dir: PathDirection,
    ) -> &mut Self {
        // TODO(Shaohua):
        self
    }

    pub fn add_polygon(&mut self, _points: &[Point], _is_closed: bool) -> &mut Self {
        // TODO(Shaohua):
        self
    }

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
        // TODO(Shaohua):
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
        // TODO(Shaohua):
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
        // TODO(Shaohua):
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
        // TODO(Shaohua):
        self
    }

    // called right before we add a (non-move) verb
    fn ensure_move(&mut self) -> &mut Self {
        self.is_a = IsA::MoreThanMoves;
        if self.needs_move_verb {
            match self.points.get(self.last_move_to_index).copied() {
                Some(p) => self.move_to_point(p),
                None => self.move_to(0.0, 0.0),
            }
        } else {
            self
        }
    }
}
