// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

use core::f64::consts::PI;
use serde::{Deserialize, Serialize};

use super::point::{Point, PointF};

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
            p1: Point::from(x1, y1),
            p2: Point::from(x2, y2),
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
        Point::from(
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

    /// Translates this line by the given point (`x`, `y`).
    pub fn translate(&mut self, x: i32, y: i32) {
        self.translate_point(Point::from(x, y))
    }

    /// Translates this line by the given `offset`.
    pub fn translate_point(&mut self, offset: Point) {
        self.p1 += offset;
        self.p2 += offset;
    }

    /// Returns this line translated by the given point (`x`, `y`).
    pub fn translated(&self, x: i32, y: i32) -> Self {
        self.translated_point(Point::from(x, y))
    }

    /// Returns this line translated by the given `offset`.
    pub fn translated_point(&self, offset: Point) -> Self {
        Line::from_points(self.p1 + offset, self.p2 + offset)
    }
}

/// The LineF struct provides a two-dimensional vector using floating point precision.
///
/// A LineF describes a finite length line (or a line segment) on a two-dimensional surface.
/// The start and end points of the line are specified using float point accuracy for coordinates.
///
/// The positions of the line's start and end points can be retrieved using
/// the p1(), x1(), y1(), p2(), x2(), and y2() functions.
///
/// The dx() and dy() functions return the horizontal and vertical components of the line.
///
/// Use is_null() to determine whether the LineF represents a valid line or a null line.
///
/// Finally, the line can be translated a given offset using the translate() function.
#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
pub struct LineF {
    p1: PointF,
    p2: PointF,
}

impl LineF {
    /// Constructs a null line.
    pub fn new() -> Self {
        Self::from(0.0, 0.0, 0.0, 0.0)
    }

    /// Constructs a line object that represents the line between (x1, y1) and (x2, y2).
    pub fn from(x1: f64, y1: f64, x2: f64, y2: f64) -> Self {
        Self {
            p1: PointF::from(x1, y1),
            p2: PointF::from(x2, y2),
        }
    }

    /// Constructs a line object that represents the line between p1 and p2.
    pub fn from_points(p1: PointF, p2: PointF) -> Self {
        Self { p1, p2 }
    }

    /// Returns a LineF with the given length and angle.
    ///
    /// The first point of the line will be on the origin.
    ///
    /// Positive values for the angles mean counter-clockwise while negative values
    /// mean the clockwise direction. Zero degrees is at the 3 o'clock position.
    pub fn from_polar(length: f64, angle: f64) -> Self {
        let angle_r = angle * 2.0 * PI / 360.0;
        LineF::from(0.0, 0.0, angle_r.cos() * length, -angle_r.sin() * length)
    }

    /// Returns the line's start point.
    pub fn p1(&self) -> PointF {
        self.p1
    }

    /// Returns the line's end point.
    pub fn p2(&self) -> PointF {
        self.p2
    }

    /// Returns the x-coordinate of the line's start point.
    pub fn x1(&self) -> f64 {
        self.p1.x()
    }

    /// Returns the x-coordinate of the line's end point.
    pub fn x2(&self) -> f64 {
        self.p2.x()
    }

    /// Returns the y-coordinate of the line's start point.
    pub fn y1(&self) -> f64 {
        self.p1.y()
    }

    /// Returns the y-coordinate of the line's end point.
    pub fn y2(&self) -> f64 {
        self.p2.y()
    }

    /// Returns the angle of the line in degrees.
    ///
    /// The return value will be in the range of values from 0.0 up to but not including 360.0.
    /// The angles are measured counter-clockwise from a point on the x-axis
    /// to the right of the origin (x > 0).
    pub fn angle(&self) -> f64 {
        unimplemented!()
    }

    /// Returns the angle (in degrees) from this line to the given line,
    /// taking the direction of the lines into account.
    ///
    /// If the lines do not intersect within their range, it is the intersection point
    /// of the extended lines that serves as origin.
    ///
    /// The returned value represents the number of degrees you need to add to this line
    /// to make it have the same angle as the given line, going counter-clockwise.
    pub fn angle_to(&self, line: &LineF) -> f64 {
        unimplemented!()
    }

    /// Returns the center point of this line.
    ///
    /// This is equivalent to (p1() + p2()) / 2, except it will never overflow.
    pub fn center(&self) -> PointF {
        PointF::from(
            (self.p1.x() + self.p2.x()) / 2.0,
            (self.p1.y() + self.p2.y()) / 2.0,
        )
    }

    /// Returns the horizontal component of the line's vector.
    pub fn dx(&self) -> f64 {
        self.p2.x() - self.p1.x()
    }

    /// Returns the vertical component of the line's vector.
    pub fn dy(&self) -> f64 {
        self.p2.y() - self.p1.y()
    }

    /// Returns a value indicating whether or not this line intersects with the given line.
    ///
    /// The actual intersection point is extracted to `intersection_point` (if the pointer is valid).
    /// If the lines are parallel, the intersection point is undefined.
    pub fn intersects() {
        unimplemented!()
    }

    /// Returns true if the line does not have distinct start and end points;
    /// otherwise returns false.
    pub fn is_null(&self) -> bool {
        self.p1 == self.p2
    }

    /// Returns the length of the line.
    pub fn length(&self) -> f64 {
        let dx = self.dx();
        let dy = self.dy();
        (dx * dx + dy * dy).sqrt()
    }

    /// Returns a line that is perpendicular to this line with the same starting point and length.
    pub fn normal_vector(&self) -> Self {
        unimplemented!()
    }

    /// Returns the point at the parameterized position specified by `t`.
    ///
    /// The function returns the line's start point if t = 0, and its end point if t = 1.
    pub fn point_at(&self, t: f64) -> PointF {
        unimplemented!()
    }

    /// Sets the starting point of this line to `p1`.
    pub fn set_p1(&mut self, p1: PointF) {
        self.p1 = p1;
    }

    /// Sets the end point of this line to `p2`.
    pub fn set_p2(&mut self, p2: PointF) {
        self.p2 = p2;
    }

    /// Sets the angle of the line to the given angle (in degrees).
    ///
    /// This will change the position of the second point of the line such that the line has the given angle.
    ///
    /// Positive values for the angles mean counter-clockwise while negative values
    /// mean the clockwise direction. Zero degrees is at the 3 o'clock position.
    pub fn set_angle(&mut self, angle: f64) {
        unimplemented!()
    }

    /// Sets the length of the line to the given length.
    ///
    /// LineF will move the end point - p2() - of the line to give the line its new length.
    ///
    /// A null line will not be rescaled.
    /// For non-null lines with very short lengths (represented by denormal floating-point values),
    /// results may be imprecise.
    pub fn set_length(&mut self, mut length: f64) {
        if self.is_null() {
            return;
        }
        debug_assert!(self.length() > 0.0);
        let vector = self.unit_vector();
        // In case it's not quite exactly 1.
        length /= vector.length();
        self.p2 = PointF::from(
            self.p1.x() + length * vector.dx(),
            self.p1.y() + length * vector.dy(),
        );
    }

    /// Sets this line to the start in `x1`, `y1` and end in `x2`, `y2`.
    pub fn set_line(&mut self, x1: f64, y1: f64, x2: f64, y2: f64) {
        self.p1.set(x1, y1);
        self.p2.set(x2, y2);
    }

    /// Sets the start point of this line to `p1` and the end point of this line to `p2`.
    pub fn set_points(&mut self, p1: PointF, p2: PointF) {
        self.p1 = p1;
        self.p2 = p2;
    }

    /// Returns an integer based copy of this line.
    ///
    /// Note that the returned line's start and end points are rounded to the nearest integer.
    pub fn to_line(&self) -> Line {
        Line::from_points(self.p1.to_point(), self.p2.to_point())
    }

    /// Translates this line by the given point (`x`, `y`).
    pub fn translate(&mut self, x: f64, y: f64) {
        self.translate_point(PointF::from(x, y))
    }

    /// Translates this line by the given `offset`.
    pub fn translate_point(&mut self, offset: PointF) {
        self.p1 += offset;
        self.p2 += offset;
    }

    /// Returns this line translated by the given point (`x`, `y`).
    pub fn translated(&self, x: f64, y: f64) -> Self {
        self.translated_point(PointF::from(x, y))
    }

    /// Returns this line translated by the given `offset`.
    pub fn translated_point(&self, offset: PointF) -> Self {
        LineF::from_points(self.p1 + offset, self.p2 + offset)
    }

    /// Returns the unit vector for this line.
    ///
    /// i.e a line starting at the same point as this line with a length of 1.0,
    /// provided the line is non-null.
    pub fn unit_vector(&self) -> Self {
        let x = self.dx();
        let y = self.dy();
        let hypot = (x * x + y * y).sqrt();
        LineF::from_points(
            self.p1(),
            PointF::from(self.p1.x() + x / hypot, self.p1.y() + y / hypot),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_polar() {
        let new_line = LineF::from_polar(8.4, 42.1);
        assert_eq!(
            new_line,
            LineF::from(0.0, 0.0, 6.232597064195178, -5.631583599253912)
        );
    }

    #[test]
    fn test_unit_vector() {
        let line = LineF::from(0.0, 0.0, 1.0, 1.0);
        let unit_line = line.unit_vector();
        assert_eq!(
            unit_line,
            LineF::from(0.0, 0.0, 0.7071067811865475, 0.7071067811865475)
        );
    }
}
