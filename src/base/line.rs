// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

use serde::{Deserialize, Serialize};

use super::point::Point;

/// The Line class provides a two-dimensional vector using integer precision.
///
/// A Line describes a finite length line (or a line segment) on a two-dimensional surface.
/// The start and end points of the line are specified using integer point accuracy for coordinates.
/// Use the `LineF` constructor to retrieve a floating point copy.
///
/// The positions of the line's start and end points can be retrieved using
/// the p1(), x1(), y1(), p2(), x2(), and y2() functions.
///
/// The dx() and dy() functions return the horizontal and vertical components of the line.
///
/// Use is_null() to determine whether the Line represents a valid line or a null line.
///
/// Finally, the line can be translated a given offset using the translate() function.
#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Line {
    p1: Point,
    p2: Point,
}

impl Line {
    /// Constructs a null line.
    pub fn new() -> Self {
        Self::from(0, 0, 0, 0)
    }

    /// Constructs a line object that represents the line between (x1, y1) and (x2, y2).
    pub fn from(x1: i32, y1: i32, x2: i32, y2: i32) -> Self {
        Self {
            p1: Point::new(x1, y1),
            p2: Point::new(x2, y2),
        }
    }

    /// Constructs a line object that represents the line between p1 and p2.
    pub fn from_points(p1: Point, p2: Point) -> Self {
        Self { p1, p2 }
    }

    /// Returns the line's start point.
    pub fn p1(&self) -> Point {
        self.p1
    }

    /// Returns the line's end point.
    pub fn p2(&self) -> Point {
        self.p2
    }

    /// Returns the x-coordinate of the line's start point.
    pub fn x1(&self) -> i32 {
        self.p1.x()
    }

    /// Returns the x-coordinate of the line's end point.
    pub fn x2(&self) -> i32 {
        self.p2.x()
    }

    /// Returns the y-coordinate of the line's start point.
    pub fn y1(&self) -> i32 {
        self.p1.y()
    }

    /// Returns the y-coordinate of the line's end point.
    pub fn y2(&self) -> i32 {
        self.p2.y()
    }

    /// Returns the center point of this line.
    ///
    /// This is equivalent to (p1() + p2()) / 2, except it will never overflow.
    pub fn center(&self) -> Point {
        Point::new(
            ((self.p1.x() as i64 + self.p2.x() as i64) / 2) as i32,
            ((self.p1.y() as i64 + self.p2.y() as i64) / 2) as i32,
        )
    }

    /// Returns the horizontal component of the line's vector.
    pub fn dx(&self) -> i32 {
        self.p2.x() - self.p1.x()
    }

    /// Returns the vertical component of the line's vector.
    pub fn dy(&self) -> i32 {
        self.p2.y() - self.p1.y()
    }

    /// Returns true if the line does not have distinct start and end points;
    /// otherwise returns false.
    pub fn is_null(&self) -> bool {
        self.p1 == self.p2
    }

    /// Sets the starting point of this line to `p1`.
    pub fn set_p1(&mut self, p1: Point) {
        self.p1 = p1;
    }

    /// Sets the end point of this line to `p2`.
    pub fn set_p2(&mut self, p2: Point) {
        self.p2 = p2;
    }

    /// Sets this line to the start in `x1`, `y1` and end in `x2`, `y2`.
    pub fn set_line(&mut self, x1: i32, y1: i32, x2: i32, y2: i32) {
        self.p1.set(x1, y1);
        self.p2.set(x2, y2);
    }

    /// Sets the start point of this line to `p1` and the end point of this line to `p2`.
    pub fn set_points(&mut self, p1: Point, p2: Point) {
        self.p1 = p1;
        self.p2 = p2;
    }

    /// Translates this line by the given `offset`.
    pub fn translate(&mut self, offset: Point) {
        self.p1 += offset;
        self.p2 += offset;
    }
}
