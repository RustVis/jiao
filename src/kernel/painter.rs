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
    fn add_path(&mut self, other: &Self);

    /// Attempts to add a straight line from the current point to the start of current path.
    ///
    /// If the shape has already been closed or has only one point, this function does nothing.
    fn close_path(&mut self);

    /// Move the starting point of path to the given (x, y) coordinates.
    fn move_to(&mut self, point: PointF);

    /// Connects the last point in the path to the (x, y) coordinates with a straight line.
    fn line_to(&mut self, point: PointF);

    /// Creates a path for a rectangle at position (x, y) with a size that is determined by width and height.
    fn rect_f64(&mut self, x: f64, y: f64, width: f64, height: f64);

    /// Creates a path for a rectangle.
    fn rect(&mut self, rect: &RectF) {
        self.rect_f64(rect.x(), rect.y(), rect.width(), rect.height());
    }
}
