// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

#[derive(Debug, Clone)]
pub struct Font {
    font_size: i32,
    font_family: Option<String>,
    font_stretch: f64,
}

impl Font {
    pub fn font_family(&self) -> Option<&String> {
        self.font_family.as_ref()
    }

    pub fn set_font_family(&mut self, family: Option<String>) {
        self.font_family = family;
    }

    pub fn set_pixel_size(&mut self, pixel_size: i32) {
        self.font_size = pixel_size;
    }

    pub fn font_stretch(&self) -> f64 {
        self.font_stretch
    }

    pub fn set_font_stretch_f64(&mut self, stretch: f64) {
        assert!(stretch > 0.0);
        self.font_stretch = stretch;
    }

    pub fn set_font_stretch(&mut self, stretch: FontStretch) {
        let value = match stretch {
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
        self.set_font_stretch_f64(value);
    }
}

/// Predefined font stretch keywords.
#[derive(Debug, Clone, PartialEq)]
pub enum FontStretch {
    /// 50%
    UltraCondensed,

    /// 62.5%
    ExtraCondensed,

    /// 75%
    Condensed,

    /// 87.5%
    SemiCondensed,

    /// 100%
    Normal,

    /// 112.5%
    SemiExpanded,

    /// 125%
    Expanded,

    /// 150%
    ExtraExpanded,

    /// 200%
    UltraExpanded,
}
