// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

use std::convert::From;

/// An ARGB quadruplet on the format #AARRGGBB, equivalent to an unsigned int.
///
/// The type also holds a value for the alpha-channel.
/// The default alpha channel is ff, i.e opaque.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Rgb {
    rgb: u32,
}

/// Masks RGB values.
pub const RGB_MASK: Rgb = Rgb { rgb: 0x00ff_ffff };

impl Default for Rgb {
    fn default() -> Self {
        Self::new(0, 0, 0)
    }
}

impl From<u32> for Rgb {
    fn from(rgb: u32) -> Self {
        Self { rgb }
    }
}

impl Rgb {
    // Set RGB value.
    #[must_use]
    pub fn new(red: u8, green: u8, blue: u8) -> Self {
        Self {
            rgb: (0xff << 24) | (u32::from(red) << 16) | (u32::from(green) << 8) | u32::from(blue),
        }
    }

    /// Set RGBA value.
    #[must_use]
    pub fn with_alpha(red: u8, green: u8, blue: u8, alpha: u8) -> Self {
        Self {
            rgb: (u32::from(alpha) << 24)
                | (u32::from(red) << 16)
                | (u32::from(green) << 8)
                | u32::from(blue),
        }
    }

    /// Get red part of RGB.
    #[must_use]
    pub const fn red(&self) -> u8 {
        ((self.rgb >> 16) & 0xff) as u8
    }

    /// Get green part of RGB.
    #[must_use]
    pub const fn green(&self) -> u8 {
        ((self.rgb >> 8) & 0xff) as u8
    }

    /// Get blue part of RGB.
    #[must_use]
    pub const fn blue(&self) -> u8 {
        (self.rgb & 0xff) as u8
    }

    /// Get alpha part of RGB.
    #[must_use]
    pub const fn alpha(&self) -> u8 {
        (self.rgb >> 24) as u8
    }

    /// Convert R,G,B to gray 0..255
    #[must_use]
    pub const fn int_to_gray(red: u8, green: u8, blue: u8) -> u8 {
        (red * 11 + green * 16 + blue * 5) / 32
    }

    /// Convert RGB to gray 0..255
    #[must_use]
    pub const fn to_gray(&self) -> u8 {
        Self::int_to_gray(self.red(), self.green(), self.blue())
    }

    #[must_use]
    pub const fn is_gray(&self) -> bool {
        self.red() == self.green() && self.red() == self.blue()
    }

    #[must_use]
    pub fn premultiply(&self) -> Self {
        let alpha = u32::from(self.alpha());
        let mut t = (self.rgb & 0x00ff_00ff) * alpha;
        t = (t + ((t >> 8) & 0x00ff_00ff) + 0x0080_0080) >> 8;
        t &= 0x00ff_00ff;

        let mut x = ((self.rgb >> 8) & 0xff) * alpha;
        x = x + ((x >> 8) & 0xff) + 0x80;
        x &= 0xff00;
        Self {
            rgb: x | t | (alpha << 24),
        }
    }

    #[must_use]
    pub const fn int(&self) -> u32 {
        self.rgb
    }
}
