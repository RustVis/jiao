// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

use core::ops;

use crate::base::Point;
use crate::base::Rect;

/// The Region struct specifies a clip region for a painter.
///
/// Region is used with `Painter::set_clip_region()` to limit the paint area to
/// what needs to be painted.
/// Region is the best tool for minimizing the amount of screen area to be updated by a repaint.
///
/// This class is not suitable for constructing shapes for rendering, especially as outlines.
/// Use `PainterPath` to create paths and shapes for use with Painter.
#[allow(clippy::module_name_repetitions)]
#[derive(Debug, Clone)]
pub struct Region {
    num_rects: usize,
    inner_area: usize,
    rects: Vec<Rect>,
    extents: Rect,
    inner_rect: Rect,
}

/// Specifies the shape of the region to be created.
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RegionType {
    /// The region covers the entire rectangle.
    Rectangle = 0,
    /// The region is an ellipse inside the rectangle.
    Ellipse = 1,
}

impl Default for Region {
    fn default() -> Self {
        Self::new()
    }
}

impl Region {
    /// Constructs an empty region.
    #[must_use]
    pub fn new() -> Self {
        Self {
            num_rects: 0,
            inner_area: 0,
            rects: Vec::new(),
            extents: Rect::new(),
            inner_rect: Rect::new(),
        }
    }

    /// Create a region based on the rectangle `rect`.
    ///
    /// If the rectangle is invalid a null region will be created.
    #[must_use]
    pub fn with_rect(rect: &Rect) -> Self {
        Self::with_rect_type(rect, RegionType::Rectangle)
    }

    /// Create a region based on the rectangle `rect` with region type t.
    ///
    /// If the rectangle is invalid a null region will be created.
    #[must_use]
    pub fn with_rect_type(_rect: &Rect, _t: RegionType) -> Self {
        unimplemented!()
    }

    /// Constructs a rectangular region.
    ///
    /// If t is Rectangle, the region is the filled rectangle (x, y, w, h).
    /// If t is Ellipse, the region is the filled ellipse with center at (x + w / 2, y + h / 2) and size (w ,h).
    #[must_use]
    pub fn with_int4(x: i32, y: i32, width: i32, height: i32) -> Self {
        Self::with_int4_type(x, y, width, height, RegionType::Rectangle)
    }

    /// Constructs a rectangular or elliptic region.
    ///
    /// If t is Rectangle, the region is the filled rectangle (x, y, w, h).
    /// If t is Ellipse, the region is the filled ellipse with center at (x + w / 2, y + h / 2) and size (w ,h).
    #[must_use]
    pub fn with_int4_type(x: i32, y: i32, width: i32, height: i32, t: RegionType) -> Self {
        Self::with_rect_type(&Rect::from(x, y, width, height), t)
    }

    /// Returns the bounding rectangle of this region.
    ///
    /// An empty region gives a rectangle that is `Rect::is_null()`.
    #[must_use]
    pub fn bounding_rect(&self) -> Rect {
        unimplemented!()
    }

    /// Returns true if the region contains the point; otherwise returns false.
    #[must_use]
    pub fn contains_point(&self, _point: &Point) -> bool {
        unimplemented!()
    }

    /// Returns true if the region overlaps the rectangle; otherwise returns false.
    #[must_use]
    pub fn contains_rect(&self, _rect: &Rect) -> bool {
        unimplemented!()
    }

    /// Returns a region which is the intersection of this region and other.
    #[must_use]
    pub fn intersected(&self, _other: &Self) -> Self {
        unimplemented!()
    }

    /// Returns a region which is the intersection of this region and the given rect.
    #[must_use]
    pub fn intersected_rect(&self, _rect: &Rect) -> Self {
        unimplemented!()
    }

    /// Returns true if this region intersects with region, otherwise returns false.
    #[must_use]
    pub fn intersects(&self, other: &Self) -> bool {
        if self.is_empty() || other.is_empty() {
            return false;
        }

        if !Self::rect_intersects(&self.bounding_rect(), &other.bounding_rect()) {
            return false;
        }
        if self.rect_count() == 1 && other.rect_count() == 1 {
            return true;
        }

        /*
        for (const QRect &myRect : *this)
            for (const QRect &otherRect : region)
                if (rect_intersects(myRect, otherRect))
                    return true;
        */
        false
    }

    /// Returns true if this region intersects with rect, otherwise returns false.
    #[must_use]
    pub fn intersects_rect(&self, _rect: &Rect) -> bool {
        unimplemented!()
    }

    /// Returns true if the region is empty; otherwise returns false.
    ///
    /// An empty region is a region that contains no points.
    #[must_use]
    pub const fn is_empty(&self) -> bool {
        self.num_rects == 0
    }

    /// Returns true if the region is empty; otherwise returns false.
    ///
    /// An empty region is a region that contains no points.
    ///
    /// This function is the same as `is_empty`.
    #[must_use]
    pub const fn is_null(&self) -> bool {
        self.num_rects == 0
    }

    /// Returns the number of rectangles that this region is composed of.
    #[must_use]
    pub const fn rect_count(&self) -> usize {
        self.num_rects
    }

    fn reset(&mut self) {
        self.num_rects = 0;
        self.inner_area = 0;
        self.rects.clear();
        self.extents = Rect::new();
        self.inner_rect = Rect::new();
    }

    fn update_inner_rect(&mut self, _rect: &Rect) {
        self.inner_area = 0;
        unimplemented!()
    }

    /// Sets the region using the array of rectangles specified by rects.
    ///
    /// The rectangles must be optimally Y-X sorted and follow these restrictions:
    /// - The rectangles must not intersect.
    /// - All rectangles with a given top coordinate must have the same height.
    /// - No two rectangles may abut horizontally (they should be combined into
    ///   a single wider rectangle in that case).
    /// - The rectangles must be sorted in ascending order, with Y as the major sort key
    ///   and X as the minor sort key.
    pub fn set_rects(&mut self, rects: &[Rect]) {
        self.reset();
        if rects.is_empty() {
            return;
        }

        let num = rects.len();
        self.num_rects = num;
        if num == 1 {
            self.extents = rects[0].clone();
            self.inner_rect = rects[0].clone();
        } else {
            self.rects.reserve(num);
            let mut left = i32::MAX;
            let mut top = i32::MAX;
            let mut right = i32::MAX;
            let mut bottom = i32::MAX;
            for rect in rects {
                self.rects.push(rect.clone());
                left = rect.left().min(left);
                right = rect.right().max(right);
                top = rect.top().min(top);
                bottom = rect.bottom().max(bottom);
                self.update_inner_rect(rect);
            }
            self.extents = Rect::from_points(Point::from(left, top), Point::from(right, bottom));
        }
    }

    /// Returns a region which is `other` subtracted from this region.
    #[must_use]
    pub fn subtracted(&self, _other: &Self) -> Self {
        unimplemented!()
    }

    /// Swaps region other with this region.
    ///
    /// This operation is very fast and never fails.
    pub fn swap(&mut self, _other: &mut Self) {
        unimplemented!()
    }

    /// Translates (moves) the region dx along the X axis and dy along the Y axis.
    pub fn translate(&mut self, _x: i32, _y: i32) {
        unimplemented!()
    }

    /// Translates the region point.x() along the x axis and point.y() along the y axis,
    /// relative to the current position.
    ///
    /// Positive values move the region to the right and down.
    /// Translates to the given point.
    pub fn translate_point(&mut self, point: &Point) {
        self.translate(point.x(), point.y());
    }

    /// Returns a copy of the region that is translated dx along the x axis and dy along the y axis,
    /// relative to the current position.
    ///
    /// Positive values move the region to the right and down.
    #[must_use]
    pub fn translated(&self, x: i32, y: i32) -> Self {
        let mut ret = self.clone();
        ret.translate(x, y);
        ret
    }

    /// Returns a copy of the regtion that is translated p.x() along the x axis and p.y() along the y axis,
    /// relative to the current position.
    ///
    /// Positive values move the rectangle to the right and down.
    #[must_use]
    pub fn translated_point(&self, point: &Point) -> Self {
        self.translated(point.x(), point.y())
    }

    /// Returns a region which is the union of this region and other.
    #[must_use]
    pub fn united(&self, _other: &Self) -> Self {
        unimplemented!()
    }

    /// Returns a region which is the union of this region and the given rect.
    #[must_use]
    pub fn united_rect(&self, _rect: &Rect) -> Self {
        unimplemented!()
    }

    /// Returns a region which is the exclusive or (XOR) of this region and other.
    #[must_use]
    pub fn xored(&self, _other: &Self) -> Self {
        unimplemented!()
    }

    const fn rect_intersects(r1: &Rect, r2: &Rect) -> bool {
        r1.right() >= r2.left()
            && r1.left() <= r2.right()
            && r1.bottom() >= r2.top()
            && r1.top() <= r2.bottom()
    }
}

impl ops::BitXor<&Region> for &Region {
    type Output = Region;
    /// Applies the united() function to this region and other.
    fn bitxor(self, other: &Region) -> Region {
        self.xored(other)
    }
}

impl ops::BitAnd<&Region> for &Region {
    type Output = Region;
    /// Applies the `intersected()` function to this region and other.
    ///
    /// `r1 & r2` is equivalent to `r1.intersected(r2)`.
    fn bitand(self, other: &Region) -> Region {
        self.intersected(other)
    }
}

impl ops::BitAnd<&Rect> for &Region {
    type Output = Region;
    /// Applies the `intersected()` function to this region and rect.
    ///
    /// `r1 & rect` is equivalent to `r1.intersected_rect(rect)`.
    fn bitand(self, rect: &Rect) -> Region {
        self.intersected_rect(rect)
    }
}

impl ops::Add<&Self> for Region {
    type Output = Self;
    /// Applies the united() function to this region and other.
    ///
    /// `r1 + r2` is equivalent to `r1.united(r2)`.
    fn add(self, other: &Self) -> Self {
        self.united(other)
    }
}

impl ops::Add<&Region> for &Region {
    type Output = Region;
    /// Applies the united() function to this region and other.
    ///
    /// `r1 + r2` is equivalent to `r1.united(r2)`.
    fn add(self, other: &Region) -> Region {
        self.united(other)
    }
}

impl ops::Add<&Rect> for &Region {
    type Output = Region;
    /// Applies the united() function to this region and rect.
    ///
    /// `r1 + rect` is equivalent to `r1.united_rect(rect)`.
    fn add(self, rect: &Rect) -> Region {
        self.united_rect(rect)
    }
}

impl ops::Sub<&Region> for &Region {
    type Output = Region;
    /// Applies the `subtracted()` function to this region and other.
    ///
    /// `r1 - r2` is equivalent to `r1.subtracted(r2)`.
    fn sub(self, other: &Region) -> Region {
        self.subtracted(other)
    }
}

impl ops::AddAssign<&Self> for Region {
    fn add_assign(&mut self, other: &Self) {
        *self = self.united(other);
    }
}

impl ops::SubAssign<&Self> for Region {
    fn sub_assign(&mut self, other: &Self) {
        *self = self.subtracted(other);
    }
}

impl ops::BitAndAssign<&Self> for Region {
    fn bitand_assign(&mut self, other: &Self) {
        *self = self.intersected(other);
    }
}

impl ops::BitOrAssign<&Self> for Region {
    fn bitor_assign(&mut self, other: &Self) {
        *self = self.united(other);
    }
}

impl ops::BitXorAssign<&Self> for Region {
    fn bitxor_assign(&mut self, other: &Self) {
        *self = self.xored(other);
    }
}
