// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

use core::ops;
use serde::{Deserialize, Serialize};

/// The Margins struct defines the four margins of a rectangle.
///
/// Margin defines a set of four margins; left, top, right and bottom, that describe
/// the size of the borders surrounding a rectangle.
/// The `is_null()` function returns true only if all margins are set to zero.
#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Margins {
    top: i32,
    right: i32,
    bottom: i32,
    left: i32,
}

impl Margins {
    /// Constructs a margins object with all margins set to 0.
    pub fn new() -> Self {
        Self::from(0, 0, 0, 0)
    }

    /// Constructs margins with the given `left`, `top`, `right`, `bottom`
    pub fn from(top: i32, right: i32, bottom: i32, left: i32) -> Self {
        Self {
            top,
            right,
            bottom,
            left,
        }
    }

    /// Returns the bottom margin.
    pub fn bottom(&self) -> i32 {
        self.bottom
    }

    /// Returns true if all margins are is 0; otherwise returns false.
    pub fn is_null(&self) -> bool {
        self.top == 0 && self.right == 0 && self.bottom == 0 && self.left == 0
    }

    /// Returns the left margin.
    pub fn left(&self) -> i32 {
        self.left
    }

    /// Returns the right margin.
    pub fn right(&self) -> i32 {
        self.right
    }

    /// Sets the bottom margin to bottom.
    pub fn set_bottom(&mut self, bottom: i32) {
        self.bottom = bottom;
    }

    /// Sets the left margin to left.
    pub fn set_left(&mut self, left: i32) {
        self.left = left;
    }

    /// Sets the right margin to right.
    pub fn set_right(&mut self, right: i32) {
        self.right = right;
    }

    /// Sets the Top margin to Top.
    pub fn set_top(&mut self, top: i32) {
        self.top = top;
    }

    /// Returns the top margin.
    pub fn top(&self) -> i32 {
        self.top
    }
}

impl ops::MulAssign<i32> for Margins {
    /// Multiplies each component of this object by factor.
    fn mul_assign(&mut self, factor: i32) {
        self.top *= factor;
        self.right *= factor;
        self.bottom *= factor;
        self.left *= factor;
    }
}

impl ops::MulAssign<f64> for Margins {
    /// Multiplies each component of this object by factor.
    fn mul_assign(&mut self, factor: f64) {
        self.top = (self.top as f64 * factor).round() as i32;
        self.right = (self.right as f64 * factor).round() as i32;
        self.bottom = (self.bottom as f64 * factor).round() as i32;
        self.left = (self.left as f64 * factor).round() as i32;
    }
}

impl ops::AddAssign<&Margins> for Margins {
    /// Add each component of margins to the respective component of this object.
    fn add_assign(&mut self, other: &Margins) {
        self.top += other.top;
        self.right += other.right;
        self.bottom += other.bottom;
        self.left += other.left;
    }
}

impl ops::AddAssign<i32> for Margins {
    /// Adds the `addend` to each component of this object.
    fn add_assign(&mut self, addend: i32) {
        self.top += addend;
        self.right += addend;
        self.bottom += addend;
        self.left += addend;
    }
}

impl ops::SubAssign<&Margins> for Margins {
    /// Subtract each component of margins from the respective component of this object.
    fn sub_assign(&mut self, other: &Margins) {
        self.top -= other.top;
        self.right -= other.right;
        self.bottom -= other.bottom;
        self.left -= other.left;
    }
}

impl ops::SubAssign<i32> for Margins {
    /// Subtracts the `subtrahend` from each component of this object.
    fn sub_assign(&mut self, subtrahend: i32) {
        self.top -= subtrahend;
        self.right -= subtrahend;
        self.bottom -= subtrahend;
        self.left -= subtrahend;
    }
}

impl ops::DivAssign<i32> for Margins {
    /// Divides each component of this object by `divisor`.
    fn div_assign(&mut self, divisor: i32) {
        assert!(divisor != 0);
        self.top /= divisor;
        self.right /= divisor;
        self.bottom /= divisor;
        self.left /= divisor;
    }
}

impl ops::DivAssign<f64> for Margins {
    /// Divides each component of this object by `divisor`.
    fn div_assign(&mut self, divisor: f64) {
        assert!(divisor != 0.0);
        self.top = (self.top as f64 / divisor).round() as i32;
        self.right = (self.right as f64 / divisor).round() as i32;
        self.bottom = (self.bottom as f64 / divisor).round() as i32;
        self.left = (self.left as f64 / divisor).round() as i32;
    }
}

impl ops::Add<&Margins> for &Margins {
    type Output = Margins;

    /// Returns a Margin object that is formed from all components of margins.
    fn add(self, other: &Margins) -> Margins {
        Margins {
            top: self.top + other.top,
            right: self.right + other.right,
            bottom: self.bottom + other.bottom,
            left: self.left + other.left,
        }
    }
}

impl ops::Add<i32> for &Margins {
    type Output = Margins;

    /// Returns a Margin object that is formed from all components of margins.
    fn add(self, addend: i32) -> Margins {
        Margins {
            top: self.top + addend,
            right: self.right + addend,
            bottom: self.bottom + addend,
            left: self.left + addend,
        }
    }
}

impl ops::Sub<&Margins> for &Margins {
    type Output = Margins;

    /// Returns a Margins object that is formed by subtracting `other` from self;
    /// each component is subtracted separately.
    fn sub(self, other: &Margins) -> Margins {
        Margins {
            top: self.top - other.top,
            right: self.right - other.right,
            bottom: self.bottom - other.bottom,
            left: self.left - other.left,
        }
    }
}

impl ops::Sub<i32> for &Margins {
    type Output = Margins;

    /// Returns a QMargins object that is formed by subtracting `subtracted` from self.
    fn sub(self, subtrahend: i32) -> Margins {
        Margins {
            top: self.top - subtrahend,
            right: self.right - subtrahend,
            bottom: self.bottom - subtrahend,
            left: self.left - subtrahend,
        }
    }
}

impl ops::Div<i32> for &Margins {
    type Output = Margins;

    /// Returns a QMargins object that is formed by dividing the components of the given margins by the given `divisor`.
    fn div(self, divisor: i32) -> Margins {
        assert!(divisor != 0);
        Margins {
            top: self.top / divisor,
            right: self.right / divisor,
            bottom: self.bottom / divisor,
            left: self.left / divisor,
        }
    }
}

/// The MarginsF class defines the four margins of a rectangle.
///
/// Margin defines a set of four margins; left, top, right and bottom, that describe
/// the size of the borders surrounding a rectangle.
/// The `is_null()` function returns true only if all margins are set to zero.
#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
pub struct MarginsF {
    top: f64,
    right: f64,
    bottom: f64,
    left: f64,
}

impl MarginsF {
    /// Constructs a margins object with all margins set to 0.
    pub fn new() -> Self {
        Self::from(0.0, 0.0, 0.0, 0.0)
    }

    /// Constructs margins with the given `left`, `top`, `right`, `bottom`
    pub fn from(top: f64, right: f64, bottom: f64, left: f64) -> Self {
        Self {
            top,
            right,
            bottom,
            left,
        }
    }

    /// Returns the bottom margin.
    pub fn bottom(&self) -> f64 {
        self.bottom
    }

    /// Returns true if all margins are is 0; otherwise returns false.
    pub fn is_null(&self) -> bool {
        self.top == 0.0 && self.right == 0.0 && self.bottom == 0.0 && self.left == 0.0
    }

    /// Returns the left margin.
    pub fn left(&self) -> f64 {
        self.left
    }

    /// Returns the right margin.
    pub fn right(&self) -> f64 {
        self.right
    }

    /// Sets the bottom margin to bottom.
    pub fn set_bottom(&mut self, bottom: f64) {
        self.bottom = bottom;
    }

    /// Sets the left margin to left.
    pub fn set_left(&mut self, left: f64) {
        self.left = left;
    }

    /// Sets the right margin to right.
    pub fn set_right(&mut self, right: f64) {
        self.right = right;
    }

    /// Sets the Top margin to Top.
    pub fn set_top(&mut self, top: f64) {
        self.top = top;
    }

    /// Returns the top margin.
    pub fn top(&self) -> f64 {
        self.top
    }
}

impl ops::MulAssign<f64> for MarginsF {
    /// Multiplies each component of this object by factor.
    fn mul_assign(&mut self, factor: f64) {
        self.top *= factor;
        self.right *= factor;
        self.bottom *= factor;
        self.left *= factor;
    }
}

impl ops::AddAssign<&MarginsF> for MarginsF {
    /// Add each component of margins to the respective component of this object.
    fn add_assign(&mut self, other: &MarginsF) {
        self.top += other.top;
        self.right += other.right;
        self.bottom += other.bottom;
        self.left += other.left;
    }
}

impl ops::AddAssign<f64> for MarginsF {
    /// Adds the `addend` to each component of this object.
    fn add_assign(&mut self, addend: f64) {
        self.top += addend;
        self.right += addend;
        self.bottom += addend;
        self.left += addend;
    }
}

impl ops::SubAssign<&MarginsF> for MarginsF {
    /// Subtract each component of margins from the respective component of this object.
    fn sub_assign(&mut self, other: &MarginsF) {
        self.top -= other.top;
        self.right -= other.right;
        self.bottom -= other.bottom;
        self.left -= other.left;
    }
}

impl ops::SubAssign<f64> for MarginsF {
    /// Subtracts the `subtrahend` from each component of this object.
    fn sub_assign(&mut self, subtrahend: f64) {
        self.top -= subtrahend;
        self.right -= subtrahend;
        self.bottom -= subtrahend;
        self.left -= subtrahend;
    }
}

impl ops::DivAssign<f64> for MarginsF {
    /// Divides each component of this object by `divisor`.
    fn div_assign(&mut self, divisor: f64) {
        assert!(divisor != 0.0);
        self.top /= divisor;
        self.right /= divisor;
        self.bottom /= divisor;
        self.left /= divisor;
    }
}

impl ops::Add<&MarginsF> for &MarginsF {
    type Output = MarginsF;

    /// Returns a Margin object that is formed from all components of margins.
    fn add(self, other: &MarginsF) -> MarginsF {
        MarginsF {
            top: self.top + other.top,
            right: self.right + other.right,
            bottom: self.bottom + other.bottom,
            left: self.left + other.left,
        }
    }
}

impl ops::Add<f64> for &MarginsF {
    type Output = MarginsF;

    /// Returns a Margin object that is formed from all components of margins.
    fn add(self, addend: f64) -> MarginsF {
        MarginsF {
            top: self.top + addend,
            right: self.right + addend,
            bottom: self.bottom + addend,
            left: self.left + addend,
        }
    }
}

impl ops::Sub<&MarginsF> for &MarginsF {
    type Output = MarginsF;

    /// Returns a MarginsF object that is formed by subtracting `other` from self;
    /// each component is subtracted separately.
    fn sub(self, other: &MarginsF) -> MarginsF {
        MarginsF {
            top: self.top - other.top,
            right: self.right - other.right,
            bottom: self.bottom - other.bottom,
            left: self.left - other.left,
        }
    }
}

impl ops::Sub<f64> for &MarginsF {
    type Output = MarginsF;

    /// Returns a QMarginsF object that is formed by subtracting `subtracted` from self.
    fn sub(self, subtrahend: f64) -> MarginsF {
        MarginsF {
            top: self.top - subtrahend,
            right: self.right - subtrahend,
            bottom: self.bottom - subtrahend,
            left: self.left - subtrahend,
        }
    }
}

impl ops::Div<f64> for &MarginsF {
    type Output = MarginsF;

    /// Returns a QMarginsF object that is formed by dividing the components of the given margins by the given `divisor`.
    fn div(self, divisor: f64) -> MarginsF {
        assert!(divisor != 0.0);
        MarginsF {
            top: self.top / divisor,
            right: self.right / divisor,
            bottom: self.bottom / divisor,
            left: self.left / divisor,
        }
    }
}
