// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

use crate::base::PointF;

pub trait PainterTrait {
    /// Makes a copy of current state of canvas and saves it on an internal stack.
    fn save(&mut self);

    /// Restores to the state saved by a preceding call to [`Self::save`] and removes
    /// that state from internal stack.
    fn restore(&mut self);

    fn clear_all(&mut self);

    /// Turns the current path into the current clipping region.
    fn clip(&mut self);

    /// Fills the current path with the current fill style.
    fn fill(&mut self);

    /// Strokes (outlines) the current path with the current stoke style.
    fn stroke(&mut self);

    /// Add a rotation to the transformation matrix.
    fn rotate(&mut self, angle: f64);

    /// Add a scaling transformation to the canvas units horizontally and/or vertically.
    fn scale(&mut self, x: f64, y: f64);

    /// Add a translation transformation to the current matrix.
    fn translate(&mut self, x: f64, y: f64);

    /// Starts a new path by emptying the list of sub-paths.
    fn begin_path(&mut self);

    /// Attempts to add a straight line from the current point to the start of current
    /// sub-path.
    ///
    /// If the shape has already been closed or has only one point, this function
    /// does nothing.
    fn close_path(&mut self);

    /// Add a straight line to the current sub-path by connecting the sub-path's
    /// last point to the specified (x, y) coordinates.
    fn line_to(&mut self, point: PointF);

    /// Begins a new sub-path at the point specified by the given (x, y) coordinates.
    fn move_to(&mut self, point: PointF);
}
