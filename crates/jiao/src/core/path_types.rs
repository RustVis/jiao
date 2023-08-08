// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

use bitflags::bitflags;

#[repr(u8)]
#[derive(Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum PathFillType {
    /// Specifies that "inside" is computed by a non-zero sum of signed edge crossings.
    Winding = 0,

    /// Specifies that "inside" is computed by an odd number of edge crossings.
    EvenOdd,

    /// Same as Winding, but draws outside of the path, rather than inside.
    InverseWinding,

    /// Same as EvenOdd, but draws outside of the path, rather than inside.
    InverseEvenOdd,
}

impl PathFillType {
    #[must_use]
    pub const fn is_event_odd(self) -> bool {
        let num = self as u8;
        num & 1 != 0
    }

    #[must_use]
    pub const fn is_inverse(self) -> bool {
        let num = self as u8;
        num & 2 != 0
    }

    #[must_use]
    pub const fn inverse(self) -> Self {
        match self {
            Self::Winding => Self::InverseWinding,
            Self::EvenOdd => Self::InverseEvenOdd,
            Self::InverseWinding => Self::Winding,
            Self::InverseEvenOdd => Self::EvenOdd,
        }
    }

    #[must_use]
    pub const fn convert_to_non_inverse(self) -> Self {
        match self {
            Self::Winding => Self::InverseWinding,
            Self::EvenOdd => Self::InverseEvenOdd,
            _ => self,
        }
    }
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum PathDirection {
    /// clockwise direction for adding closed contours
    CW,

    /// counter-clockwise direction for adding closed contours
    CCW,
}

bitflags! {
    #[derive(Debug, Default, Clone, Copy, Eq, PartialEq, Hash)]
    pub struct PathSegmentMask : u8 {
        const Line = 1 << 0;
        const Quad = 1 << 1;
        const Conic = 1 << 2;
        const Cubic = 1 << 3;
    }
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum PathVerb {
    /// Iter returns 1 point.
    Move,

    /// Iter returns 2 points
    Line,

    /// Iter returns 3 points
    Quad,

    /// Iter returns 3 points + 1 weight
    Conic,

    /// Iter returns 4 points
    Cubic,

    /// Iter returns 0 points
    Close,
}

impl PathVerb {
    /// Get number of points of a path verb.
    #[must_use]
    #[allow(clippy::match_same_arms)]
    pub const fn points(self) -> usize {
        match self {
            Self::Move => 1,
            Self::Line => 2,
            Self::Quad => 3,
            Self::Conic => 3,
            Self::Cubic => 4,
            Self::Close => 0,
        }
    }
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum ArcSize {
    /// smaller of arc pair
    Small,
    /// larger of arc pair
    Large,
}
