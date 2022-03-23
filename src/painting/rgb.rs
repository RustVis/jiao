// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

/// An ARGB quadruplet on the format #AARRGGBB, equivalent to an unsigned int.
///
/// The type also holds a value for the alpha-channel.
/// The default alpha channel is ff, i.e opaque.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Rgb {
    rgb: u32,
}

/// Masks RGB values.
pub const RGB_MASK: Rgb = Rgb { rgb: 0x00ffffff };

impl Default for Rgb {
    fn default() -> Self {
        Rgb::new(0, 0, 0)
    }
}

impl Rgb {
    // Set RGB value.
    pub fn new(red: u8, green: u8, blue: u8) -> Self {
        Self {
            rgb: (0xff << 24) | ((red as u32) << 16) | ((green as u32) << 8) | (blue as u32),
        }
    }

    /// Set RGBA value.
    pub fn with_alpha(red: u8, green: u8, blue: u8, alpha: u8) -> Self {
        Self {
            rgb: ((alpha as u32) << 24)
                | ((red as u32) << 16)
                | ((green as u32) << 8)
                | (blue as u32),
        }
    }

    /// Get red part of RGB.
    pub fn red(&self) -> u8 {
        ((self.rgb >> 16) & 0xff) as u8
    }

    /// Get green part of RGB.
    pub fn green(&self) -> u8 {
        ((self.rgb >> 8) & 0xff) as u8
    }

    /// Get blue part of RGB.
    pub fn blue(&self) -> u8 {
        (self.rgb & 0xff) as u8
    }

    /// Get alpha part of RGB.
    pub fn alpha(&self) -> u8 {
        (self.rgb >> 24) as u8
    }

    /// Convert R,G,B to gray 0..255
    pub fn int_to_gray(red: u8, green: u8, blue: u8) -> u8 {
        return (red * 11 + green * 16 + blue * 5) / 32;
    }

    /// Convert RGB to gray 0..255
    pub fn to_gray(&self) -> u8 {
        Self::int_to_gray(self.red(), self.green(), self.blue())
    }

    pub fn is_gray(&self) -> bool {
        self.red() == self.green() && self.red() == self.blue()
    }

    pub fn premultiply(&self) -> Self {
        let alpha = self.alpha() as u32;
        let mut t = (self.rgb & 0xff00ff) * alpha;
        t = (t + ((t >> 8) & 0xff00ff) + 0x800080) >> 8;
        t &= 0xff00ff;

        let mut x = ((self.rgb >> 8) & 0xff) * alpha;
        x = x + ((x >> 8) & 0xff) + 0x80;
        x &= 0xff00;
        Self {
            rgb: x | t | (alpha << 24),
        }
    }

    pub fn int(&self) -> u32 {
        self.rgb
    }
}