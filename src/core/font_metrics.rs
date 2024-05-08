// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that can be found
// in the LICENSE file.

use bitflags::bitflags;

use crate::core::scalar::Scalar;

/// `FontMetrics` represents the metrics of a Font.
///
/// The metric values are consistent with the y-down coordinate system.
#[derive(Debug, Clone, PartialEq)]
pub struct FontMetrics {
    /// `FontMetricsFlags` indicating which metrics are valid
    pub flags: FontMetricsFlags,

    /// Greatest extent above origin of any glyph bounding box, typically negative;
    /// deprecated with variable fonts.
    pub top: Scalar,

    /// Distance to reserve above baseline, typically negative.
    pub ascent: Scalar,

    /// Distance to reserve below baseline, typically positive.
    pub descent: Scalar,

    /// Greatest extent below origin of any glyph bounding box, typically positive;
    /// deprecated with variable fonts.
    pub bottom: Scalar,

    /// Distance to add between lines, typically positive or zero.
    pub leading: Scalar,

    /// Average character width, zero if unknown.
    pub avg_char_width: Scalar,

    /// Maximum character width, zero if unknown.
    pub max_char_width: Scalar,

    /// Greatest extent to left of origin of any glyph bounding box, typically negative;
    /// deprecated with variable fonts.
    pub x_min: Scalar,

    /// Greatest extent to right of origin of any glyph bounding box, typically positive;
    /// deprecated with variable fonts.
    pub x_max: Scalar,

    /// Height of lower-case 'x', zero if unknown, typically negative.
    pub x_height: Scalar,

    /// Height of an upper-case letter, zero if unknown, typically negative.
    pub cap_height: Scalar,

    /// Underline thickness.
    pub underline_thickness: Scalar,

    /// Distance from baseline to top of stroke, typically positive.
    pub underline_position: Scalar,

    /// Strikeout thickness.
    pub strikeout_thickness: Scalar,

    /// Distance from baseline to bottom of stroke, typically negative.
    pub strikeout_position: Scalar,
}

bitflags! {
    /// `FontMetricsFlags` indicate when certain metrics are valid;
    /// the underline or strikeout metrics may be valid and zero.
    ///
    /// Fonts with embedded bitmaps may not have valid underline or strikeout metrics.
    #[derive(Debug, Clone, Copy, Eq, PartialEq)]
    pub struct FontMetricsFlags : u32 {
        /// Set if underline_thickness is valid.
        const UnderlineThicknessIsValid = 1 << 0;

        /// Set if underline_position is valid.
        const UnderlinePositionIsValid = 1 << 1;

        /// Set if strikeout_thickness is valid.
        const StrikeoutThicknessIsValid = 1 << 2;

        /// Set if strikeout_position is valid.
        const StrikeoutPositionIsValid = 1 << 3;

        /// Set if top, bottom, xmin, xmax invalid.
        const BoundsInvalid = 1 << 4;
    }
}

impl FontMetrics {
    /// Returns `Some(thickness)` if `FontMetrics` has a valid underline thickness.
    ///
    /// If the underline thickness is not valid, return None.
    #[must_use]
    #[inline]
    pub const fn has_underline_thickness(&self) -> Option<Scalar> {
        if self
            .flags
            .contains(FontMetricsFlags::UnderlineThicknessIsValid)
        {
            Some(self.underline_thickness)
        } else {
            None
        }
    }

    /// Returns `Some(position)` value if `FontMetrics` has a valid underline position.
    ///
    /// If the underline position is not valid, return None.
    #[must_use]
    #[inline]
    pub const fn has_underline_position(&self) -> Option<Scalar> {
        if self
            .flags
            .contains(FontMetricsFlags::UnderlinePositionIsValid)
        {
            Some(self.underline_position)
        } else {
            None
        }
    }

    /// Returns `Some(thickness)` if `FontMetrics` has a valid strikeout thickness.
    ///
    /// If the underline thickness is not valid, return None.
    #[must_use]
    #[inline]
    pub const fn has_strikeout_thickness(&self) -> Option<Scalar> {
        if self
            .flags
            .contains(FontMetricsFlags::StrikeoutThicknessIsValid)
        {
            Some(self.strikeout_thickness)
        } else {
            None
        }
    }

    /// Returns `Some(position)` if `FontMetrics` has a valid strikeout position.
    ///
    /// If the underline position is not valid, return None.
    #[must_use]
    #[inline]
    pub const fn has_strikeout_position(&self) -> Option<Scalar> {
        if self
            .flags
            .contains(FontMetricsFlags::StrikeoutPositionIsValid)
        {
            Some(self.strikeout_position)
        } else {
            None
        }
    }

    /// Returns true if `FontMetrics` has a valid top, bottom, `x_min`, and `x_max`.
    ///
    /// If the bounds are not valid, return false.
    ///
    /// Returns true if font specifies maximum glyph bounds.
    #[must_use]
    #[inline]
    pub const fn has_bounds(&self) -> bool {
        !self.flags.contains(FontMetricsFlags::BoundsInvalid)
    }
}
