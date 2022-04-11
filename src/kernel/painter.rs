// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

use crate::base::{PointF, RectF};
use crate::platforms;

pub trait PainterTrait {
    /// Makes a copy of current state of canvas and saves it on an internal stack.
    fn save(&mut self);

    /// Restores to the state saved by a preceding call to [`Self::save`] and removes
    /// that state from internal stack.
    fn restore(&mut self);

    fn clear_all(&mut self);

    /// Turns the current path into the current clipping region.
    fn clip(&mut self);

    /// Fills the path with the current fill style.
    fn fill(&mut self, path: &platforms::Path);

    /// Strokes (outlines) the path with the current stoke style.
    fn stroke(&mut self, path: &platforms::Path);

    /// Add a rotation to the transformation matrix.
    fn rotate(&mut self, angle: f64);

    /// Add a scaling transformation to the canvas units horizontally and/or vertically.
    fn scale(&mut self, x: f64, y: f64);

    /// Add a translation transformation to the current matrix.
    fn translate(&mut self, x: f64, y: f64);
}

pub trait PathTrait {
    /// Clears the path elements stored.
    fn clear(&mut self);

    /// Adds the given path to this path as a closed subpath.
    fn add_path(&mut self, other: &Self);

    /// Attempts to add a straight line from the current point to the start of current path.
    ///
    /// If the shape has already been closed or has only one point, this function does nothing.
    fn close_path(&mut self);

    /// Move the starting point of path to the given (x, y) coordinates.
    fn move_to(&mut self, point: PointF);

    /// Move the starting point of path to the given (x, y) coordinates.
    ///
    /// Overload of `move_to()`.
    #[inline]
    fn move_to_f64(&mut self, x: f64, y: f64) {
        self.move_to(PointF::from(x, y));
    }

    /// Connects the last point in the path to `point` with a straight line.
    fn line_to(&mut self, point: PointF);

    /// Connects the last point in the path to the (x, y) coordinates with a straight line.
    ///
    /// Overload of `line_to()`.
    #[inline]
    fn line_to_f64(&mut self, x: f64, y: f64) {
        self.line_to(PointF::from(x, y));
    }

    /// Creates a path for a rectangle.
    ///
    /// Overload of `rect()`.
    fn rect_f64(&mut self, x: f64, y: f64, width: f64, height: f64);

    /// Creates a path for a rectangle.
    #[inline]
    fn rect(&mut self, rect: &RectF) {
        self.rect_f64(rect.x(), rect.y(), rect.width(), rect.height());
    }

    /// Adds a cubic Bézier curve to the path.
    ///
    /// It requires three points.
    /// The first two points are control points and the third one is the end point.
    /// The starting point is the last point in the current path.
    #[inline]
    fn cubic_to(&mut self, p1: PointF, p2: PointF, end_point: PointF) {
        self.cubic_to_f64(p1.x(), p1.y(), p2.x(), p2.y(), end_point.x(), end_point.y());
    }

    /// Adds a cubic Bézier curve to the path.
    ///
    /// Overload of `cubic_to()`.
    fn cubic_to_f64(&mut self, cp1x: f64, cp1y: f64, cp2x: f64, cp2y: f64, x: f64, y: f64);

    /// Adds a quadratic Bézier curve between the current position and the given `end_point`
    /// with the control point specified by `c`.
    #[inline]
    fn quad_to(&mut self, c: PointF, end_point: PointF) {
        self.quad_to_f64(c.x(), c.y(), end_point.x(), end_point.y());
    }

    /// Adds a quadratic Bézier curve to the current path.
    ///
    /// Overload of `quad_to()`.
    fn quad_to_f64(&mut self, cpx: f64, cpy: f64, x: f64, y: f64);

    /// Adds an arc to the path in clockwise direction.
    ///
    /// Note that `radius` must be non-negative.
    #[inline]
    fn arc(&mut self, center: PointF, radius: f64, start_angle: f64, end_angle: f64) {
        self.arc_f64(center.x(), center.y(), radius, start_angle, end_angle);
    }

    /// Adds an arc to the path.
    ///
    /// Overload of `arc()`.
    fn arc_f64(&mut self, x: f64, y: f64, radius: f64, start_angle: f64, end_angle: f64);

    /// Adds a circular arc to the path with the given control points and radius,
    /// connected to the previous point by a straight line.
    ///
    /// Note that `radius` must be non-negative.
    #[inline]
    fn arc_to(&mut self, p1: PointF, p2: PointF, radius: f64) {
        self.arc_to_f64(p1.x(), p1.y(), p2.x(), p2.y(), radius);
    }

    /// Adds a circular arc to the path with the given control points and radius,
    /// connected to the previous point by a straight line.
    ///
    /// Overload of `arc_to()`.
    fn arc_to_f64(&mut self, x1: f64, y1: f64, x2: f64, y2: f64, radius: f64);

    /// Adds an elliptical arc to the path, in clockwise direction.
    #[inline]
    fn ellipse(
        &mut self,
        center: PointF,
        radius_x: f64,
        radius_y: f64,
        rotation: f64,
        start_angle: f64,
        end_angle: f64,
    ) {
        self.ellipse_f64(
            center.x(),
            center.y(),
            radius_x,
            radius_y,
            rotation,
            start_angle,
            end_angle,
        );
    }

    /// Adds an elliptical arc to the path, in clockwise direction.
    ///
    /// Overload of `ellipse()`.
    fn ellipse_f64(
        &mut self,
        center_x: f64,
        center_y: f64,
        radius_x: f64,
        radius_y: f64,
        rotation: f64,
        start_angle: f64,
        end_angle: f64,
    );
}
