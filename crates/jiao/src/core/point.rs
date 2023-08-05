// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

use std::ops::{Add, AddAssign, Neg, Sub, SubAssign};

/// `IVector` provides an alternative name for `IPoint`.
///
/// `IVector` and `IPoint` can be used interchangeably for all purposes.
pub type IVector = IPoint;

/// `IPoint` holds two 32-bit integer coordinates.
#[derive(Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct IPoint {
    /// x-axis value
    x: i32,
    /// y-axis value
    y: i32,
}

impl IPoint {
    #[must_use]
    pub const fn make(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    /// Returns x-axis value of `IPoint`.
    #[must_use]
    pub const fn x(&self) -> i32 {
        self.x
    }

    /// Returns y-axis value of `IPoint`.
    #[must_use]
    pub const fn y(&self) -> i32 {
        self.y
    }

    /// Returns true if x and y are both zero.
    #[must_use]
    pub const fn is_zero(&self) -> bool {
        self.x | self.y == 0
    }

    /// Sets new x and y.
    pub fn set(&mut self, x: i32, y: i32) {
        self.x = x;
        self.y = y;
    }

    #[must_use]
    pub const fn equals(&self, x: i32, y: i32) -> bool {
        self.x == x && self.y == y
    }
}

impl Add<Self> for IPoint {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl AddAssign<Self> for IPoint {
    fn add_assign(&mut self, other: Self) {
        self.x += other.x;
        self.y += other.y;
    }
}

impl Sub<Self> for IPoint {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl SubAssign<Self> for IPoint {
    fn sub_assign(&mut self, other: Self) {
        self.x -= other.x;
        self.y -= other.y;
    }
}

impl Neg for IPoint {
    type Output = Self;

    fn neg(self) -> Self {
        Self {
            x: -self.x,
            y: -self.y,
        }
    }
}
