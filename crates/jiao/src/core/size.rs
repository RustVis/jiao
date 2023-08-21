// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

#![allow(clippy::module_name_repetitions)]

use crate::core::scalar::{Scalar, ScalarExt};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ISize {
    width: i32,
    height: i32,
}

impl Default for ISize {
    fn default() -> Self {
        Self::new()
    }
}

impl ISize {
    #[must_use]
    pub const fn new() -> Self {
        Self {
            width: 0,
            height: 0,
        }
    }

    #[must_use]
    pub const fn from_wh(width: i32, height: i32) -> Self {
        Self { width, height }
    }

    #[inline]
    pub fn set(&mut self, width: i32, height: i32) {
        self.width = width;
        self.height = height;
    }

    /// Returns true iff width == 0 && height == 0
    #[must_use]
    pub const fn is_zero(&self) -> bool {
        self.width == 0 && self.height == 0
    }

    /// Returns true if either width or height are <= 0
    #[must_use]
    pub const fn is_empty(&self) -> bool {
        self.width <= 0 || self.height <= 0
    }

    /// Set the width and height to 0
    pub fn set_empty(&mut self) {
        self.width = 0;
        self.height = 0;
    }

    #[must_use]
    pub const fn width(&self) -> i32 {
        self.width
    }

    #[must_use]
    pub const fn height(&self) -> i32 {
        self.height
    }

    #[must_use]
    pub const fn area(&self) -> i64 {
        self.width as i64 * self.height as i64
    }

    #[must_use]
    pub const fn equals(&self, width: i32, height: i32) -> bool {
        self.width == width && self.height == height
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Size {
    width: Scalar,
    height: Scalar,
}

impl Default for Size {
    fn default() -> Self {
        Self::new()
    }
}

impl Size {
    #[must_use]
    pub const fn new() -> Self {
        Self {
            width: 0.0,
            height: 0.0,
        }
    }

    #[must_use]
    pub const fn from_wh(width: Scalar, height: Scalar) -> Self {
        Self { width, height }
    }

    #[must_use]
    #[allow(clippy::cast_precision_loss)]
    pub const fn from_isize(src: &ISize) -> Self {
        Self {
            width: src.width() as Scalar,
            height: src.height() as Scalar,
        }
    }

    pub fn set(&mut self, width: Scalar, height: Scalar) {
        self.width = width;
        self.height = height;
    }

    /// Returns true iff width == 0 && height == 0
    #[must_use]
    pub fn is_zero(&self) -> bool {
        self.width == 0.0 && self.height == 0.0
    }

    /// Returns true if either width or height are <= 0
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.width <= 0.0 || self.height <= 0.0
    }

    /// Set the width and height to 0
    pub fn set_empty(&mut self) {
        self.width = 0.0;
        self.height = 0.0;
    }

    #[must_use]
    pub const fn width(&self) -> Scalar {
        self.width
    }

    #[must_use]
    pub const fn height(&self) -> Scalar {
        self.height
    }

    #[must_use]
    pub fn equals(&self, width: Scalar, height: Scalar) -> bool {
        self.width.fuzzy_equal(width) && self.height.fuzzy_equal(height)
    }

    #[must_use]
    pub fn to_round(&self) -> ISize {
        ISize::from_wh(self.width.round_to_int(), self.height.round_to_int())
    }

    #[must_use]
    pub fn to_ceil(&self) -> ISize {
        ISize::from_wh(self.width.ceil_to_int(), self.height.ceil_to_int())
    }

    #[must_use]
    pub fn to_floor(&self) -> ISize {
        ISize::from_wh(self.width.floor_to_int(), self.height.floor_to_int())
    }
}
