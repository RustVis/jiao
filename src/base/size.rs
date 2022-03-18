// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

use core::{mem, ops};
use serde::{Deserialize, Serialize};

use super::aspect_ratio_mode::AspectRatioMode;

/// The Size struct defines the size of a two-dimensional object using integer point precision.
///
/// A size is specified by a `width()` and a `height()`. It can be set in the constructor
/// and changed using the `set_width()`, `set_height()`, or `scale()` functions,
/// or using arithmetic operators.
///
/// A size can also be manipulated directly by retrieving references to the width and height
/// using the `rwidth()` and `rheight()` functions.
///
/// Finally, the width and height can be swapped using the `transpose()` function.
///
/// The `is_valid()` function determines if a size is valid (a valid size has both width and
/// height greater than or equal to zero).
///
/// The `is_empty()` function returns true if either of the width and height is less than,
/// or equal to, zero, while the `is_null()` function returns true only if both the width
/// and the height is zero.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Size {
    width: i32,
    height: i32,
}

impl Size {
    /// Constructs a size with an invalid width and height (i.e., `is_valid()` returns false).
    pub fn new() -> Self {
        Self {
            width: 0,
            height: 0,
        }
    }

    /// Constructs a size with the given width and height.
    pub fn from(width: i32, height: i32) -> Self {
        Self { width, height }
    }

    // Returns the size that results from shrinking this size by margins.
    //pub fn shrunk_by(&self, margins: &Margins) -> Self {
    //    unimplemented!()
    //}

    // Returns the size that results from growing this size by margins.
    //pub fn grown_by(&self, margins: &Margins) -> Self {
    //    unimplemented!()
    //}

    /// Returns a size holding the minimum width and height of this size and the given `other`.
    pub fn bounded_to(&self, other: &Self) -> Self {
        unimplemented!()
    }

    /// Returns a size holding the maximum width and height of this size and the given `other`.
    pub fn expanded_to(&self, other: &Self) -> Self {
        unimplemented!()
    }

    /// Returns the height.
    pub fn height(&self) -> i32 {
        self.height
    }

    /// Returns true if either of the width and height is less than or equal to 0;
    /// otherwise returns false.
    pub fn is_empty(&self) -> bool {
        self.width == 0 || self.height == 0
    }

    /// Returns true if both the width and height is 0; otherwise returns false.
    pub fn is_null(&self) -> bool {
        self.width == 0 && self.height == 0
    }

    /// Returns true if both the width and height is equal to or greater than 0;
    /// otherwise returns false.
    pub fn is_valid(&self) -> bool {
        self.width >= 0 || self.height >= 0
    }

    /// Returns a reference to the height.
    pub fn rheight(&mut self) -> &mut i32 {
        &mut self.height
    }

    /// Returns a reference to the width.
    pub fn rwidth(&mut self) -> &mut i32 {
        &mut self.width
    }

    /// Scales the size to a rectangle with the given width and height, according to the specified `mode`.
    pub fn scale(&mut self, width: i32, height: i32, mode: AspectRatioMode) {
        unimplemented!()
    }

    /// Return a size scaled to a rectangle with the given width and height, according to the specified `mode`.
    pub fn scaled(&self, width: i32, height: i32, mode: AspectRatioMode) -> Self {
        unimplemented!()
    }

    /// Sets the height to the given height.
    pub fn set_height(&mut self, height: i32) {
        self.height = height;
    }

    /// Sets the width to the given width.
    pub fn set_width(&mut self, width: i32) {
        self.width = width;
    }

    /// Swaps the width and height values.
    pub fn transpose(&mut self) {
        mem::swap(&mut self.width, &mut self.height);
    }

    /// Returns a Size with width and height swapped.
    pub fn transposed(&self) -> Self {
        Self {
            width: self.height,
            height: self.width,
        }
    }

    /// Returns the width.
    pub fn width(&self) -> i32 {
        self.width
    }
}

impl ops::AddAssign<Size> for Size {
    fn add_assign(&mut self, other: Size) {
        self.width += other.width;
        self.height += other.height;
    }
}

impl ops::AddAssign<&Size> for Size {
    fn add_assign(&mut self, other: &Size) {
        self.width += other.width;
        self.height += other.height;
    }
}

impl ops::Add<Size> for Size {
    type Output = Size;

    fn add(self, other: Self) -> Self {
        Self {
            width: self.width + other.width,
            height: self.height + other.height,
        }
    }
}

impl ops::Add<&Size> for &Size {
    type Output = Size;

    fn add(self, other: &Size) -> Size {
        Size {
            width: self.width + other.width,
            height: self.height + other.height,
        }
    }
}

impl ops::Mul<f64> for &Size {
    type Output = Size;

    /// Multiplies the given size by the given `factor`,
    /// and returns the result rounded to the nearest integer.
    fn mul(self, factor: f64) -> Self::Output {
        Self::Output {
            width: (self.width as f64 * factor).round() as i32,
            height: (self.height as f64 * factor).round() as i32,
        }
    }
}

impl ops::MulAssign<f64> for Size {
    /// Multiplies the given size by the given `factor`,
    /// and returns the result rounded to the nearest integer.
    fn mul_assign(&mut self, factor: f64) {
        self.width = (self.width as f64 * factor).round() as i32;
        self.height = (self.height as f64 * factor).round() as i32;
    }
}

impl ops::Div<f64> for &Size {
    type Output = Size;

    /// Divides the given size by the given `divisor`,
    /// and returns the result rounded to the nearest integer.
    fn div(self, divisor: f64) -> Self::Output {
        assert!(divisor != 0.0);
        Self::Output {
            width: (self.width as f64 / divisor).round() as i32,
            height: (self.height as f64 / divisor).round() as i32,
        }
    }
}

impl ops::DivAssign<f64> for Size {
    /// Divides the given size by the given `divisor`,
    /// and returns the result rounded to the nearest integer.
    fn div_assign(&mut self, divisor: f64) {
        assert!(divisor != 0.0);
        self.width = (self.width as f64 / divisor).round() as i32;
        self.height = (self.height as f64 / divisor).round() as i32;
    }
}
