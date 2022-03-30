// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

use core::ops;

use crate::base::point::Point;
use crate::base::rect::Rect;

/// The Region struct specifies a clip region for a painter.
///
/// Region is used with `Painter::set_clip_region()` to limit the paint area to
/// what needs to be painted.
/// Region is the best tool for minimizing the amount of screen area to be updated by a repaint.
///
/// This class is not suitable for constructing shapes for rendering, especially as outlines.
/// Use PainterPath to create paths and shapes for use with Painter.
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
pub enum RegionType {
    /// The region covers the entire rectangle.
    Rectangle = 0,
    /// The region is an ellipse inside the rectangle.
    Ellipse = 1,
}

impl Region {
    /// Constructs an empty region.
    pub fn new() -> Self {
        unimplemented!()
    }

    /// Create a region based on the rectangle `rect`.
    ///
    /// If the rectangle is invalid a null region will be created.
    pub fn with_rect(rect: &Rect) -> Self {
        Self::with_rect_type(rect, RegionType::Rectangle)
    }

    /// Create a region based on the rectangle `rect` with region type t.
    ///
    /// If the rectangle is invalid a null region will be created.
    pub fn with_rect_type(_rect: &Rect, _t: RegionType) -> Self {
        unimplemented!()
    }

    /// Constructs a rectangular region.
    ///
    /// If t is Rectangle, the region is the filled rectangle (x, y, w, h).
    /// If t is Ellipse, the region is the filled ellipse with center at (x + w / 2, y + h / 2) and size (w ,h).
    pub fn with_int4(x: i32, y: i32, width: i32, height: i32) -> Self {
        Self::with_int4_type(x, y, width, height, RegionType::Rectangle)
    }

    /// Constructs a rectangular or elliptic region.
    ///
    /// If t is Rectangle, the region is the filled rectangle (x, y, w, h).
    /// If t is Ellipse, the region is the filled ellipse with center at (x + w / 2, y + h / 2) and size (w ,h).
    pub fn with_int4_type(x: i32, y: i32, width: i32, height: i32, t: RegionType) -> Self {
        Self::with_rect_type(&Rect::from(x, y, width, height), t)
    }

    /// Returns the bounding rectangle of this region.
    ///
    /// An empty region gives a rectangle that is `Rect::is_null()`.
    pub fn bounding_rect(&self) -> Rect {
        unimplemented!()
    }

    /// Returns true if the region contains the point; otherwise returns false.
    pub fn contains_point(&self, point: &Point) -> bool {
        unimplemented!()
    }

    /// Returns true if the region overlaps the rectangle; otherwise returns false.
    pub fn contains_rect(&self, rect: &Rect) -> bool {
        unimplemented!()
    }

    /// Returns a region which is the intersection of this region and other.
    pub fn intersected(&self, other: &Self) -> Self {
        unimplemented!()
    }

    /// Returns a region which is the intersection of this region and the given rect.
    pub fn intersected_rect(&self, rect: &Rect) -> Self {
        unimplemented!()
    }

    /// Returns true if this region intersects with region, otherwise returns false.
    pub fn intersects(&self, other: &Self) -> bool {
        unimplemented!()
    }

    /// Returns true if this region intersects with rect, otherwise returns false.
    pub fn intersects_rect(&self, rect: &Rect) -> bool {
        unimplemented!()
    }

    /// Returns true if the region is empty; otherwise returns false.
    ///
    /// An empty region is a region that contains no points.
    pub fn is_empty(&self) -> bool {
        unimplemented!()
    }

    /// Returns true if the region is empty; otherwise returns false.
    ///
    /// An empty region is a region that contains no points.
    ///
    /// This function is the same as `is_empty`.
    pub fn is_null(&self) -> bool {
        unimplemented!()
    }

    /// Returns the number of rectangles that this region is composed of.
    pub fn rect_count(&self) -> usize {
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
        unimplemented!()
    }

    /// Returns a region which is `other` subtracted from this region.
    pub fn subtracted(&self, other: &Self) -> Self {
        unimplemented!()
    }

    /// Swaps region other with this region.
    ///
    /// This operation is very fast and never fails.
    pub fn swap(&mut self, other: &mut Self) {
        unimplemented!()
    }

    /// Translates (moves) the region dx along the X axis and dy along the Y axis.
    pub fn translate(&mut self, x: i32, y: i32) {
        unimplemented!()
    }

    /// Translates the region point.x() along the x axis and point.y() along the y axis,
    /// relative to the current position.
    ///
    /// Positive values move the region to the right and down.
    /// Translates to the given point.
    pub fn translate_point(&mut self, point: &Point) {
        unimplemented!()
    }

    /// Returns a copy of the region that is translated dx along the x axis and dy along the y axis,
    /// relative to the current position.
    ///
    /// Positive values move the region to the right and down.
    pub fn translated(&self, x: i32, y: i32) -> Self {
        unimplemented!()
    }

    /// Returns a copy of the regtion that is translated p.x() along the x axis and p.y() along the y axis,
    /// relative to the current position.
    ///
    /// Positive values move the rectangle to the right and down.
    pub fn translated_point(&self, point: &Point) -> Self {
        unimplemented!()
    }

    /// Returns a region which is the union of this region and other.
    pub fn united(&self, other: &Self) -> Self {
        unimplemented!()
    }

    /// Returns a region which is the union of this region and the given rect.
    pub fn united_rect(&self, rect: &Rect) -> Self {
        unimplemented!()
    }

    /// Returns a region which is the exclusive or (XOR) of this region and other.
    pub fn xored(&self, other: &Self) -> Self {
        unimplemented!()
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

impl ops::Add<&Region> for Region {
    type Output = Region;
    /// Applies the united() function to this region and other.
    ///
    /// `r1 + r2` is equivalent to `r1.united(r2)`.
    fn add(self, other: &Region) -> Region {
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

impl ops::AddAssign<&Region> for Region {
    fn add_assign(&mut self, other: &Region) {
        *self = self.united(other);
    }
}

impl ops::SubAssign<&Region> for Region {
    fn sub_assign(&mut self, other: &Region) {
        *self = self.subtracted(other);
    }
}

impl ops::BitAndAssign<&Region> for Region {
    fn bitand_assign(&mut self, other: &Region) {
        *self = self.intersected(other);
    }
}

impl ops::BitOrAssign<&Region> for Region {
    fn bitor_assign(&mut self, other: &Region) {
        *self = self.united(other);
    }
}

impl ops::BitXorAssign<&Region> for Region {
    fn bitxor_assign(&mut self, other: &Region) {
        *self = self.xored(other);
    }
}
