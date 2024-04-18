// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that can be found
// in the LICENSE file.

#[repr(u16)]
#[derive(Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum FontWeight {
    Invisible = 0,
    Thin = 100,
    ExtraLight = 200,
    Light = 300,
    Normal = 400,
    Medium = 500,
    SemiBold = 600,
    Bold = 700,
    ExtraBold = 800,
    Black = 900,
    ExtraBlack = 1000,
}

impl Default for FontWeight {
    fn default() -> Self {
        Self::Normal
    }
}

impl FontWeight {
    #[must_use]
    pub const fn is_bold(self) -> bool {
        self as u16 >= Self::SemiBold as u16
    }
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum FontWidth {
    UltraCondensed = 1,
    ExtraCondensed = 2,
    Condensed = 3,
    SemiCondensed = 4,
    Normal = 5,
    SemiExpanded = 6,
    Expanded = 7,
    ExtraExpanded = 8,
    UltraExpanded = 9,
}

impl Default for FontWidth {
    fn default() -> Self {
        Self::Normal
    }
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum FontSlant {
    Upright = 0,
    Italic,
    Oblique,
}

impl Default for FontSlant {
    fn default() -> Self {
        Self::Upright
    }
}

impl FontSlant {
    #[must_use]
    pub fn is_italic(self) -> bool {
        self != Self::Upright
    }
}

#[derive(Debug, Default, Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct FontStyle {
    weight: FontWeight,
    width: FontWidth,
    slant: FontSlant,
}

impl From<FontStyle> for u32 {
    fn from(style: FontStyle) -> Self {
        Self::from(style.weight as u16)
            + (Self::from(style.width as u8) << 16)
            + (Self::from(style.slant as u8) << 24)
    }
}

// TODO(Shaohua): Convert from u32
//impl TryFrom<u32> for FontStyle {
//    fn Tryfrom(value: u32) -> Option<Self> {
//        let weight = (value & 0xFFFF) as u16;
//        let width = ((value >> 16) & 0xFF) as u8;
//        let slant = ((value >> 24) & 0xFF) as u8;
//        Some(Self {
//            weight,
//            width,
//            slant,
//        })
//    }
//}

impl FontStyle {
    #[must_use]
    pub const fn weight(self) -> FontWeight {
        self.weight
    }

    #[must_use]
    pub const fn width(self) -> FontWidth {
        self.width
    }

    #[must_use]
    pub const fn slant(self) -> FontSlant {
        self.slant
    }

    #[must_use]
    pub const fn normal() -> Self {
        Self {
            weight: FontWeight::Normal,
            width: FontWidth::Normal,
            slant: FontSlant::Upright,
        }
    }

    #[must_use]
    pub const fn bold() -> Self {
        Self {
            weight: FontWeight::Bold,
            width: FontWidth::Normal,
            slant: FontSlant::Upright,
        }
    }

    #[must_use]
    pub const fn italic() -> Self {
        Self {
            weight: FontWeight::Normal,
            width: FontWidth::Normal,
            slant: FontSlant::Italic,
        }
    }

    #[must_use]
    pub const fn bold_italic() -> Self {
        Self {
            weight: FontWeight::Bold,
            width: FontWidth::Normal,
            slant: FontSlant::Italic,
        }
    }
}
