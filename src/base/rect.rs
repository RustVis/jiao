// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

use core::ops;
use serde::{Deserialize, Serialize};

use super::margins::{Margins, MarginsF};
use super::point::{Point, PointF};
use super::size::{Size, SizeF};

/// The Rect struct defines a rectangle in the plane using integer precision.
///
/// A rectangle is normally expressed as a top-left corner and a size.
/// The size (width and height) of a Rect is always equivalent to the mathematical rectangle
/// that forms the basis for its rendering.
///
/// A Rect can be constructed with a set of left, top, width and height integers,
/// or from a Point and a Size.
#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Rect {
    x1: i32,
    y1: i32,
    x2: i32,
    y2: i32,
}

impl Rect {
    /// Constructs a null rectangle.
    pub fn new() -> Self {
        Self::from(0, 0, 0, 0)
    }

    /// Constructs a rectangle with `left`, `top` as its top-left corner and the given `width` and `height`.
    pub fn from(left: i32, top: i32, width: i32, height: i32) -> Self {
        Self {
            x1: left,
            y1: top,
            x2: left + width,
            y2: top + height,
        }
    }

    /// Constructs a rectangle with the given `top_left` corner and the given `size`.
    pub fn from_size(top_left: Point, size: Size) -> Self {
        Self {
            x1: top_left.x(),
            y1: top_left.y(),
            x2: top_left.x() + size.width(),
            y2: top_left.y() + size.height(),
        }
    }

    /// Constructs a rectangle with the given `top_left` and `bottom_right` corners.
    pub fn from_points(top_left: Point, bottom_right: Point) -> Self {
        Self {
            x1: top_left.x(),
            y1: top_left.y(),
            x2: bottom_right.x(),
            y2: bottom_right.y(),
        }
    }

    /// Adds `dx1`, `dy1`, `dx2` and `dy2` respectively to the existing coordinates of the rectangle.
    pub fn adjust(&mut self, dx1: i32, dy1: i32, dx2: i32, dy2: i32) {
        self.x1 += dx1;
        self.y1 += dy1;
        self.x2 += dx2;
        self.y2 += dy2;
    }

    /// Returns a new rectangle with `dx1`, `dy1`, `dx2` and `dy2` added respectively
    /// to the existing coordinates of this rectangle.
    pub fn adjusted(&self, dx1: i32, dy1: i32, dx2: i32, dy2: i32) -> Self {
        Self::from(self.x1 + dx1, self.y1 + dy1, self.x2 + dx2, self.y2 + dy2)
    }

    /// Returns the y-coordinate of the rectangle's bottom edge.
    pub fn bottom(&self) -> i32 {
        self.y2
    }

    /// Returns the position of the rectangle's bottom-left corner.
    pub fn bottom_left(&self) -> Point {
        Point::from(self.x1, self.y2)
    }

    /// Returns the position of the rectangle's bottom-right corner.
    pub fn bottom_right(&self) -> Point {
        Point::from(self.x2, self.y2)
    }

    /// Returns the center point of the rectangle.
    pub fn center(&self) -> Point {
        // cast avoids overflow on addition
        Point::from(
            ((self.x1 as i64 + self.x2 as i64) / 2) as i32,
            ((self.y1 as i64 + self.y2 as i64) / 2) as i32,
        )
    }

    /// Returns true if the point (`x`, `y`) is inside this rectangle, otherwise returns false.
    pub fn contains(&self, x: i32, y: i32) -> bool {
        self.contains_helper(x, y, false)
    }

    /// Returns true if the point (`x`, `y`) is inside this rectangle (not on the edge),
    /// otherwise returns false.
    pub fn contains_proper(&self, x: i32, y: i32) -> bool {
        self.contains_helper(x, y, true)
    }

    fn contains_helper(&self, x: i32, y: i32, proper: bool) -> bool {
        let l;
        let r;
        // TODO(Shaohua): Replace with self.x2 < self.x1
        if self.x2 < self.x1 - 1 {
            l = self.x2;
            r = self.x1;
        } else {
            l = self.x1;
            r = self.x2;
        }
        if proper {
            if x <= l || x >= r {
                return false;
            }
        } else {
            if x < l || x > r {
                return false;
            }
        }

        let t;
        let b;
        if self.y2 < self.y1 - 1 {
            t = self.y2;
            b = self.y1;
        } else {
            t = self.y1;
            b = self.y2;
        }

        if proper {
            if y <= t || y >= b {
                return false;
            }
        } else {
            if y < t || y > b {
                return false;
            }
        }
        return true;
    }

    /// Returns true if the given point is inside or on the edge of the rectangle,
    /// otherwise returns false.
    pub fn contains_point(&self, point: &Point) -> bool {
        self.contains(point.x(), point.y())
    }

    /// Returns true if the given point is inside of the rectangle,
    /// otherwise returns false (including on the edges).
    pub fn contains_point_proper(&self, point: &Point) -> bool {
        self.contains_proper(point.x(), point.y())
    }

    /// Returns true if the given rectangle is inside this rectangle, including
    /// on the edge, otherwise returns false.
    pub fn contains_rect(&self, rect: &Self) -> bool {
        self.contains_rect_helper(rect, false)
    }

    /// Returns true if the given rectangle is inside this rectangle,
    /// otherwise returns false.
    pub fn contains_rect_proper(&self, rect: &Self) -> bool {
        self.contains_rect_helper(rect, true)
    }

    fn contains_rect_helper(&self, rect: &Self, proper: bool) -> bool {
        if self.is_null() || rect.is_null() {
            return false;
        }

        let mut l1 = self.x1;
        let mut r1 = self.x1;
        // TODO(Shaohua): Replace with self.x2 < self.x1
        if self.x2 - self.x1 + 1 < 0 {
            l1 = self.x2;
        } else {
            r1 = self.x2;
        }

        let mut l2 = rect.x1;
        let mut r2 = rect.x1;
        if rect.x2 - rect.x1 + 1 < 0 {
            l2 = rect.x2;
        } else {
            r2 = rect.x2;
        }

        if proper {
            if l2 <= l1 || r2 >= r1 {
                return false;
            }
        } else {
            if l2 < l1 || r2 > r1 {
                return false;
            }
        }

        let mut t1 = self.y1;
        let mut b1 = self.y1;
        if self.y2 - self.y1 + 1 < 0 {
            t1 = self.y2;
        } else {
            b1 = self.y2;
        }

        let mut t2 = rect.y1;
        let mut b2 = rect.y1;
        if rect.y2 - rect.y1 + 1 < 0 {
            t2 = rect.y2;
        } else {
            b2 = rect.y2;
        }

        if proper {
            if t2 <= t1 || b2 >= b1 {
                return false;
            }
        } else {
            if t2 < t1 || b2 > b1 {
                return false;
            }
        }

        return true;
    }

    /// Extracts the position of the rectangle's top-left corner to `x1` and `y1`,
    /// and the position of the bottom-right corner to `x2` and `y2`.
    pub fn get_coords(&self, x1: &mut i32, y1: &mut i32, x2: &mut i32, y2: &mut i32) {
        *x1 = self.x1;
        *y1 = self.y1;
        *x2 = self.x2;
        *y2 = self.y2;
    }

    /// Extracts the position of the rectangle's top-left corner to `x` and `y`, and
    /// its dimensions to `width` and `height`.
    pub fn get_rect(&self, x: &mut i32, y: &mut i32, width: &mut i32, height: &mut i32) {
        *x = self.x1;
        *y = self.y1;
        *width = self.x2 - self.x1;
        *height = self.y2 - self.y1;
    }

    /// Returns the height of the rectangle.
    pub fn height(&self) -> i32 {
        self.y2 - self.y1
    }

    /// Returns the intersection of this rectangle and the given `rectangle`.
    pub fn intersected(&self, rectangle: &Self) -> Self {
        self & rectangle
    }

    /// Returns true if this rectangle intersects with the given `rectangle`
    /// (i.e., there is at least one pixel that is within both rectangles),
    /// otherwise returns false.
    pub fn intersects(&self, rectangle: &Self) -> bool {
        if self.is_null() || rectangle.is_null() {
            return false;
        }

        let mut l1 = self.x1;
        let mut r1 = self.x1;
        // TODO(Shaohua): Replace
        if self.x2 - self.x1 + 1 < 0 {
            l1 = self.x2;
        } else {
            r1 = self.x2;
        }

        let mut l2 = rectangle.x1;
        let mut r2 = rectangle.x1;
        if rectangle.x2 - rectangle.x1 + 1 < 0 {
            l2 = rectangle.x2;
        } else {
            r2 = rectangle.x2;
        }

        if l1 > r2 || l2 > r1 {
            return false;
        }

        let mut t1 = self.y1;
        let mut b1 = self.y1;
        if self.y2 - self.y1 + 1 < 0 {
            t1 = self.y2;
        } else {
            b1 = self.y2;
        }

        let mut t2 = rectangle.y1;
        let mut b2 = rectangle.y1;
        if rectangle.y2 - rectangle.y1 + 1 < 0 {
            t2 = rectangle.y2;
        } else {
            b2 = rectangle.y2;
        }

        if t1 > b2 || t2 > b1 {
            return false;
        }
        return true;
    }

    /// Returns true if the rectangle is empty, otherwise returns false.
    ///
    /// An empty rectangle has a left() >= right() or top() >= bottom().
    /// An empty rectangle is not valid (i.e., is_empty() == !is_valid()).
    ///
    /// Use the `normalized()` function to retrieve a rectangle where the corners are swapped.
    pub fn is_empty(&self) -> bool {
        self.x1 >= self.x2 || self.y1 >= self.y2
    }

    /// Returns true if the rectangle is a null rectangle, otherwise returns false.
    ///
    /// A null rectangle has both the width and the height set to 0
    /// (i.e., right() == left() and bottom() == top()).
    ///
    /// A null rectangle is also empty, and hence is not valid.
    pub fn is_null(&self) -> bool {
        self.x1 == self.x2 && self.y1 == self.y2
    }

    /// Returns true if the rectangle is valid, otherwise returns false.
    ///
    /// A valid rectangle has a left() < right() and top() < bottom().
    /// Note that non-trivial operations like intersections are not defined for invalid rectangles.
    ///
    /// A valid rectangle is not empty (i.e., is_valid() == !is_empty()).
    pub fn is_valid(&self) -> bool {
        self.x1 < self.x2 && self.y1 < self.y2
    }

    /// Returns the x-coordinate of the rectangle's left edge. Equivalent to `x()`.
    pub fn left(&self) -> i32 {
        self.x1
    }

    /// Returns a rectangle grown by the `margins`.
    pub fn margins_added(&self, margins: &Margins) -> Self {
        Self::from(
            self.x1 - margins.left(),
            self.y1 - margins.top(),
            self.x2 + margins.right(),
            self.y2 + margins.bottom(),
        )
    }

    /// Removes the `margins` from the rectangle, shrinking it.
    pub fn margins_removed(&self, margins: &Margins) -> Self {
        Self::from(
            self.x1 + margins.left(),
            self.y1 + margins.top(),
            self.x2 - margins.right(),
            self.y2 - margins.bottom(),
        )
    }

    /// Moves the rectangle vertically, leaving the rectangle's bottom edge
    /// at the given `y` coordinate.
    ///
    /// The rectangle's size is unchanged.
    pub fn move_bottom(&mut self, y: i32) {
        self.y1 += y - self.y2;
        self.y2 = y;
    }

    /// Moves the rectangle, leaving the bottom-left corner at the given `position`.
    ///
    /// The rectangle's size is unchanged.
    pub fn move_bottom_left(&mut self, position: &Point) {
        self.move_left(position.x());
        self.move_bottom(position.y());
    }

    /// Moves the rectangle, leaving the bottom-right corner at the given `position`.
    ///
    /// The rectangle's size is unchanged.
    pub fn move_bottom_right(&mut self, position: &Point) {
        self.move_right(position.x());
        self.move_bottom(position.y());
    }

    /// Moves the rectangle, leaving the center point at the given `position`.
    ///
    /// The rectangle's size is unchanged.
    pub fn move_center(&mut self, position: &Point) {
        let width = self.x2 - self.x1;
        let height = self.y2 - self.y1;
        self.x1 = position.x() - width / 2;
        self.y1 = position.y() - width / 2;
        self.x2 = self.x1 + width;
        self.y2 = self.y1 + height;
    }

    /// Moves the rectangle horizontally, leaving the rectangle's left edge at the given `x` coordinate.
    ///
    /// The rectangle's size is unchanged.
    pub fn move_left(&mut self, x: i32) {
        self.x2 += x - self.x1;
        self.x1 = x;
    }

    /// Moves the rectangle horizontally, leaving the rectangle's right edge at the given `x` coordinate.
    ///
    /// The rectangle's size is unchanged.
    pub fn move_right(&mut self, x: i32) {
        self.x1 += x - self.x2;
        self.x2 = x;
    }

    /// Moves the rectangle, leaving the top-left corner at the given position (`x`, `y`).
    ///
    /// The rectangle's size is unchanged.
    pub fn move_to(&mut self, x: i32, y: i32) {
        self.x2 += x - self.x1;
        self.y2 += y - self.y1;
        self.x1 = x;
        self.y1 = y;
    }

    /// Moves the rectangle, leaving the top-left corner at the given `position`.
    pub fn move_to_point(&mut self, point: &Point) {
        self.x2 += point.x() - self.x1;
        self.y2 += point.y() - self.y1;
        self.x1 = point.x();
        self.y1 = point.y();
    }

    /// Moves the rectangle vertically, leaving the rectangle's top edge at the given `y` coordinate.
    ///
    /// The rectangle's size is unchanged.
    pub fn move_top(&mut self, y: i32) {
        self.y2 += y - self.y1;
        self.y1 = y;
    }

    /// Moves the rectangle, leaving the top-left corner at the given `position`.
    ///
    /// The rectangle's size is unchanged.
    pub fn move_top_left(&mut self, position: &Point) {
        self.move_left(position.x());
        self.move_top(position.y());
    }

    /// Moves the rectangle, leaving the top-right corner at the given `position`.
    ///
    /// The rectangle's size is unchanged.
    pub fn move_top_right(&mut self, position: &Point) {
        self.move_right(position.x());
        self.move_top(position.y());
    }

    /// Returns a normalized rectangle; i.e., a rectangle that has a non-negative width and height.
    ///
    /// If width() < 0 the function swaps the left and right corners, and it swaps
    /// the top and bottom corners if height() < 0.
    pub fn normalized(&self) -> Self {
        let mut r = Rect::new();
        // TODO(Shaohua): Replace
        if self.x2 < self.x1 - 1 {
            // swap bad x values
            r.x1 = self.x2;
            r.x2 = self.x1;
        } else {
            r.x1 = self.x1;
            r.x2 = self.x2;
        }
        if self.y2 < self.y1 - 1 {
            // swap bad y values
            r.y1 = self.y2;
            r.y2 = self.y1;
        } else {
            r.y1 = self.y1;
            r.y2 = self.y2;
        }
        return r;
    }

    /// Returns the x-coordinate of the rectangle's right edge.
    pub fn right(&self) -> i32 {
        self.x2
    }

    /// Sets the bottom edge of the rectangle to the given `y` coordinate.
    ///
    /// May change the height, but will never change the top edge of the rectangle.
    pub fn set_bottom(&mut self, y: i32) {
        self.y2 = y;
    }

    /// Set the bottom-left corner of the rectangle to the given `position`.
    ///
    /// May change the size, but will never change the top-right corner of the rectangle.
    pub fn set_bottom_left(&mut self, position: &Point) {
        self.x1 = position.x();
        self.y2 = position.y();
    }

    /// Set the bottom-right corner of the rectangle to the given `position`.
    ///
    /// May change the size, but will never change the top-left corner of the rectangle.
    pub fn set_bottom_right(&mut self, position: &Point) {
        self.x2 = position.x();
        self.y2 = position.y();
    }

    /// Sets the coordinates of the rectangle's top-left corner to (`x1`, `y1`),
    /// and the coordinates of its bottom-right corner to (`x2`, `y2`).
    pub fn set_coords(&mut self, x1: i32, y1: i32, x2: i32, y2: i32) {
        self.x1 = x1;
        self.y1 = y1;
        self.x2 = x2;
        self.y2 = y2;
    }

    /// Sets the height of the rectangle to the given `height`.
    ///
    /// The bottom edge is changed, but not the top one.
    pub fn set_height(&mut self, height: i32) {
        self.y2 = self.y1 + height;
    }

    /// Sets the left edge of the rectangle to the given `x` coordinate.
    ///
    /// May change the width, but will never change the right edge of the rectangle.
    pub fn set_left(&mut self, x: i32) {
        self.x1 = x;
    }

    /// Sets the coordinates of the rectangle's top-left corner to (`x`, `y`),
    /// and its size to the given `width` and `height`.
    pub fn set_rect(&mut self, x: i32, y: i32, width: i32, height: i32) {
        self.x1 = x;
        self.y1 = y;
        self.x2 = x + width;
        self.y2 = y + height;
    }

    /// Sets the right edge of the rectangle to the given `x` coordinate.
    ///
    /// May change the width, but will never change the left edge of the rectangle.
    pub fn set_right(&mut self, x: i32) {
        self.x2 = x;
    }

    /// Sets the size of the rectangle to the given `size`.
    ///
    /// The top-left corner is not moved.
    pub fn set_size(&mut self, size: &Size) {
        self.x2 = self.x1 + size.width();
        self.y2 = self.y1 + size.height();
    }

    /// Sets the top edge of the rectangle to the given `y` coordinate.
    ///
    /// May change the height, but will never change the bottom edge of the rectangle.
    pub fn set_top(&mut self, y: i32) {
        self.y1 = y;
    }

    /// Set the top-left corner of the rectangle to the given `position`.
    ///
    /// May change the size, but will never change the bottom-right corner of the rectangle.
    pub fn set_top_left(&mut self, position: &Point) {
        self.x1 = position.x();
        self.y1 = position.y();
    }

    /// Set the top-right corner of the rectangle to the given `position`.
    ///
    /// May change the size, but will never change the bottom-left corner of the rectangle.
    pub fn set_top_right(&mut self, position: &Point) {
        self.x2 = position.x();
        self.y1 = position.y();
    }

    /// Sets the width of the rectangle to the given `width`.
    ///
    /// The right edge is changed, but not the left one.
    pub fn set_width(&mut self, width: i32) {
        self.x2 = self.x1 + width;
    }

    /// Sets the left edge of the rectangle to the given `x` coordinate.
    ///
    /// May change the width, but will never change the right edge of the rectangle.
    ///
    /// Equivalent to `set_left()`.
    pub fn set_x(&mut self, x: i32) {
        self.x1 = x;
    }

    /// Sets the top edge of the rectangle to the given `y` coordinate.
    ///
    /// May change the height, but will never change the bottom edge of the rectangle.
    ///
    /// Equivalent to `set_top()`.
    pub fn set_y(&mut self, y: i32) {
        self.y1 = y;
    }

    /// Returns the size of the rectangle.
    pub fn size(&self) -> Size {
        Size::from(self.width(), self.height())
    }

    /// Returns the y-coordinate of the rectangle's top edge.
    ///
    /// Equivalent to `y()`.
    pub fn top(&self) -> i32 {
        self.y1
    }

    /// Returns the position of the rectangle's top-left corner.
    pub fn top_left(&self) -> Point {
        Point::from(self.x1, self.y1)
    }

    /// Returns the position of the rectangle's top-right corner.
    pub fn top_right(&self) -> Point {
        Point::from(self.x2, self.y1)
    }

    /// Moves the rectangle `dx` along the x axis and `dy` along the y axis,
    /// relative to the current position.
    ///
    /// Positive values move the rectangle to the right and down.
    pub fn translate(&mut self, dx: i32, dy: i32) {
        self.x1 += dx;
        self.y1 += dy;
        self.x2 += dx;
        self.y2 += dy;
    }

    /// Moves the rectangle `offset.x()` along the x axis and `offset.y()` along the y axis,
    /// relative to the current position.
    pub fn translate_point(&mut self, offset: &Point) {
        self.x1 += offset.x();
        self.y1 += offset.y();
        self.x2 += offset.x();
        self.y2 += offset.y();
    }

    /// Returns a copy of the rectangle that is translated `dx` along the x axis
    /// and `dy` along the y axis, relative to the current position.
    ///
    /// Positive values move the rectangle to the right and down.
    pub fn translated(&self, dx: i32, dy: i32) -> Self {
        Self::from(self.x1 + dx, self.y1 + dy, self.x2 + dx, self.y2 + dy)
    }

    /// Returns a copy of the rectangle that is translated `offset.x()` along the x axis
    /// and `offset.y()` along the y axis, relative to the current position.
    pub fn translated_point(&self, offset: &Point) -> Self {
        Self::from(
            self.x1 + offset.x(),
            self.y1 + offset.y(),
            self.x2 + offset.x(),
            self.y2 + offset.y(),
        )
    }

    /// Returns a copy of the rectangle that has its width and height exchanged:
    pub fn transposed(&self) -> Self {
        Self::from_size(self.top_left(), self.size().transposed())
    }

    /// Returns the bounding rectangle of this rectangle and the given `rectangle`.
    pub fn united(&self, rectangle: &Self) -> Self {
        self | rectangle
    }

    /// Returns the width of the rectangle.
    pub fn width(&self) -> i32 {
        self.x2 - self.x1
    }

    /// Returns the x-coordinate of the rectangle's left edge.
    ///
    /// Equivalent to `left()`.
    pub fn x(&self) -> i32 {
        self.x1
    }

    /// Returns the y-coordinate of the rectangle's top edge.
    ///
    /// Equivalent to `top()`.
    pub fn y(&self) -> i32 {
        self.y1
    }
}

impl ops::AddAssign<&Margins> for Rect {
    /// Adds the margins to the rectangle, growing it.
    fn add_assign(&mut self, margins: &Margins) {
        self.x1 -= margins.left();
        self.y1 -= margins.top();
        self.x2 += margins.right();
        self.y2 += margins.bottom();
    }
}

impl ops::Add<&Rect> for &Margins {
    type Output = Rect;
    fn add(self, rectangle: &Rect) -> Rect {
        rectangle.margins_added(self)
    }
}

impl ops::SubAssign<&Margins> for Rect {
    /// Returns a rectangle shrunk by the margins.
    fn sub_assign(&mut self, margins: &Margins) {
        self.x1 += margins.left();
        self.y1 += margins.top();
        self.x2 -= margins.right();
        self.y2 -= margins.bottom();
    }
}

impl ops::Sub<&Rect> for &Margins {
    type Output = Rect;
    fn sub(self, rectangle: &Rect) -> Rect {
        rectangle.margins_removed(self)
    }
}

impl ops::BitAnd<&Rect> for &Rect {
    type Output = Rect;

    /// Returns the intersection of this rectangle and the given `rectangle`.
    ///
    /// Returns an empty rectangle if there is no intersection.
    fn bitand(self, rectangle: &Rect) -> Rect {
        if self.is_null() || rectangle.is_null() {
            return Rect::new();
        }

        let mut l1 = self.x1;
        let mut r1 = self.x1;
        // TODO(Shaohua): Replace
        if self.x2 - self.x1 + 1 < 0 {
            l1 = self.x2;
        } else {
            r1 = self.x2;
        }

        let mut l2 = rectangle.x1;
        let mut r2 = rectangle.x1;
        if rectangle.x2 - rectangle.x1 + 1 < 0 {
            l2 = rectangle.x2;
        } else {
            r2 = rectangle.x2;
        }

        if l1 > r2 || l2 > r1 {
            return Rect::new();
        }

        let mut t1 = self.y1;
        let mut b1 = self.y1;
        if self.y2 - self.y1 + 1 < 0 {
            t1 = self.y2;
        } else {
            b1 = self.y2;
        }

        let mut t2 = rectangle.y1;
        let mut b2 = rectangle.y1;
        if rectangle.y2 - rectangle.y1 + 1 < 0 {
            t2 = rectangle.y2;
        } else {
            b2 = rectangle.y2;
        }

        if t1 > b2 || t2 > b1 {
            return Rect::new();
        }

        return Rect::from(l1.max(l2), r1.min(r2), t1.max(t2), b1.min(b2));
    }
}

impl ops::BitAndAssign<&Rect> for Rect {
    /// Intersects this rectangle with the given `rectangle`.
    fn bitand_assign(&mut self, rectangle: &Rect) {
        let new_rect = rectangle & self;
        *self = new_rect;
    }
}

impl ops::BitOr<&Rect> for &Rect {
    type Output = Rect;

    /// Returns the bounding rectangle of this rectangle and the given `rectangle`.
    fn bitor(self, rectangle: &Rect) -> Rect {
        if self.is_null() {
            return rectangle.clone();
        }
        if rectangle.is_null() {
            return self.clone();
        }

        let mut l1 = self.x1;
        let mut r1 = self.x1;
        // TODO(Shaohua): Replace
        if self.x2 - self.x1 + 1 < 0 {
            l1 = self.x2;
        } else {
            r1 = self.x2;
        }

        let mut l2 = rectangle.x1;
        let mut r2 = rectangle.x1;
        if rectangle.x2 - rectangle.x1 + 1 < 0 {
            l2 = rectangle.x2;
        } else {
            r2 = rectangle.x2;
        }

        let mut t1 = self.y1;
        let mut b1 = self.y1;
        if self.y2 - self.y1 + 1 < 0 {
            t1 = self.y2;
        } else {
            b1 = self.y2;
        }

        let mut t2 = rectangle.y1;
        let mut b2 = rectangle.y1;
        if rectangle.y2 - rectangle.y1 + 1 < 0 {
            t2 = rectangle.y2;
        } else {
            b2 = rectangle.y2;
        }

        return Rect::from(l1.min(l2), r1.max(r2), t1.min(t2), b1.max(b2));
    }
}

impl ops::BitOrAssign<&Rect> for Rect {
    /// Unites this rectangle with the given `rectangle`.
    fn bitor_assign(&mut self, rectangle: &Rect) {
        let new_rect = rectangle | self;
        *self = new_rect;
    }
}

/// The RectF class defines a rectangle in the plane using floating point precision.
///
/// A rectangle is normally expressed as a top-left corner and a size.
/// The size (width and height) of a RectF is always equivalent to the mathematical rectangle
/// that forms the basis for its rendering.
///
/// A RectF can be constructed with a set of left, top, width and height values,
/// or from a PointF and a SizeF.
#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
pub struct RectF {
    x1: f64,
    y1: f64,
    x2: f64,
    y2: f64,
}

impl RectF {
    /// Constructs a null rectangle.
    pub fn new() -> Self {
        Self::from(0.0, 0.0, 0.0, 0.0)
    }

    /// Constructs a rectangle with `left`, `top` as its top-left corner and the given `width` and `height`.
    pub fn from(left: f64, top: f64, width: f64, height: f64) -> Self {
        Self {
            x1: left,
            y1: top,
            x2: left + width,
            y2: top + height,
        }
    }

    /// Constructs a rectangle with the given `top_left` corner and the given `size`.
    pub fn from_size(top_left: PointF, size: SizeF) -> Self {
        Self {
            x1: top_left.x(),
            y1: top_left.y(),
            x2: top_left.x() + size.width(),
            y2: top_left.y() + size.height(),
        }
    }

    /// Constructs a rectangle with the given `top_left` and `bottom_right` corners.
    pub fn from_points(top_left: PointF, bottom_right: PointF) -> Self {
        Self {
            x1: top_left.x(),
            y1: top_left.y(),
            x2: bottom_right.x(),
            y2: bottom_right.y(),
        }
    }

    /// Adds `dx1`, `dy1`, `dx2` and `dy2` respectively to the existing coordinates of the rectangle.
    pub fn adjust(&mut self, dx1: f64, dy1: f64, dx2: f64, dy2: f64) {
        self.x1 += dx1;
        self.y1 += dy1;
        self.x2 += dx2;
        self.y2 += dy2;
    }

    /// Returns a new rectangle with `dx1`, `dy1`, `dx2` and `dy2` added respectively
    /// to the existing coordinates of this rectangle.
    pub fn adjusted(&self, dx1: f64, dy1: f64, dx2: f64, dy2: f64) -> Self {
        Self::from(self.x1 + dx1, self.y1 + dy1, self.x2 + dx2, self.y2 + dy2)
    }

    /// Returns the y-coordinate of the rectangle's bottom edge.
    pub fn bottom(&self) -> f64 {
        self.y2
    }

    /// Returns the position of the rectangle's bottom-left corner.
    pub fn bottom_left(&self) -> PointF {
        PointF::from(self.x1, self.y2)
    }

    /// Returns the position of the rectangle's bottom-right corner.
    pub fn bottom_right(&self) -> PointF {
        PointF::from(self.x2, self.y2)
    }

    /// Returns the center point of the rectangle.
    pub fn center(&self) -> PointF {
        // cast avoids overflow on addition
        PointF::from(
            ((self.x1 as i64 + self.x2 as i64) / 2) as f64,
            ((self.y1 as i64 + self.y2 as i64) / 2) as f64,
        )
    }

    /// Returns true if the point (`x`, `y`) is inside this rectangle, otherwise returns false.
    pub fn contains(&self, x: f64, y: f64) -> bool {
        self.contains_helper(x, y, false)
    }

    /// Returns true if the point (`x`, `y`) is inside this rectangle (not on the edge),
    /// otherwise returns false.
    pub fn contains_proper(&self, x: f64, y: f64) -> bool {
        self.contains_helper(x, y, true)
    }

    fn contains_helper(&self, x: f64, y: f64, proper: bool) -> bool {
        let l;
        let r;
        // TODO(Shaohua): Replace with self.x2 < self.x1
        if self.x2 < self.x1 - 1.0 {
            l = self.x2;
            r = self.x1;
        } else {
            l = self.x1;
            r = self.x2;
        }
        if proper {
            if x <= l || x >= r {
                return false;
            }
        } else {
            if x < l || x > r {
                return false;
            }
        }

        let t;
        let b;
        if self.y2 < self.y1 - 1.0 {
            t = self.y2;
            b = self.y1;
        } else {
            t = self.y1;
            b = self.y2;
        }

        if proper {
            if y <= t || y >= b {
                return false;
            }
        } else {
            if y < t || y > b {
                return false;
            }
        }
        return true;
    }

    /// Returns true if the given point is inside or on the edge of the rectangle,
    /// otherwise returns false.
    pub fn contains_point(&self, point: &PointF) -> bool {
        self.contains(point.x(), point.y())
    }

    /// Returns true if the given point is inside of the rectangle,
    /// otherwise returns false (including on the edges).
    pub fn contains_point_proper(&self, point: &PointF) -> bool {
        self.contains_proper(point.x(), point.y())
    }

    /// Returns true if the given rectangle is inside this rectangle, including
    /// on the edge, otherwise returns false.
    pub fn contains_rect(&self, rect: &Self) -> bool {
        self.contains_rect_helper(rect, false)
    }

    /// Returns true if the given rectangle is inside this rectangle,
    /// otherwise returns false.
    pub fn contains_rect_proper(&self, rect: &Self) -> bool {
        self.contains_rect_helper(rect, true)
    }

    fn contains_rect_helper(&self, rect: &Self, proper: bool) -> bool {
        if self.is_null() || rect.is_null() {
            return false;
        }

        let mut l1 = self.x1;
        let mut r1 = self.x1;
        // TODO(Shaohua): Replace with self.x2 < self.x1
        if self.x2 - self.x1 + 1.0 < 0.0 {
            l1 = self.x2;
        } else {
            r1 = self.x2;
        }

        let mut l2 = rect.x1;
        let mut r2 = rect.x1;
        if rect.x2 - rect.x1 + 1.0 < 0.0 {
            l2 = rect.x2;
        } else {
            r2 = rect.x2;
        }

        if proper {
            if l2 <= l1 || r2 >= r1 {
                return false;
            }
        } else {
            if l2 < l1 || r2 > r1 {
                return false;
            }
        }

        let mut t1 = self.y1;
        let mut b1 = self.y1;
        if self.y2 - self.y1 + 1.0 < 0.0 {
            t1 = self.y2;
        } else {
            b1 = self.y2;
        }

        let mut t2 = rect.y1;
        let mut b2 = rect.y1;
        if rect.y2 - rect.y1 + 1.0 < 0.0 {
            t2 = rect.y2;
        } else {
            b2 = rect.y2;
        }

        if proper {
            if t2 <= t1 || b2 >= b1 {
                return false;
            }
        } else {
            if t2 < t1 || b2 > b1 {
                return false;
            }
        }

        return true;
    }

    /// Extracts the position of the rectangle's top-left corner to `x1` and `y1`,
    /// and the position of the bottom-right corner to `x2` and `y2`.
    pub fn get_coords(&self, x1: &mut f64, y1: &mut f64, x2: &mut f64, y2: &mut f64) {
        *x1 = self.x1;
        *y1 = self.y1;
        *x2 = self.x2;
        *y2 = self.y2;
    }

    /// Extracts the position of the rectangle's top-left corner to `x` and `y`, and
    /// its dimensions to `width` and `height`.
    pub fn get_rect(&self, x: &mut f64, y: &mut f64, width: &mut f64, height: &mut f64) {
        *x = self.x1;
        *y = self.y1;
        *width = self.x2 - self.x1;
        *height = self.y2 - self.y1;
    }

    /// Returns the height of the rectangle.
    pub fn height(&self) -> f64 {
        self.y2 - self.y1
    }

    /// Returns the intersection of this rectangle and the given rectangle.
    pub fn intersected(&self, rect: &Self) -> Self {
        unimplemented!()
    }

    /// Returns true if this rectangle intersects with the given rectangle
    /// (i.e., there is at least one pixel that is within both rectangles),
    /// otherwise returns false.
    pub fn intersects(&self, rect: &Self) -> bool {
        unimplemented!()
    }

    /// Returns true if the rectangle is empty, otherwise returns false.
    ///
    /// An empty rectangle has a left() >= right() or top() >= bottom().
    /// An empty rectangle is not valid (i.e., is_empty() == !is_valid()).
    ///
    /// Use the `normalized()` function to retrieve a rectangle where the corners are swapped.
    pub fn is_empty(&self) -> bool {
        self.x1 >= self.x2 || self.y1 >= self.y2
    }

    /// Returns true if the rectangle is a null rectangle, otherwise returns false.
    ///
    /// A null rectangle has both the width and the height set to 0
    /// (i.e., right() == left() and bottom() == top()).
    ///
    /// A null rectangle is also empty, and hence is not valid.
    pub fn is_null(&self) -> bool {
        self.x1 == self.x2 && self.y1 == self.y2
    }

    /// Returns true if the rectangle is valid, otherwise returns false.
    ///
    /// A valid rectangle has a left() < right() and top() < bottom().
    /// Note that non-trivial operations like intersections are not defined for invalid rectangles.
    ///
    /// A valid rectangle is not empty (i.e., is_valid() == !is_empty()).
    pub fn is_valid(&self) -> bool {
        self.x1 < self.x2 && self.y1 < self.y2
    }

    /// Returns the x-coordinate of the rectangle's left edge. Equivalent to `x()`.
    pub fn left(&self) -> f64 {
        self.x1
    }

    /// Returns a rectangle grown by the `margins`.
    pub fn margins_added(&self, margins: &MarginsF) -> Self {
        Self::from(
            self.x1 - margins.left(),
            self.y1 - margins.top(),
            self.x2 + margins.right(),
            self.y2 + margins.bottom(),
        )
    }

    /// Removes the `margins` from the rectangle, shrinking it.
    pub fn margins_removed(&self, margins: &MarginsF) -> Self {
        Self::from(
            self.x1 + margins.left(),
            self.y1 + margins.top(),
            self.x2 - margins.right(),
            self.y2 - margins.bottom(),
        )
    }

    /// Moves the rectangle vertically, leaving the rectangle's bottom edge
    /// at the given `y` coordinate.
    ///
    /// The rectangle's size is unchanged.
    pub fn move_bottom(&mut self, y: f64) {
        self.y1 += y - self.y2;
        self.y2 = y;
    }

    /// Moves the rectangle, leaving the bottom-left corner at the given `position`.
    ///
    /// The rectangle's size is unchanged.
    pub fn move_bottom_left(&mut self, position: &PointF) {
        self.move_left(position.x());
        self.move_bottom(position.y());
    }

    /// Moves the rectangle, leaving the bottom-right corner at the given `position`.
    ///
    /// The rectangle's size is unchanged.
    pub fn move_bottom_right(&mut self, position: &PointF) {
        self.move_right(position.x());
        self.move_bottom(position.y());
    }

    /// Moves the rectangle, leaving the center point at the given `position`.
    ///
    /// The rectangle's size is unchanged.
    pub fn move_center(&mut self, position: &PointF) {
        let width = self.x2 - self.x1;
        let height = self.y2 - self.y1;
        self.x1 = position.x() - width / 2.0;
        self.y1 = position.y() - width / 2.0;
        self.x2 = self.x1 + width;
        self.y2 = self.y1 + height;
    }

    /// Moves the rectangle horizontally, leaving the rectangle's left edge at the given `x` coordinate.
    ///
    /// The rectangle's size is unchanged.
    pub fn move_left(&mut self, x: f64) {
        self.x2 += x - self.x1;
        self.x1 = x;
    }

    /// Moves the rectangle horizontally, leaving the rectangle's right edge at the given `x` coordinate.
    ///
    /// The rectangle's size is unchanged.
    pub fn move_right(&mut self, x: f64) {
        self.x1 += x - self.x2;
        self.x2 = x;
    }

    /// Moves the rectangle, leaving the top-left corner at the given position (`x`, `y`).
    ///
    /// The rectangle's size is unchanged.
    pub fn move_to(&mut self, x: f64, y: f64) {
        self.x2 += x - self.x1;
        self.y2 += y - self.y1;
        self.x1 = x;
        self.y1 = y;
    }

    /// Moves the rectangle, leaving the top-left corner at the given `position`.
    pub fn move_to_point(&mut self, point: &PointF) {
        self.x2 += point.x() - self.x1;
        self.y2 += point.y() - self.y1;
        self.x1 = point.x();
        self.y1 = point.y();
    }

    /// Moves the rectangle vertically, leaving the rectangle's top edge at the given `y` coordinate.
    ///
    /// The rectangle's size is unchanged.
    pub fn move_top(&mut self, y: f64) {
        self.y2 += y - self.y1;
        self.y1 = y;
    }

    /// Moves the rectangle, leaving the top-left corner at the given `position`.
    ///
    /// The rectangle's size is unchanged.
    pub fn move_top_left(&mut self, position: &PointF) {
        self.move_left(position.x());
        self.move_top(position.y());
    }

    /// Moves the rectangle, leaving the top-right corner at the given `position`.
    ///
    /// The rectangle's size is unchanged.
    pub fn move_top_right(&mut self, position: &PointF) {
        self.move_right(position.x());
        self.move_top(position.y());
    }

    /// Returns a normalized rectangle; i.e., a rectangle that has a non-negative width and height.
    ///
    /// If width() < 0 the function swaps the left and right corners, and it swaps
    /// the top and bottom corners if height() < 0.
    pub fn normalized(&self) -> Self {
        unimplemented!()
    }

    /// Returns the x-coordinate of the rectangle's right edge.
    pub fn right(&self) -> f64 {
        self.x2
    }

    /// Sets the bottom edge of the rectangle to the given `y` coordinate.
    ///
    /// May change the height, but will never change the top edge of the rectangle.
    pub fn set_bottom(&mut self, y: f64) {
        self.y2 = y;
    }

    /// Set the bottom-left corner of the rectangle to the given `position`.
    ///
    /// May change the size, but will never change the top-right corner of the rectangle.
    pub fn set_bottom_left(&mut self, position: &PointF) {
        self.x1 = position.x();
        self.y2 = position.y();
    }

    /// Set the bottom-right corner of the rectangle to the given `position`.
    ///
    /// May change the size, but will never change the top-left corner of the rectangle.
    pub fn set_bottom_right(&mut self, position: &PointF) {
        self.x2 = position.x();
        self.y2 = position.y();
    }

    /// Sets the coordinates of the rectangle's top-left corner to (`x1`, `y1`),
    /// and the coordinates of its bottom-right corner to (`x2`, `y2`).
    pub fn set_coords(&mut self, x1: f64, y1: f64, x2: f64, y2: f64) {
        self.x1 = x1;
        self.y1 = y1;
        self.x2 = x2;
        self.y2 = y2;
    }

    /// Sets the height of the rectangle to the given `height`.
    ///
    /// The bottom edge is changed, but not the top one.
    pub fn set_height(&mut self, height: f64) {
        self.y2 = self.y1 + height;
    }

    /// Sets the left edge of the rectangle to the given `x` coordinate.
    ///
    /// May change the width, but will never change the right edge of the rectangle.
    pub fn set_left(&mut self, x: f64) {
        self.x1 = x;
    }

    /// Sets the coordinates of the rectangle's top-left corner to (`x`, `y`),
    /// and its size to the given `width` and `height`.
    pub fn set_rect(&mut self, x: f64, y: f64, width: f64, height: f64) {
        self.x1 = x;
        self.y1 = y;
        self.x2 = x + width;
        self.y2 = y + height;
    }

    /// Sets the right edge of the rectangle to the given `x` coordinate.
    ///
    /// May change the width, but will never change the left edge of the rectangle.
    pub fn set_right(&mut self, x: f64) {
        self.x2 = x;
    }

    /// Sets the size of the rectangle to the given `size`.
    ///
    /// The top-left corner is not moved.
    pub fn set_size(&mut self, size: &SizeF) {
        self.x2 = self.x1 + size.width();
        self.y2 = self.y1 + size.height();
    }

    /// Sets the top edge of the rectangle to the given `y` coordinate.
    ///
    /// May change the height, but will never change the bottom edge of the rectangle.
    pub fn set_top(&mut self, y: f64) {
        self.y1 = y;
    }

    /// Set the top-left corner of the rectangle to the given `position`.
    ///
    /// May change the size, but will never change the bottom-right corner of the rectangle.
    pub fn set_top_left(&mut self, position: &PointF) {
        self.x1 = position.x();
        self.y1 = position.y();
    }

    /// Set the top-right corner of the rectangle to the given `position`.
    ///
    /// May change the size, but will never change the bottom-left corner of the rectangle.
    pub fn set_top_right(&mut self, position: &PointF) {
        self.x2 = position.x();
        self.y1 = position.y();
    }

    /// Sets the width of the rectangle to the given `width`.
    ///
    /// The right edge is changed, but not the left one.
    pub fn set_width(&mut self, width: f64) {
        self.x2 = self.x1 + width;
    }

    /// Sets the left edge of the rectangle to the given `x` coordinate.
    ///
    /// May change the width, but will never change the right edge of the rectangle.
    ///
    /// Equivalent to `set_left()`.
    pub fn set_x(&mut self, x: f64) {
        self.x1 = x;
    }

    /// Sets the top edge of the rectangle to the given `y` coordinate.
    ///
    /// May change the height, but will never change the bottom edge of the rectangle.
    ///
    /// Equivalent to `set_top()`.
    pub fn set_y(&mut self, y: f64) {
        self.y1 = y;
    }

    /// Returns the size of the rectangle.
    pub fn size(&self) -> SizeF {
        SizeF::from(self.width(), self.height())
    }

    /// Returns the y-coordinate of the rectangle's top edge.
    ///
    /// Equivalent to `y()`.
    pub fn top(&self) -> f64 {
        self.y1
    }

    /// Returns the position of the rectangle's top-left corner.
    pub fn top_left(&self) -> PointF {
        PointF::from(self.x1, self.y1)
    }

    /// Returns the position of the rectangle's top-right corner.
    pub fn top_right(&self) -> PointF {
        PointF::from(self.x2, self.y1)
    }

    /// Moves the rectangle `dx` along the x axis and `dy` along the y axis,
    /// relative to the current position.
    ///
    /// Positive values move the rectangle to the right and down.
    pub fn translate(&mut self, dx: f64, dy: f64) {
        self.x1 += dx;
        self.y1 += dy;
        self.x2 += dx;
        self.y2 += dy;
    }

    /// Moves the rectangle `offset.x()` along the x axis and `offset.y()` along the y axis,
    /// relative to the current position.
    pub fn translate_point(&mut self, offset: &PointF) {
        self.x1 += offset.x();
        self.y1 += offset.y();
        self.x2 += offset.x();
        self.y2 += offset.y();
    }

    /// Returns a copy of the rectangle that is translated `dx` along the x axis
    /// and `dy` along the y axis, relative to the current position.
    ///
    /// Positive values move the rectangle to the right and down.
    pub fn translated(&self, dx: f64, dy: f64) -> Self {
        Self::from(self.x1 + dx, self.y1 + dy, self.x2 + dx, self.y2 + dy)
    }

    /// Returns a copy of the rectangle that is translated `offset.x()` along the x axis
    /// and `offset.y()` along the y axis, relative to the current position.
    pub fn translated_point(&self, offset: &PointF) -> Self {
        Self::from(
            self.x1 + offset.x(),
            self.y1 + offset.y(),
            self.x2 + offset.x(),
            self.y2 + offset.y(),
        )
    }

    /// Returns a copy of the rectangle that has its width and height exchanged:
    pub fn transposed(&self) -> Self {
        Self::from_size(self.top_left(), self.size().transposed())
    }

    /// Returns the bounding rectangle of this rectangle and the given `rectangle`.
    pub fn united(&self, rectangle: &Self) -> Self {
        unimplemented!()
    }

    /// Returns the width of the rectangle.
    pub fn width(&self) -> f64 {
        self.x2 - self.x1
    }

    /// Returns the x-coordinate of the rectangle's left edge.
    ///
    /// Equivalent to `left()`.
    pub fn x(&self) -> f64 {
        self.x1
    }

    /// Returns the y-coordinate of the rectangle's top edge.
    ///
    /// Equivalent to `top()`.
    pub fn y(&self) -> f64 {
        self.y1
    }
}
