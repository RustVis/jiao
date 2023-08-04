// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

#![allow(clippy::module_name_repetitions)]

use crate::core::size::ISize;

/// `IRect` holds four 32-bit integer coordinates describing the upper and
/// lower bounds of a rectangle.
///
/// `IRect` may be created from outer bounds or from position, width, and height.
/// `IRect` describes an area; if its right is less than or equal to its left,
/// or if its bottom is less than or equal to its top, it is considered empty.
#[derive(Debug, Default, Clone, Eq, PartialEq)]
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

impl IRect {
    /// Returns constructed `IRect` set to (0, 0, size.width(), size.height()).
    ///
    /// Does not validate input; size.width() or size.height() may be negative.
    #[must_use]
    pub const fn make_size(size: ISize) -> Self {
        Self {
            left: 0,
            top: 0,
            right: size.width(),
            bottom: size.height(),
        }
    }
}
