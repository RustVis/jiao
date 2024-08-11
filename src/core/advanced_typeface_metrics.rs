// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that can be found
// in the LICENSE file.

use bitflags::bitflags;

use crate::core::irect::IRect;

/// The `AdvancedTypefaceMetrics` class is used by the PDF backend to correctly
/// embed typefaces.
///
/// This class is created and filled in with information by `Typeface::get_advanced_metrics()`.
#[derive(Debug, Default, Clone)]
pub struct AdvancedTypefaceMetrics {
    /// The `PostScript` name of the font.
    ///
    /// See `FontName` and `BaseFont` in PDF standard.
    post_script_name: String,
    font_name: String,

    /// Font style characteristics.
    style: StyleFlags,

    /// The type of the underlying font program.
    ///
    /// This field determines which of the following fields are valid.
    /// If it is Other the per glyph information will never be populated.
    font_type: FontType,

    /// Global font flags.
    flags: FontFlags,

    /// Counterclockwise degrees from vertical of the dominant vertical stroke for an Italic face.
    italic_angle: i16,

    // The following fields are all in font units.
    /// Max height above baseline, not including accents.
    ascent: i16,

    /// Max depth below baseline (negative).
    descent: i16,

    /// Thickness of dominant vertical stem.
    stem_v: i16,

    /// Height (from baseline) of top of flat capitals.
    cap_height: i16,

    /// The bounding box of all glyphs (in font units).
    bbox: IRect,
}

bitflags! {
    /// These enum values match the values used in the PDF file format.
    #[derive(Debug, Clone, Copy, Eq, PartialEq)]
    pub struct StyleFlags: u32 {
        const FixedPitch = 0x0000_0001;
        const Serif = 0x0000_0002;
        const Script = 0x0000_0008;
        const Italic = 0x0000_0040;
        const AllCaps = 0x0001_0000;
        const SmallCaps = 0x0002_0000;
        const ForceBold = 0x0004_0000;
    }
}

impl Default for StyleFlags {
    #[inline]
    fn default() -> Self {
        Self::empty()
    }
}

#[repr(u8)]
#[derive(Debug, Default, Clone, Copy, Eq, PartialEq, Hash)]
pub enum FontType {
    Type1,
    Type1Cid,
    Cff,
    TrueType,
    #[default]
    Other,
}

bitflags! {
    #[derive(Debug, Clone, Copy, Eq, PartialEq)]
    pub struct FontFlags : u8 {
        /// May be true for Type1, Cff, or TrueType fonts.
        const Variable = 1 << 0;

        /// May not be embedded.
        const NotEmbeddable = 1 << 1;

        /// May not be subset.
        const NotSubsettable = 1 << 2;

        /// Data compressed. Table access may still work.
        const AltDataFormat = 1 << 3;
    }
}

impl Default for FontFlags {
    #[inline]
    fn default() -> Self {
        Self::empty()
    }
}
