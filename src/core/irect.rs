// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that can be found
// in the LICENSE file.

use crate::core::point::IPoint;
use crate::core::size::ISize;

/// `IRect` holds four 32-bit integer coordinates describing the upper and
/// lower bounds of a rectangle.
///
/// `IRect` may be created from outer bounds or from position, width, and height.
/// `IRect` describes an area; if its right is less than or equal to its left,
/// or if its bottom is less than or equal to its top, it is considered empty.
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct IRect {
    /// smaller x-axis bounds
    left: i32,

    /// smaller y-axis bounds
    top: i32,

    /// larger x-axis bounds
    right: i32,

    /// larger y-axis bounds
    bottom: i32,
}

impl Default for IRect {
    fn default() -> Self {
        Self::new()
    }
}

impl IRect {
    /// Returns constructed `IRect` set to (0, 0, 0, 0).
    ///
    /// Many other rectangles are empty; if left is equal to or greater than right,
    /// or if top is equal to or greater than bottom. Setting all members to zero
    /// is a convenience, but does not designate a special empty rectangle.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            left: 0,
            top: 0,
            right: 0,
            bottom: 0,
        }
    }

    /// Returns constructed `IRect` set to (0, 0, width, height).
    ///
    /// Does not validate input; width or height may be negative.
    #[must_use]
    pub const fn from_wh(width: i32, height: i32) -> Self {
        Self {
            left: 0,
            top: 0,
            right: width,
            bottom: height,
        }
    }

    /// Returns constructed `IRect` set to (0, 0, `size.width()`, `size.height()`).
    ///
    /// Does not validate input; `size.width()` or `size.height()` may be negative.
    #[must_use]
    pub const fn from_size(size: ISize) -> Self {
        Self {
            left: 0,
            top: 0,
            right: size.width(),
            bottom: size.height(),
        }
    }

    /// Returns constructed `IRect` set to
    /// (`pt.x()`, `pt.y()`, `pt.x()` + `size.width()`, `pt.y()` + `size.height()`).
    ///
    /// Does not validate input; `size.width()` or `size.height()` may be negative.
    ///
    /// # Parameters
    /// - `pt` - values for `IRect` left and top
    /// - `size` - values for `IRect` width and height
    #[must_use]
    pub const fn from_pt_size(&self, size: ISize) -> Self {
        Self::from_xywh(self.x(), self.y(), size.width(), size.height())
    }

    /// Returns constructed `IRect` set to (l, t, r, b).
    ///
    /// Does not sort input; `IRect` may result in left greater than right,
    /// or top greater than bottom.
    #[must_use]
    pub const fn from_ltrb(left: i32, top: i32, right: i32, bottom: i32) -> Self {
        Self {
            left,
            top,
            right,
            bottom,
        }
    }

    /// Returns constructed `IRect` set to: (x, y, x + w, y + h).
    /// Does not validate input; w or h may be negative.
    ///
    /// # Parameters
    /// - `x` - stored in left
    /// - `y` - stored in top
    /// - `width` - added to x and stored in right
    /// - `height` - added to y and stored in bottom
    #[must_use]
    pub const fn from_xywh(x: i32, y: i32, width: i32, height: i32) -> Self {
        Self {
            left: x,
            top: y,
            // TODO(Shaohua): Replace with 32_sat_add()
            right: x + width,
            bottom: y + height,
        }
    }

    /// Returns left edge of `IRect`, if sorted.
    ///
    /// Call `sort()` to reverse left and right if needed.
    #[must_use]
    pub const fn left(&self) -> i32 {
        self.left
    }

    /// Returns top edge of `IRect`, if sorted.
    /// Call `is_empty()` to see if `IRect` may be invalid,
    /// and `sort()` to reverse top and bottom if needed.
    #[must_use]
    pub const fn top(&self) -> i32 {
        self.top
    }

    /// Returns right edge of `IRect`, if sorted.
    ///
    /// Call `sort()` to reverse left and right if needed.
    #[must_use]
    pub const fn right(&self) -> i32 {
        self.right
    }

    /// Returns bottom edge of `IRect`, if sorted.
    ///
    /// Call `is_empty()` to see if `IRect` may be invalid,
    /// and `sort()` to reverse top and bottom if needed.
    #[must_use]
    pub const fn bottom(&self) -> i32 {
        self.bottom
    }

    /// Returns left edge of `IRect`, if sorted.
    ///
    /// Call `is_empty()` to see if `IRect` may be invalid,
    /// and `sort()` to reverse left and right if needed.
    #[must_use]
    pub const fn x(&self) -> i32 {
        self.left
    }

    /// Returns top edge of `IRect`, if sorted.
    ///
    /// Call `is_empty()` to see if `IRect` may be invalid,
    /// and `sort()` to reverse top and bottom if needed.
    #[must_use]
    pub const fn y(&self) -> i32 {
        self.top
    }

    #[must_use]
    pub const fn top_left(&self) -> IPoint {
        IPoint::from_xy(self.left, self.top)
    }

    /// Returns span on the x-axis.
    ///
    /// This does not check if `IRect` is sorted, or if result fits in 32-bit signed integer;
    /// result may be negative.
    #[must_use]
    pub const fn width(&self) -> i32 {
        // FIXME(Shaohua): check overflow
        self.right - self.left
    }

    /// Returns span on the y-axis.
    ///
    /// This does not check if `IRect` is sorted, or if result fits in 32-bit signed integer;
    /// result may be negative.
    #[must_use]
    pub const fn height(&self) -> i32 {
        // FIXME(Shaohua): check overflow
        self.bottom - self.top
    }

    /// Returns spans on the x-axis and y-axis.
    ///
    /// This does not check if `IRect` is sorted, or if result fits in 32-bit signed integer;
    /// result may be negative.
    #[must_use]
    pub const fn size(&self) -> ISize {
        ISize::from_wh(self.width(), self.height())
    }

    /// Returns span on the x-axis.
    ///
    /// This does not check if `IRect` is sorted, so the result may be negative.
    /// This is safer than calling `width()` since `width()` might overflow in its calculation.
    #[must_use]
    pub const fn width64(&self) -> i64 {
        self.right as i64 - self.left as i64
    }

    /// Returns span on the y-axis.
    ///
    /// This does not check if `IRect` is sorted, so the result may be negative.
    /// This is safer than calling `height()` since `height()` might overflow in its calculation.
    #[must_use]
    pub const fn height64(&self) -> i64 {
        self.bottom as i64 - self.top as i64
    }

    /// Returns true if left is equal to or greater than right,
    /// or if top is equal to or greater than bottom.
    ///
    /// Call `sort()` to reverse rectangles with negative `width64()` or `height64()`.
    ///
    /// Returns true if `width64()` or `height64()` are zero or negative
    #[must_use]
    pub const fn is_empty64(&self) -> bool {
        self.right <= self.left || self.bottom <= self.top
    }

    /// Returns true if `width()` or `height()` are zero or negative.
    #[must_use]
    pub const fn is_empty(&self) -> bool {
        let w = self.width64();
        let h = self.height64();
        if w <= 0 || h <= 0 {
            return true;
        }
        // Return true if either exceeds i32
        //return !TFitsIn<int32_t>(w | h);
        // FIXME(Shaohua): Check fits in i32
        false
    }

    /// Sets `IRect` to (0, 0, 0, 0).
    ///
    /// Many other rectangles are empty; if left is equal to or greater than right,
    /// or if top is equal to or greater than bottom.
    /// Setting all members to zero is a convenience, but does not designate
    /// a special empty rectangle.
    pub fn set_empty(&mut self) {
        self.left = 0;
        self.top = 0;
        self.right = 0;
        self.bottom = 0;
    }

    /// Sets `IRect` to (left, top, right, bottom).
    ///
    /// left and right are not sorted; left is not necessarily less than right.
    /// top and bottom are not sorted; top is not necessarily less than bottom.
    pub fn set_ltrb(&mut self, left: i32, top: i32, right: i32, bottom: i32) {
        self.left = left;
        self.top = top;
        self.right = right;
        self.bottom = bottom;
    }

    /// Sets `IRect` to: (x, y, x + width, y + height).
    ///
    /// Does not validate input; width or height may be negative.
    ///
    /// # Parameters
    /// - `x` - stored in left
    /// - `y` - stored in top
    /// - `width` - added to x and stored in right
    /// - `height` - added to y and stored in bottom
    pub fn set_xywh(&mut self, x: i32, y: i32, width: i32, height: i32) {
        self.left = x;
        self.top = y;
        // FIXME(Shaohua); Call sat_add to check overflow
        self.right = x + width;
        self.bottom = y + height;
    }

    pub fn set_wh(&mut self, width: i32, height: i32) {
        self.left = 0;
        self.top = 0;
        self.right = width;
        self.bottom = height;
    }

    pub fn set_size(&mut self, size: ISize) {
        self.left = 0;
        self.top = 0;
        self.right = size.width();
        self.bottom = size.height();
    }

    /// Returns `IRect` offset by (dx, dy).
    ///
    /// If dx is negative, `IRect` returned is moved to the left.
    /// If dx is positive, `IRect` returned is moved to the right.
    /// If dy is negative, `IRect` returned is moved upward.
    /// If dy is positive, `IRect` returned is moved downward.
    ///
    /// # Parameters
    /// - `dx` - offset added to left and right
    /// - `dy` - offset added to top and bottom
    ///
    /// Returns `IRect` offset by dx and dy, with original width and height
    #[must_use]
    pub const fn from_offset(&self, dx: i32, dy: i32) -> Self {
        // FIXME(Shaohua): Check add overflow
        Self {
            left: self.left + dx,
            top: self.top + dy,
            right: self.right + dx,
            bottom: self.bottom + dy,
        }
    }

    /// Returns `IRect` offset by (`offset.x()`, `offset.y()`).
    ///
    /// If `offset.x()` is negative, `IRect` returned is moved to the left.
    /// If `offset.x()` is positive, `IRect` returned is moved to the right.
    /// If `offset.y()` is negative, `IRect` returned is moved upward.
    /// If `offset.y()` is positive, `IRect` returned is moved downward.
    ///
    /// # Parameters
    /// - `offset` - translation vector
    ///
    /// Returns `IRect` translated by offset, with original width and height
    #[must_use]
    pub const fn from_offset_point(&self, offset: &IPoint) -> Self {
        self.from_offset(offset.x(), offset.y())
    }

    /// Returns `IRect`, inset by (dx, dy).
    ///
    /// If dx is negative, `IRect` returned is wider.
    /// If dx is positive, `IRect` returned is narrower.
    /// If dy is negative, `IRect` returned is taller.
    /// If dy is positive, `IRect` returned is shorter.
    ///
    /// # Parameters
    /// - `dx` - offset added to left and subtracted from right
    /// - `dy` - offset added to top and subtracted from bottom
    ///
    /// Returns `IRect` inset symmetrically left and right, top and bottom
    #[must_use]
    pub const fn from_inset(&self, dx: i32, dy: i32) -> Self {
        // FIXME(Shaohua): Check add/sub overflow
        Self {
            left: self.left + dx,
            top: self.top + dy,
            right: self.right - dx,
            bottom: self.bottom - dy,
        }
    }

    /// Returns `IRect`, outset by (dx, dy).
    ///
    /// If dx is negative, `IRect` returned is narrower.
    /// If dx is positive, `IRect` returned is wider.
    /// If dy is negative, `IRect` returned is shorter.
    /// If dy is positive, `IRect` returned is taller.
    ///
    /// # Parameters
    /// - `dx` - offset subtracted to left and added from right
    /// - `dy` - offset subtracted to top and added from bottom
    ///
    /// Returns `IRect` outset symmetrically left and right, top and bottom
    #[must_use]
    pub const fn from_outset(&self, dx: i32, dy: i32) -> Self {
        // FIXME(Shaohua): Check add/sub overflow
        Self {
            left: self.left - dx,
            top: self.top - dy,
            right: self.right + dx,
            bottom: self.bottom + dy,
        }
    }

    /// Offsets `IRect` by adding dx to left, right; and by adding dy to top, bottom.
    ///
    /// If dx is negative, moves `IRect` returned to the left.
    /// If dx is positive, moves `IRect` returned to the right.
    /// If dy is negative, moves `IRect` returned upward.
    /// If dy is positive, moves `IRect` returned downward.
    ///
    /// # Parameters
    /// - `dx` - offset added to left and right
    /// - `dy` - offset added to top and bottom
    pub fn offset(&mut self, dx: i32, dy: i32) {
        // FIXME(Shaohua): Check add overflow
        self.left += dx;
        self.top += dy;
        self.right += dx;
        self.bottom += dy;
    }

    /// Offsets `IRect` by adding delta.x to left, right;
    /// and by adding delta.y to top, bottom.
    ///
    /// If delta.x is negative, moves `IRect` returned to the left.
    /// If delta.x is positive, moves `IRect` returned to the right.
    /// If delta.y is negative, moves `IRect` returned upward.
    /// If delta.y is positive, moves `IRect` returned downward.
    pub fn offset_point(&mut self, delta: IPoint) {
        self.offset(delta.x(), delta.y());
    }

    /// Offsets `IRect` so that left equals `new_x`, and top equals `new_y`.
    ///
    /// width and height are unchanged.
    ///
    /// # Parameters
    /// - `new_x` - stored in left, preserving `width()`
    /// - `new_y` - stored in top, preserving `height()`
    pub fn offset_to(&mut self, new_x: i32, new_y: i32) {
        // TODO(Shaohua): Call pin_tos32()
        self.right = self.right + new_x - self.left;
        self.bottom = self.bottom + new_y - self.top;
        self.left = new_x;
        self.top = new_y;
    }

    /// Insets `IRect` by (dx,dy).
    ///
    /// If dx is positive, makes `IRect` narrower.
    /// If dx is negative, makes `IRect` wider.
    /// If dy is positive, makes `IRect` shorter.
    /// If dy is negative, makes `IRect` taller.
    ///
    /// # Parameters
    /// - `dx` - offset added to left and subtracted from right
    /// - `dy` - offset added to top and subtracted from bottom
    pub fn inset(&mut self, dx: i32, dy: i32) {
        // TODO(Shaohua): Check add overflow
        self.left += dx;
        self.top += dy;
        self.right += dx;
        self.bottom += dy;
    }

    /// Outsets `IRect` by (dx, dy).
    ///
    /// If dx is positive, makes `IRect` wider.
    /// If dx is negative, makes `IRect` narrower.
    /// If dy is positive, makes `IRect` taller.
    /// If dy is negative, makes `IRect` shorter.
    ///
    /// # Parameters
    /// - `dx`  subtracted to left and added from right
    /// - `dy`  subtracted to top and added from bottom
    pub fn outset(&mut self, dx: i32, dy: i32) {
        self.inset(-dx, -dy);
    }

    /// Adjusts `IRect` by adding `delta_left` to left, `delta_top` to top,
    /// `delta_right` to right, and `delta_bottom` to bottom.
    ///
    /// - If `delta_left` is positive, narrows `IRect` on the left. If negative, widens it on the left.
    /// - If `delta_top` is positive, shrinks `IRect` on the top. If negative, lengthens it on the top.
    /// - If `delta_right` is positive, narrows `IRect` on the right. If negative, widens it on the right.
    /// - If `delta_bottom` is positive, shrinks `IRect` on the bottom. If negative, lengthens it on the bottom.
    ///
    /// The resulting `IRect` is not checked for validity. Thus, if the resulting `IRect` left is
    /// greater than right, the `IRect` will be considered empty. Call `sort()` after this call
    /// if that is not the desired behavior.
    ///
    /// # Parameters
    /// - `delta_left` - offset added to left
    /// - `delta_top` - offset added to top
    /// - `delta_right` - offset added to right
    /// - `delta_bottom` - offset added to bottom
    pub fn adjust(&mut self, delta_left: i32, delta_top: i32, delta_right: i32, delta_bottom: i32) {
        // TODO(Shaohua): Check add overflow
        self.left += delta_left;
        self.top += delta_top;
        self.right += delta_right;
        self.bottom += delta_bottom;
    }

    /// Returns true if: `left <= x < right && top <= y < bottom`.
    /// Returns false if `IRect` is empty.
    ///
    /// Considers input to describe constructed `IRect`: `(x, y, x + 1, y + 1)` and
    /// returns true if constructed area is completely enclosed by `IRect` area.
    ///
    /// # Parameters
    /// - `x` - test `IPoint` x-coordinate
    /// - `y` - test `IPoint` y-coordinate
    ///
    /// Returns true if (x, y) is inside `IRect`
    #[must_use]
    pub const fn contains(&self, x: i32, y: i32) -> bool {
        self.left <= x && x < self.right && self.top <= y && y < self.bottom
    }

    /// Returns true if `IRect` contains other rect.
    /// Returns false if `IRect` is empty or other rect is empty.
    ///
    /// `IRect` contains rect when `IRect` area completely includes other rect area.
    #[must_use]
    pub const fn contains_rect(&self, other: &Self) -> bool {
        !other.is_empty()
            && self.is_empty()
            && self.left <= other.left
            && self.top <= other.top
            && self.right >= other.right
            && self.bottom >= other.bottom
    }

    /// Returns true if `IRect` contains construction.
    ///
    /// Asserts if `IRect` is empty or construction is empty.
    ///
    /// Return is undefined if `IRect` is empty or construction is empty.
    #[must_use]
    pub const fn contains_no_empty_check(&self, other: &Self) -> bool {
        debug_assert!(self.left < self.right && self.top < self.bottom);
        debug_assert!(other.left < other.right && other.top < other.bottom);
        self.left <= other.left
            && self.top <= other.top
            && self.right >= other.right
            && self.bottom >= other.bottom
    }

    /// Returns true if `IRect` intersects r, and sets `IRect` to intersection.
    ///
    /// Returns false if `IRect` does not intersect r, and leaves `IRect` unchanged.
    /// Returns false if either r or `IRect` is empty, leaving `IRect` unchanged.
    #[must_use]
    pub const fn intersect(&self, _r: &Self) -> bool {
        unimplemented!()
    }

    /// Sets `IRect` to the union of itself and r.
    ///
    /// Has no effect if r is empty. Otherwise, if `IRect` is empty, sets `IRect` to r.
    pub fn join(&mut self, _r: &Self) {
        unimplemented!()
    }

    /// Swaps left and right if left is greater than right;
    /// and swaps top and bottom if top is greater than bottom.
    ///
    /// Result may be empty, and `width()` and `height()` will be zero or positive.
    pub fn sort(&mut self) {
        (self.left, self.right) = (self.left.min(self.right), self.left.max(self.right));
        (self.top, self.bottom) = (self.top.min(self.bottom), self.top.max(self.bottom));
    }

    /// Returns `IRect` with left and right swapped if left is greater than right;
    /// and with top and bottom swapped if top is greater than bottom.
    ///
    /// Result may be empty; and `width()` and `height()` will be zero or positive.
    #[must_use]
    pub fn from_sorted(&self) -> Self {
        Self::from_ltrb(
            self.left.min(self.right),
            self.top.min(self.bottom),
            self.left.max(self.right),
            self.top.max(self.bottom),
        )
    }
}
