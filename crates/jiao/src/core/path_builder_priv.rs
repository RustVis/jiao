// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

use crate::core::path_types::PathDirection;
use crate::core::point::Point;
use crate::core::rect::Rect;
use crate::core::rrect::{Corner, RRect};

pub const RECT_POINTS: usize = 4;
pub const OVAL_POINTS: usize = 4;
pub const RRECT_POINTS: usize = 8;

pub struct PointIter {
    points: Vec<Point>,
    start_index: usize,
    current: usize,
    advance: usize,
}

impl PointIter {
    pub fn new_rect(rect: &Rect, dir: PathDirection, start_index: usize) -> Self {
        let points = vec![
            Point::from_xy(rect.left(), rect.top()),
            Point::from_xy(rect.right(), rect.top()),
            Point::from_xy(rect.right(), rect.bottom()),
            Point::from_xy(rect.left(), rect.bottom()),
        ];
        Self::new(points, dir, start_index)
    }

    pub fn new_oval(oval: &Rect, dir: PathDirection, start_index: usize) -> Self {
        let cx = oval.center_x();
        let cy = oval.center_y();
        let points = vec![
            Point::from_xy(cx, oval.top()),
            Point::from_xy(oval.right(), cy),
            Point::from_xy(cx, oval.bottom()),
            Point::from_xy(oval.left(), cy),
        ];
        Self::new(points, dir, start_index)
    }

    pub fn new_rrect(rrect: &RRect, dir: PathDirection, start_index: usize) -> Self {
        let bounds = rrect.get_bounds();
        let left = bounds.left();
        let top = bounds.top();
        let right = bounds.right();
        let bottom = bounds.bottom();

        let points = vec![
            Point::from_xy(left + rrect.radii(Corner::UpperLeft).x(), top),
            Point::from_xy(right - rrect.radii(Corner::UpperRight).x(), top),
            Point::from_xy(right, top + rrect.radii(Corner::UpperRight).y()),
            Point::from_xy(right, bottom - rrect.radii(Corner::LowerRight).y()),
            Point::from_xy(right - rrect.radii(Corner::LowerRight).x(), bottom),
            Point::from_xy(left + rrect.radii(Corner::LowerLeft).x(), bottom),
            Point::from_xy(left, bottom - rrect.radii(Corner::LowerLeft).y()),
            Point::from_xy(left, top + rrect.radii(Corner::UpperLeft).y()),
        ];
        Self::new(points, dir, start_index)
    }

    fn new(points: Vec<Point>, dir: PathDirection, start_index: usize) -> Self {
        let current = start_index % points.len();
        let advance = match dir {
            PathDirection::Cw => 1,
            PathDirection::Ccw => points.len() - 1,
        };

        Self {
            points,
            start_index,
            current,
            advance,
        }
    }

    #[must_use]
    pub fn current(&self) -> Point {
        debug_assert!(self.current < self.points.len());
        self.points[self.current]
    }

    pub fn next(&mut self) -> Point {
        self.current = (self.current + self.advance) % self.points.len();
        self.current()
    }
}
