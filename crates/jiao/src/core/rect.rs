// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that can be found
// in the LICENSE file.

#![allow(clippy::module_name_repetitions)]

use crate::core::irect::IRect;
use crate::core::point::Point;
use crate::core::point::Vector;
use crate::core::scalar::{Scalar, ScalarExt};
use crate::core::size::{ISize, Size};

/// Rect holds four float coordinates describing the upper and lower bounds of a rectangle.
///
/// Rect may be created from outer bounds or from position, width, and height.
/// Rect describes an area; if its right is less than or equal to its left,
/// or if its bottom is less than or equal to its top, it is considered empty.
#[derive(Debug, Default, Clone, PartialEq)]
pub struct Rect {
    /// smaller x-axis bounds
    left: f32,
    /// smaller y-axis bounds
    top: f32,
    /// larger x-axis bounds
    right: f32,
    /// larger y-axis bounds
    bottom: f32,
}

impl Rect {
    /// Returns constructed Rect set to (0, 0, 0, 0).
    ///
    /// Many other rectangles are empty; if left is equal to or greater than right,
    /// or if top is equal to or greater than bottom. Setting all members to zero
    /// is a convenience, but does not designate a special empty rectangle.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            left: 0.0,
            top: 0.0,
            right: 0.0,
            bottom: 0.0,
        }
    }

    /// Returns constructed Rect set to float values (0, 0, width, height).
    ///
    /// Does not validate input; width or hight may be negative.
    ///
    /// Passing integer values may generate a compiler warning since Rect cannot
    /// represent 32-bit integers exactly. Use `IRect` for an exact integer rectangle.
    #[must_use]
    pub const fn from_wh(width: f32, height: f32) -> Self {
        Self {
            left: 0.0,
            top: 0.0,
            right: width,
            bottom: height,
        }
    }

    /// Returns constructed Rect set to integer values (0, 0, width, height).
    ///
    /// Does not validate input; width or height may be negative.
    #[must_use]
    #[allow(clippy::cast_precision_loss)]
    pub const fn from_iwh(width: i32, height: i32) -> Self {
        Self {
            left: 0.0,
            top: 0.0,
            right: width as f32,
            bottom: height as f32,
        }
    }

    /// Returns constructed Rect set to (0, 0, `size.width()`, `size.height()`).
    ///
    /// Does not validate input; `size.width()` or `size.height()` may be negative.
    #[must_use]
    pub const fn from_size(size: &Size) -> Self {
        Self {
            left: 0.0,
            top: 0.0,
            right: size.width(),
            bottom: size.height(),
        }
    }

    /// Returns constructed Rect set to (left, top, right, bottom).
    ///
    /// Does not sort input; Rect may result in left greater than right,
    /// or top greater than bottom.
    #[must_use]
    pub const fn from_ltrb(left: f32, top: f32, right: f32, bottom: f32) -> Self {
        Self {
            left,
            top,
            right,
            bottom,
        }
    }

    /// Returns constructed Rect set to (x, y, x + width, y + height).
    ///
    /// Does not validate input; width or height may be negative.
    ///
    /// # Parameters
    /// - `x` - stored in left
    /// - `y` - stored in top
    /// - `width` - added to x and stored in right
    /// - `height` - added to y and stored in bottom
    #[must_use]
    pub fn from_xywh(x: f32, y: f32, width: f32, height: f32) -> Self {
        Self {
            left: x,
            top: y,
            right: x + width,
            bottom: y + height,
        }
    }

    /// Returns constructed `IRect` set to (0, 0, `size.width()`, `size.height()`).
    ///
    /// Does not validate input; `size.width()` or `size.height()` may be negative.
    #[must_use]
    pub const fn from_isize(size: ISize) -> Self {
        Self::from_iwh(size.width(), size.height())
    }

    /// Returns constructed `IRect` set to irect, promoting integers to float.
    ///
    /// Does not validate input; left may be greater than right,
    /// top may be greater than bottom.
    #[must_use]
    #[allow(clippy::cast_precision_loss)]
    pub const fn from_irect(irect: &IRect) -> Self {
        Self {
            left: irect.left() as f32,
            top: irect.top() as f32,
            right: irect.right() as f32,
            bottom: irect.bottom() as f32,
        }
    }

    /// Returns Rect to bounds of Point array.
    #[must_use]
    pub fn from_points(points: &[Point]) -> Self {
        let mut r = Self::new();
        r.set_bounds(points);
        r
    }

    /// Returns true if left is equal to or greater than right,
    /// or if top is equal to or greater than bottom.
    ///
    /// Call `sort()` to reverse rectangles with negative `width()` or `height()`.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        // We write it as the NOT of a non-empty rect, so we will return true if any values
        // are NaN.
        self.left < self.right && self.top < self.bottom
    }

    /// Returns true if left is equal to or less than right,
    /// or if top is equal to or less than bottom.
    ///
    /// Call `sort()` to reverse rectangles with negative `width()` or `height()`.
    #[must_use]
    pub fn is_sorted(&self) -> bool {
        self.left <= self.right && self.top <= self.bottom
    }

    /// Returns true if all values in the rectangle are finite.
    #[must_use]
    pub fn is_finite(&self) -> bool {
        let mut accum = 0.0;
        accum *= self.left;
        accum *= self.top;
        accum *= self.right;
        accum *= self.bottom;

        // accum is either NaN or it is finite (zero).
        debug_assert!(0.0 == accum || accum.is_nan());

        // value==value will be true iff value is not NaN
        !accum.is_nan()
    }

    /// Returns left edge of Rect, if sorted.
    ///
    /// Call `is_sorted()` to see if Rect is valid.
    /// Call `sort()` to reverse left and right if needed.
    #[must_use]
    pub const fn x(&self) -> f32 {
        self.left
    }

    /// Returns top edge of Rect, if sorted.
    ///
    /// Call `is_empty()` to see if Rect may be invalid,
    /// and `sort()` to reverse top and bottom if needed.
    #[must_use]
    pub const fn y(&self) -> f32 {
        self.top
    }

    /// Returns left edge of Rect, if sorted.
    ///
    /// Call `is_sorted()` to see if Rect is valid.
    /// Call `sort()` to reverse left and right if needed.
    #[must_use]
    pub const fn left(&self) -> f32 {
        self.left
    }

    /// Returns top edge of Rect, if sorted.
    ///
    /// Call `is_empty()` to see if Rect may be invalid,
    /// and `sort()` to reverse top and bottom if needed.
    #[must_use]
    pub const fn top(&self) -> f32 {
        self.top
    }

    /// Returns right edge of Rect, if sorted.
    ///
    /// Call `is_sorted()` to see if Rect is valid.
    /// Call `sort()` to reverse left and right if needed.
    #[must_use]
    pub const fn right(&self) -> f32 {
        self.right
    }

    /// Returns bottom edge of Rect, if sorted.
    ///
    /// Call `is_empty()` to see if Rect may be invalid,
    /// and `sort()` to reverse top and bottom if needed.
    #[must_use]
    pub const fn bottom(&self) -> f32 {
        self.bottom
    }

    /// Returns span on the x-axis.
    ///
    /// This does not check if Rect is sorted, or if result fits in 32-bit float;
    /// result may be negative or infinity.
    #[must_use]
    pub fn width(&self) -> f32 {
        self.right - self.left
    }

    /// Returns span on the y-axis.
    ///
    /// This does not check if Rect is sorted, or if result fits in 32-bit float;
    /// result may be negative or infinity.
    #[must_use]
    pub fn height(&self) -> f32 {
        self.bottom - self.top
    }

    /// Returns average of left edge and right edge.
    ///
    /// Result does not change if Rect is sorted.
    /// Result may overflow to infinity if Rect is far from the origin.
    #[must_use]
    pub fn center_x(&self) -> f32 {
        self.left.mid_point(self.right)
    }

    /// Returns average of top edge and bottom edge.
    ///
    /// Result does not change if Rect is sorted.
    #[must_use]
    pub fn center_y(&self) -> f32 {
        self.top.mid_point(self.bottom)
    }

    /// Returns the point `(center_x(), center_y())`.
    #[must_use]
    pub fn center(&self) -> Point {
        Point::from_xy(self.center_x(), self.center_y())
    }

    /// Returns four points in quad that enclose Rect ordered as: top-left, top-right,
    /// bottom-right, bottom-left.
    pub fn to_quad(&self, _quad: &mut [Point; 4]) {
        unimplemented!()
    }

    /// Sets Rect to (0, 0, 0, 0).
    ///
    /// Many other rectangles are empty; if left is equal to or greater than right,
    /// or if top is equal to or greater than bottom. Setting all members to zero
    /// is a convenience, but does not designate a special empty rectangle.
    pub fn set_empty(&mut self) {
        *self = Self::new();
    }

    /// Sets Rect to src, promoting src members from integer to float.
    ///
    /// Very large values in src may lose precision.
    #[allow(clippy::cast_precision_loss)]
    pub fn set_irect(&mut self, src: &IRect) {
        self.left = src.left() as f32;
        self.top = src.top() as f32;
        self.right = src.right() as f32;
        self.bottom = src.bottom() as f32;
    }

    /// Sets Rect to (left, top, right, bottom).
    ///
    /// left and right are not sorted; left is not necessarily less than right.
    /// top and bottom are not sorted; top is not necessarily less than bottom.
    pub fn set_ltrb(&mut self, left: f32, top: f32, right: f32, bottom: f32) {
        self.left = left;
        self.top = top;
        self.right = right;
        self.bottom = bottom;
    }

    /// Sets to bounds of Point array.
    ///
    /// If count is zero or smaller, or if Point array contains an infinity or NaN,
    /// sets to (0, 0, 0, 0).
    ///
    /// Result is either empty or sorted: left is less than or equal to right, and
    /// top is less than or equal to bottom.
    pub fn set_bounds(&mut self, points: &[Point]) {
        self.set_bounds_check(points);
    }

    /// Sets to bounds of Point array with count entries.
    ///
    /// Returns false if count is zero or smaller, or if Point array contains
    /// an infinity or NaN; in these cases sets Rect to (0, 0, 0, 0).
    ///
    /// Result is either empty or sorted: left is less than or equal to right, and
    /// top is less than or equal to bottom.
    pub fn set_bounds_check(&mut self, _points: &[Point]) {
        unimplemented!()
    }

    /// Sets to bounds of Point pts array with count entries.
    ///
    /// If any Point in pts contains infinity or NaN, all Rect dimensions are set to NaN.
    pub fn set_bounds_no_check(&mut self, _points: &[Point]) {
        unimplemented!()
    }

    /// Sets bounds to the smallest Rect enclosing Point p0 and p1.
    ///
    /// The result is sorted and may be empty.
    /// Does not check to see if values are finite.
    ///
    /// # Parameters
    /// - `p0` - corner to include
    /// - `p1`  corner to include
    pub fn set_points(&mut self, p0: &Point, p1: &Point) {
        self.left = p0.x().min(p1.x());
        self.right = p0.x().max(p1.x());
        self.top = p0.y().min(p1.y());
        self.bottom = p0.y().max(p1.y());
    }

    /// Sets Rect to (x, y, x + width, y + height).
    ///
    /// Does not validate input; width or height may be negative.
    ///
    /// # Parameters
    /// - `x` - stored in left
    /// - `y` - stored in top
    /// - `width` - added to x and stored in right
    /// - `height` - added to y and stored in bottom
    pub fn set_xywh(&mut self, x: f32, y: f32, width: f32, height: f32) {
        self.left = x;
        self.top = y;
        self.right = x + width;
        self.bottom = y + height;
    }

    /// Sets Rect to (0, 0, width, height).
    ///
    /// Does not validate input; width or height may be negative.
    ///
    /// # Parameters
    /// - `width` - stored in right
    /// - `height` - stored in bottom
    pub fn set_wh(&mut self, width: f32, height: f32) {
        self.left = 0.0;
        self.top = 0.0;
        self.right = width;
        self.bottom = height;
    }

    #[allow(clippy::cast_precision_loss)]
    pub fn set_iwh(&mut self, width: i32, height: i32) {
        self.left = 0.0;
        self.top = 0.0;
        self.right = width as f32;
        self.bottom = height as f32;
    }

    /// Returns Rect offset by (dx, dy).
    ///
    /// If dx is negative, Rect returned is moved to the left.
    /// If dx is positive, Rect returned is moved to the right.
    /// If dy is negative, Rect returned is moved upward.
    /// If dy is positive, Rect returned is moved downward.
    ///
    /// # Parameters
    /// - `dx` - added to left and right
    /// - `dy` - added to top and bottom
    ///
    /// Returns Rect offset on axes, with original width and height.
    #[must_use]
    pub fn from_offset(&self, dx: f32, dy: f32) -> Self {
        Self::from_ltrb(
            self.left + dx,
            self.top + dy,
            self.right + dx,
            self.bottom + dy,
        )
    }

    /// Returns Rect offset by v.
    #[must_use]
    pub fn from_offset_vector(&self, v: &Vector) -> Self {
        self.from_offset(v.x(), v.y())
    }

    /// Returns Rect, inset by (dx, dy).
    ///
    /// If dx is negative, Rect returned is wider.
    /// If dx is positive, Rect returned is narrower.
    /// If dy is negative, Rect returned is taller.
    /// If dy is positive, Rect returned is shorter.
    ///
    /// # Parameters
    /// - `dx` - added to left and subtracted from right
    /// - `dy` - added to top and subtracted from bottom
    ///
    /// Returns Rect inset symmetrically left and right, top and bottom
    #[must_use]
    pub fn from_inset(&self, dx: f32, dy: f32) -> Self {
        Self::from_ltrb(
            self.left + dx,
            self.top + dy,
            self.right - dx,
            self.bottom - dy,
        )
    }

    /// Returns Rect, outset by (dx, dy).
    ///
    /// If dx is negative, Rect returned is narrower.
    /// If dx is positive, Rect returned is wider.
    /// If dy is negative, Rect returned is shorter.
    /// If dy is positive, Rect returned is taller.
    ///
    /// # Parameters
    /// - `dx` - subtracted to left and added from right
    /// - `dy` - subtracted to top and added from bottom
    ///
    /// Returns Rect outset symmetrically left and right, top and bottom
    #[must_use]
    pub fn from_outset(&self, dx: f32, dy: f32) -> Self {
        Self::from_ltrb(
            self.left - dx,
            self.top - dy,
            self.right + dx,
            self.bottom + dy,
        )
    }

    /// Offsets Rect by adding dx to left, right; and by adding dy to top, bottom.
    ///
    /// If dx is negative, moves Rect to the left.
    /// If dx is positive, moves Rect to the right.
    /// If dy is negative, moves Rect upward.
    /// If dy is positive, moves Rect downward.
    ///
    /// # Parameters
    /// - `dx` - offset added to left and right
    /// - `dy` - offset added to top and bottom
    pub fn offset(&mut self, dx: f32, dy: f32) {
        self.left += dx;
        self.top += dy;
        self.right += dx;
        self.bottom += dy;
    }

    /// Offsets Rect by adding delta.x to left, right; and by adding delta.y to top, bottom.
    ///
    /// If delta.x is negative, moves Rect to the left.
    /// If delta.x is positive, moves Rect to the right.
    /// If delta.y is negative, moves Rect upward.
    /// If delta.y is positive, moves Rect downward.
    pub fn offset_point(&mut self, delta: &Point) {
        self.offset(delta.x(), delta.y());
    }

    /// Offsets Rect so that left equals `new_x`, and top equals `new_y`.
    ///
    /// width and height are unchanged.
    ///
    /// # Parameters
    /// - `new_x` - stored in left, preserving `width()`
    /// - `new_y` - stored in top, preserving `height()`
    pub fn offset_to(&mut self, new_x: f32, new_y: f32) {
        self.right += new_x - self.left;
        self.bottom += new_y - self.top;
        self.left = new_x;
        self.top = new_y;
    }

    /// Insets Rect by (dx, dy).
    ///
    /// If dx is positive, makes Rect narrower.
    /// If dx is negative, makes Rect wider.
    /// If dy is positive, makes Rect shorter.
    /// If dy is negative, makes Rect taller.
    ///
    /// # Parameters
    /// - `dx` - added to left and subtracted from right
    /// - `dy` - added to top and subtracted from bottom
    pub fn inset(&mut self, dx: f32, dy: f32) {
        self.left += dx;
        self.top += dy;
        self.right -= dx;
        self.bottom -= dy;
    }

    /// Outsets Rect by (dx, dy).
    ///
    /// If dx is positive, makes Rect wider.
    /// If dx is negative, makes Rect narrower.
    /// If dy is positive, makes Rect taller.
    /// If dy is negative, makes Rect shorter.
    ///
    /// # Parameters
    /// - `dx` - subtracted to left and added from right
    /// - `dy` - subtracted to top and added from bottom
    pub fn outset(&mut self, dx: f32, dy: f32) {
        self.inset(-dx, -dy);
    }

    /// Returns true if Rect intersects other rect, and sets Rect to intersection.
    ///
    /// Returns false if Rect does not intersect other, and leaves Rect unchanged.
    ///
    /// Returns false if either other or Rect is empty, leaving Rect unchanged.
    pub fn intersect(&mut self, _other: &Self) -> bool {
        unimplemented!()
    }

    /// Returns true if Rect intersects other rect.
    ///
    /// Returns false if either other or self is empty, or do not intersect.
    #[must_use]
    pub fn intersects(&self, other: &Self) -> bool {
        let left = self.left.max(other.left);
        let right = self.right.min(other.right);
        let top = self.top.max(other.top);
        let bottom = self.bottom.min(other.bottom);
        left < right && top < bottom
    }

    /// Sets Rect to the union of itself and other rect.
    ///
    /// Has no effect if rect is empty.
    /// Otherwise, if Rect is empty, sets Rect to rect.
    pub fn join(&mut self, _other: &Self) {
        unimplemented!()
    }

    /// Sets Rect to the union of itself and other rect.
    ///
    /// Asserts if other is empty.  If Rect is empty, sets Rect to other.
    ///
    /// May produce incorrect results if other is empty.
    pub fn join_non_empty_arg(&mut self, other: &Self) {
        debug_assert!(!other.is_empty());

        // if we are empty, just assign
        if self.left >= self.right || self.top >= self.bottom {
            *self = other.clone();
        } else {
            self.join_possibly_empty_rect(other);
        }
    }

    /// Sets Rect to the union of itself and the construction.
    ///
    /// May produce incorrect results if Rect or other is empty.
    pub fn join_possibly_empty_rect(&mut self, other: &Self) {
        self.left = self.left.min(other.left);
        self.top = self.top.min(other.top);
        self.right = self.right.max(other.right);
        self.bottom = self.bottom.max(other.bottom);
    }

    /// Returns true if: left <= x < right && top <= y < bottom.
    ///
    /// Returns false if Rect is empty.
    ///
    /// # Parameters
    /// - `x` - test Point x-coordinate
    /// - `y` - test Point y-coordinate
    #[must_use]
    pub fn contains(&self, x: f32, y: f32) -> bool {
        self.left <= x && x < self.right && self.top <= y && y < self.bottom
    }

    /// Returns true if Rect contains other rect.
    ///
    /// Returns false if Rect is empty or rect is empty.
    /// Rect contains rect when Rect area completely includes rect area.
    #[must_use]
    pub fn contains_rect(&self, other: &Self) -> bool {
        // TODO(Shaohua): can we eliminate the this->isEmpty check?
        !other.is_empty()
            && !self.is_empty()
            && self.left <= other.left
            && self.top <= other.top
            && self.right >= other.right
            && self.bottom >= other.bottom
    }

    /// Returns true if Rect contains other rect.
    ///
    /// Returns false if Rect is empty or `other` is empty.
    /// Rect contains other rect when Rect area completely includes other rect area.
    #[must_use]
    #[allow(clippy::cast_precision_loss)]
    pub fn contains_irect(&self, other: &IRect) -> bool {
        // TODO(Shaohua): can we eliminate the this->isEmpty check?
        !other.is_empty()
            && !self.is_empty()
            && self.left <= other.left() as f32
            && self.top <= other.top() as f32
            && self.right >= other.right() as f32
            && self.bottom >= other.bottom() as f32
    }

    /// Sets `IRect` by adding 0.5 and discarding the fractional portion of Rect members, using
    /// `(left.round_to_int(), top.round_to_int(), right.round_to_int(), bottom.round_to_int())`.
    pub fn round_irect(&self, dst: &mut IRect) {
        dst.set_ltrb(
            self.left.round_to_int(),
            self.top.round_to_int(),
            self.right.round_to_int(),
            self.bottom.round_to_int(),
        );
    }

    /// Sets `IRect` by discarding the fractional portion of left and top;
    /// and rounding up right and bottom, using
    /// `(left.floor_to_int(), top.floor_to_int(), right.floor_to_int(), bottom.floor_to_int())`.
    pub fn round_out_irect(&self, dst: &mut IRect) {
        dst.set_ltrb(
            self.left.floor_to_int(),
            self.top.floor_to_int(),
            self.right.floor_to_int(),
            self.bottom.floor_to_int(),
        );
    }

    /// Sets Rect by discarding the fractional portion of left and top; and
    /// rounding up right and bottom, using
    /// `(left.floor(), top.floor(), right.floor(), bottom.floor())`.
    pub fn round_out(&self, dst: &mut Self) {
        dst.set_ltrb(
            self.left.floor(),
            self.top.floor(),
            self.right.floor(),
            self.bottom.floor(),
        );
    }

    /// Sets Rect by rounding up left and top; and discarding the fractional portion
    /// of right and bottom, using
    /// `(left.ceil_to_int(), top.ceil_to_int(), right.ceil_to_int(), bottom.ceil_to_int())`.
    pub fn round_in_irect(&self, dst: &mut IRect) {
        dst.set_ltrb(
            self.left.ceil_to_int(),
            self.top.ceil_to_int(),
            self.right.ceil_to_int(),
            self.bottom.ceil_to_int(),
        );
    }

    /// Returns `IRect` by adding 0.5 and discarding the fractional portion of Rect members,
    /// using `(left.round_to_int(), top.round_to_int(), right.round_to_int(), bottom.round_to_int())`.
    #[must_use]
    pub fn round_to_irect(&self) -> IRect {
        let mut ir = IRect::default();
        self.round_irect(&mut ir);
        ir
    }

    /// Sets `IRect` by discarding the fractional portion of left and top; and rounding
    /// up right and bottom, using
    /// `(left.floor_to_int(), top.floor_to_int(), right.floor_to_int(), bottom.floor_to_int())`.
    #[must_use]
    pub fn round_out_to_irect(&self) -> IRect {
        let mut ir = IRect::default();
        self.round_out_irect(&mut ir);
        ir
    }

    /// Sets `IRect` by rounding up left and top; and discarding the fractional portion
    /// of right and bottom, using
    /// `(left.ceil_to_int(), top.ceil_to_int(), right.ceil_to_int(), bottom.ceil_to_int())`.
    #[must_use]
    pub fn round_in_to_irect(&self) -> IRect {
        let mut ir = IRect::default();
        self.round_in_irect(&mut ir);
        ir
    }

    /// Swaps left and right if left is greater than right; and swaps
    /// top and bottom if top is greater than bottom.
    ///
    /// Result may be empty; and `width()` and `height()` will be zero or positive.
    pub fn sort(&mut self) {
        (self.left, self.right) = (self.left.min(self.right), self.left.max(self.right));
        (self.top, self.bottom) = (self.top.min(self.bottom), self.top.max(self.bottom));
    }

    /// Returns Rect with left and right swapped if left is greater than right; and
    /// with top and bottom swapped if top is greater than bottom.
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

    /// Returns pointer to first float in Rect, to treat it as an array with four entries.
    #[must_use]
    pub const fn as_scalars(&self) -> [Scalar; 4] {
        // TODO(Shaohua): Returns pointer to self.left
        //return &left;
        [self.left, self.top, self.right, self.bottom]
    }

    /// Writes text representation of Rect to standard output.
    ///
    /// Floating point values are written with limited precision; it may not be possible
    /// to reconstruct original Rect from output.
    ///
    /// Set `as_hex` to true to generate exact binary representations of floating point numbers.
    pub fn dump(&self, _as_hex: bool) {
        unimplemented!()
    }

    /// Writes text representation of Rect to standard output.
    ///
    /// Floating point values are written in hexadecimal to preserve their exact bit pattern.
    /// The output reconstructs the original Rect.
    pub fn dump_hex(&self) {
        self.dump(true);
    }
}
