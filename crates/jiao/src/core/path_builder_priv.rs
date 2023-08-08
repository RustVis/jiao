// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

use crate::core::path_types::PathDirection;
use crate::core::point::Point;
use crate::core::rect::Rect;
use crate::core::rrect::{Corner, RRect};

const RECT_POINTS: usize = 4;

pub struct RectPointIter {
    points: [Point; RECT_POINTS],
    start_index: usize,
    current: usize,
    advance: usize,
}

impl RectPointIter {
    #[must_use]
    pub const fn new(rect: &Rect, dir: PathDirection, start_index: usize) -> Self {
        let current = start_index % RECT_POINTS;
        let advance = match dir {
            PathDirection::Cw => 1,
            PathDirection::Ccw => RECT_POINTS - 1,
        };
        let points = [
            Point::from_xy(rect.left(), rect.top()),
            Point::from_xy(rect.right(), rect.top()),
            Point::from_xy(rect.right(), rect.bottom()),
            Point::from_xy(rect.left(), rect.bottom()),
        ];

        Self {
            points,
            start_index,
            current,
            advance,
        }
    }

    #[must_use]
    pub fn current(&self) -> Point {
        debug_assert!(self.current < RECT_POINTS);
        self.points[self.current]
    }

    pub fn next(&mut self) -> Point {
        self.current = (self.current + self.advance) % RECT_POINTS;
        self.current()
    }
}

const OVAL_POINTS: usize = 4;

pub struct OvalPointIter {
    points: [Point; OVAL_POINTS],
    start_index: usize,
    current: usize,
    advance: usize,
}

impl OvalPointIter {
    #[must_use]
    pub fn new(oval: &Rect, dir: PathDirection, start_index: usize) -> Self {
        let current = start_index % OVAL_POINTS;
        let advance = match dir {
            PathDirection::Cw => 1,
            PathDirection::Ccw => OVAL_POINTS - 1,
        };
        let cx = oval.center_x();
        let cy = oval.center_y();
        let points = [
            Point::from_xy(cx, oval.top()),
            Point::from_xy(oval.right(), cy),
            Point::from_xy(cx, oval.bottom()),
            Point::from_xy(oval.left(), cy),
        ];

        Self {
            points,
            start_index,
            current,
            advance,
        }
    }

    #[must_use]
    pub fn current(&self) -> Point {
        debug_assert!(self.current < OVAL_POINTS);
        self.points[self.current]
    }

    pub fn next(&mut self) -> Point {
        self.current = (self.current + self.advance) % OVAL_POINTS;
        self.current()
    }
}

const RRECT_POINTS: usize = 8;

pub struct RRectPointIter {
    points: [Point; RRECT_POINTS],
    start_index: usize,
    current: usize,
    advance: usize,
}

impl RRectPointIter {
    #[must_use]
    pub fn new(rrect: &RRect, dir: PathDirection, start_index: usize) -> Self {
        let current = start_index % RRECT_POINTS;
        let advance = match dir {
            PathDirection::Cw => 1,
            PathDirection::Ccw => RRECT_POINTS - 1,
        };
        let bounds = rrect.get_bounds();
        let left = bounds.left();
        let top = bounds.top();
        let right = bounds.right();
        let bottom = bounds.bottom();

        let points = [
            Point::from_xy(left + rrect.radii(Corner::UpperLeft).x(), top),
            Point::from_xy(right - rrect.radii(Corner::UpperRight).x(), top),
            Point::from_xy(right, top + rrect.radii(Corner::UpperRight).y()),
            Point::from_xy(right, bottom - rrect.radii(Corner::LowerRight).y()),
            Point::from_xy(right - rrect.radii(Corner::LowerRight).x(), bottom),
            Point::from_xy(left + rrect.radii(Corner::LowerLeft).x(), bottom),
            Point::from_xy(left, bottom - rrect.radii(Corner::LowerLeft).y()),
            Point::from_xy(left, top + rrect.radii(Corner::UpperLeft).y()),
        ];

        Self {
            points,
            start_index,
            current,
            advance,
        }
    }

    #[must_use]
    pub fn current(&self) -> Point {
        debug_assert!(self.current < RRECT_POINTS);
        self.points[self.current]
    }

    pub fn next(&mut self) -> Point {
        self.current = (self.current + self.advance) % RRECT_POINTS;
        self.current()
    }
}
