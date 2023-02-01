// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

use crate::base::{PointF, RectF};

#[allow(clippy::module_name_repetitions)]
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
    fn fill(&mut self, path: &dyn PathTrait);

    /// Strokes (outlines) the path with the current stoke style.
    fn stroke(&mut self, path: &dyn PathTrait);

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

    // Adds the given path to this path as a closed subpath.
    //fn add_path(&mut self, other: &Self);

    /// Attempts to add a straight line from the current point to the start of current path.
    ///
    /// If the shape has already been closed or has only one point, this function does nothing.
    fn close_path(&mut self);

    /// Move the starting point of path to the given (x, y) coordinates.
    fn move_to(&mut self, point: PointF);

    /// Connects the last point in the path to `point` with a straight line.
    fn line_to(&mut self, point: PointF);

    /// Creates a path for a rectangle.
    fn rect(&mut self, rect: &RectF);

    /// Adds a cubic Bézier curve to the path.
    ///
    /// It requires three points.
    /// The first two points are control points and the third one is the end point.
    /// The starting point is the last point in the current path.
    fn cubic_to(&mut self, p1: PointF, p2: PointF, end_point: PointF);

    /// Adds a quadratic Bézier curve between the current position and the given `end_point`
    /// with the `control_point`.
    fn quad_to(&mut self, control_point: PointF, end_point: PointF);

    /// Adds an arc to the path in clockwise direction.
    ///
    /// Note that `radius` must be non-negative.
    fn arc(&mut self, center: PointF, radius: f64, start_angle: f64, end_angle: f64);

    /// Adds a circular arc to the path with the given control points and radius,
    /// connected to the previous point by a straight line.
    ///
    /// Note that `radius` must be non-negative.
    fn arc_to(&mut self, p1: PointF, p2: PointF, radius: f64);

    /// Adds an elliptical arc to the path, in clockwise direction.
    fn ellipse(
        &mut self,
        center: PointF,
        radius_x: f64,
        radius_y: f64,
        start_angle: f64,
        end_angle: f64,
    );
}
