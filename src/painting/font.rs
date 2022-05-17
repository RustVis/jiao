// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

#[derive(Debug, Clone)]
pub struct Font {
    font_size: i32,
    font_family: String,
    stretch: f64,
    weight: u16,
    capital: Capitalization,
    style: FontStyle,
    line: LineStyle,
}

const WEIGHT_MAX: u16 = 100;

impl Font {
    /// Returns true if `weight()` is a value greater than `FontWeight::Medium`;
    /// otherwise returns false.
    #[must_use]
    pub fn is_bold(&self) -> bool {
        self.weight >= u16::from(FontWeight::Medium)
    }

    /// Returns the current capitalization type of the font.
    #[must_use]
    pub const fn capitalization(&self) -> Capitalization {
        self.capital
    }

    /// Returns the requested font family name.
    #[must_use]
    pub fn family(&self) -> &str {
        &self.font_family
    }

    /// Returns true if the `style()` of the font is not FontStyle::Normal
    #[must_use]
    pub fn is_italic(&self) -> bool {
        self.style != FontStyle::Normal
    }

    /// Returns true if overline has been set; otherwise returns false.
    #[must_use]
    pub fn is_overline(&self) -> bool {
        self.line == LineStyle::OverLine
    }

    /// Returns true if underline has been set; otherwise returns false.
    #[must_use]
    pub fn is_underline(&self) -> bool {
        self.line == LineStyle::UnderLine
    }

    /// If `enable` is true sets the font's weight to `FontWeight::Bold`;
    /// otherwise sets the weight to `FontWeight::Normal`.
    pub fn set_bold(&mut self, enable: bool) {
        self.weight = if enable {
            FontWeight::Bold.into()
        } else {
            FontWeight::Normal.into()
        };
    }

    /// Sets the capitalization of the text in this font to caps.
    ///
    /// A font's capitalization makes the text appear in the selected capitalization mode.
    pub fn set_capitalization(&mut self, _caps: Capitalization) {
        todo!()
    }

    /// Sets the list of family names for the font.
    ///
    /// The names are case insensitive and may include a foundry name.
    /// The first family in families will be set as the main family for the font.
    ///
    /// Each family name entry in families may optionally also include a foundry name,
    /// e.g. "Helvetica [Cronyx]".
    /// If the family is available from more than one foundry and the foundry isn't specified,
    /// an arbitrary foundry is chosen. If the family isn't available a family will be set
    /// using the font matching algorithm.
    pub fn set_families(&mut self, _families: &[&str]) {
        todo!()
    }

    /// Sets the family name of the font.
    ///
    /// The name is case insensitive and may include a foundry name.
    /// The family name may optionally also include a foundry name, e.g. "Helvetica [Cronyx]".
    /// If the family is available from more than one foundry and the foundry isn't specified,
    /// an arbitrary foundry is chosen.
    /// If the family isn't available a family will be set using the font matching algorithm.
    pub fn set_family(&mut self, family: String) {
        self.font_family = family;
    }

    /// Sets the `style()` of the font to `FontStyle::Italic` if enable is true;
    /// otherwise the style is set to `FontStyle::Normal`.
    pub fn set_italic(&mut self, enable: bool) {
        self.style = if enable {
            FontStyle::Italic
        } else {
            FontStyle::Normal
        };
    }

    /// If enable is true, sets overline on; otherwise sets overline off.
    pub fn set_overline(&mut self, enable: bool) {
        self.line = if enable {
            LineStyle::OverLine
        } else {
            LineStyle::Normal
        };
    }

    pub fn set_pixel_size(&mut self, pixel_size: i32) {
        self.font_size = pixel_size;
    }

    /// Sets the stretch factor for the font.
    ///
    /// The stretch factor matches a condensed or expanded version of the font or
    /// applies a stretch transform that changes the width of all characters in the font
    /// by factor percent.
    ///
    /// For example, setting factor to 150 results in all characters in the font
    /// being 1.5 times (ie. 150%) wider.
    /// The minimum stretch factor is 1, and the maximum stretch factor is 4000.
    /// The default stretch factor is AnyStretch, which will accept any stretch factor
    /// and not apply any transform on the font.
    ///
    /// The stretch factor is only applied to outline fonts.
    /// The stretch factor is ignored for bitmap fonts.
    //pub fn set_stretch(&mut self, factor: i32) {
    pub fn set_stretch_f64(&mut self, stretch: f64) {
        assert!(stretch > 0.0);
        self.stretch = stretch;
    }

    pub fn set_stretch(&mut self, stretch: FontStretch) {
        let value = match stretch {
            FontStretch::AnyStretch => 0.0,
            FontStretch::UltraCondensed => 0.5,
            FontStretch::ExtraCondensed => 0.625,
            FontStretch::Condensed => 0.75,
            FontStretch::SemiCondensed => 0.875,
            FontStretch::Normal => 1.0,
            FontStretch::SemiExpanded => 1.125,
            FontStretch::Expanded => 1.25,
            FontStretch::ExtraExpanded => 1.5,
            FontStretch::UltraExpanded => 2.0,
        };
        self.set_stretch_f64(value);
    }

    /// Sets the style of the font to `style`.
    pub fn set_style(&mut self, style: FontStyle) {
        self.style = style;
    }

    /// If enable is true, sets underline on; otherwise sets underline off.
    pub fn set_underline(&mut self, enable: bool) {
        self.line = if enable {
            LineStyle::UnderLine
        } else {
            LineStyle::Normal
        };
    }

    /// Sets the weight of the font to `weight`.
    pub fn set_weight(&mut self, weight: u16) {
        assert!(weight <= WEIGHT_MAX);
        self.weight = weight;
    }

    /// Returns the stretch factor for the font.
    #[must_use]
    pub fn stretch(&self) -> f64 {
        self.stretch
    }

    /// Returns the style of the font.
    #[must_use]
    pub fn style(&self) -> FontStyle {
        self.style
    }

    /// Returns the weight of the font.
    #[must_use]
    pub const fn weight(&self) -> u16 {
        self.weight
    }
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum FontPatternError {
    InvalidFormat,
}

impl TryFrom<&str> for Font {
    type Error = FontPatternError;

    /// Sets this font to match the description descrip.
    ///
    /// The description is a space-separated list of the font attributes, as returned by `to_string()`.
    fn try_from(_s: &str) -> Result<Self, Self::Error> {
        todo!()
    }
}

impl ToString for Font {
    /// Returns a description of the font.
    ///
    /// The description is a space-separated list of the attributes.
    fn to_string(&self) -> String {
        todo!()
    }
}

/// Predefined font stretch keywords.
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum FontStretch {
    AnyStretch = 0,
    /// 50%
    UltraCondensed = 50,
    /// 62.5%
    ExtraCondensed = 62,
    /// 75%
    Condensed = 75,
    /// 87.5%
    SemiCondensed = 87,
    /// 100%
    Normal = 100,
    /// 112.5%
    SemiExpanded = 112,
    /// 125%
    Expanded = 125,
    /// 150%
    ExtraExpanded = 150,
    /// 200%
    UltraExpanded = 200,
}

#[repr(u16)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum FontWeight {
    Thin = 100,
    ExtraLight = 200,
    Light = 300,
    Normal = 400,
    Medium = 500,
    DemiBold = 600,
    Bold = 700,
    ExtraBold = 800,
    Black = 900,
}

impl From<FontWeight> for u16 {
    fn from(weight: FontWeight) -> Self {
        match weight {
            FontWeight::Thin => 100,
            FontWeight::ExtraLight => 200,
            FontWeight::Light => 300,
            FontWeight::Normal => 400,
            FontWeight::Medium => 500,
            FontWeight::DemiBold => 600,
            FontWeight::Bold => 700,
            FontWeight::ExtraBold => 800,
            FontWeight::Black => 900,
        }
    }
}

/// Rendering option for text this font applies to.
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Capitalization {
    /// This is the normal text rendering option where no capitalization change is applied.
    MixedCase = 0,
    /// This alters the text to be rendered in all uppercase type.
    AllUppercase = 1,
    /// This alters the text to be rendered in all lowercase type.
    AllLowercase = 2,
    /// This alters the text to be rendered in small-caps type.
    SmallCaps = 3,
    /// This alters the text to be rendered with the first character of each word as an uppercase character.
    Capitalize = 4,
}

/// This enum describes the different styles of glyphs that are used to display text.
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum FontStyle {
    /// Normal glyphs used in unstyled text.
    Normal = 0,
    /// Italic glyphs that are specifically designed for the purpose of representing italicized text.
    Italic = 1,
    /// Glyphs with an italic appearance that are typically based on the unstyled glyphs,
    /// but are not fine-tuned for the purpose of representing italicized text.
    Oblique = 2,
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq)]
enum LineStyle {
    Normal = 0,
    UnderLine = 1,
    OverLine = 2,
}
