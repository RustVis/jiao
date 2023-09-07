// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by LGPL-3.0 License that can be found
// in the LICENSE file.

use core::ops;

use crate::util::div_257;

/// The Rgba64 struct contains a 64-bit RGB color.
///
/// Rgba64 is a 64-bit data-structure containing four 16-bit color channels: Red, green, blue and alpha.
///
/// Rgba64 can be used as a replacement for `Rgb` when higher precision is needed.
/// In particular a premultiplied Rgba64 can operate on unpremultiplied Rgb
/// without loss of precision except for alpha 0
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct Rgba64 {
    rgba: u64,
}

/// Make sure that the representation always has the order: red green blue alpha, independent
/// of byte order.
///
/// This way, vector operations that assume 4 16-bit values see the correct ones.
#[cfg(target_endian = "big")]
#[repr(u8)]
#[derive(Debug)]
enum Shifts {
    Red = 48,
    Green = 32,
    Blue = 16,
    Alpha = 0,
}
#[cfg(target_endian = "little")]
#[repr(u8)]
#[derive(Debug)]
enum Shifts {
    Red = 0,
    Green = 16,
    Blue = 32,
    Alpha = 48,
}

impl ops::Shl<Shifts> for u64 {
    type Output = Self;

    fn shl(self, shifts: Shifts) -> Self::Output {
        self << (shifts as u8)
    }
}

impl ops::Shr<Shifts> for u64 {
    type Output = Self;

    fn shr(self, shifts: Shifts) -> Self::Output {
        self >> (shifts as u8)
    }
}

impl Rgba64 {
    /// Returns the Rgba64 with rgba = 0.
    #[must_use]
    pub const fn new() -> Self {
        Self { rgba: 0 }
    }

    /// Constructs a Rgba64 value from the 32bit ARGB value rgb.
    #[must_use]
    pub fn from_argb32(rgb: u32) -> Self {
        #[allow(clippy::cast_possible_truncation)]
        Self::from_rgba(
            (rgb >> 16) as u8,
            (rgb >> 8) as u8,
            rgb as u8,
            (rgb >> 24) as u8,
        )
    }

    /// Returns rgba as a Rgba64 struct.
    #[must_use]
    pub const fn from_u64(rgba: u64) -> Self {
        Self { rgba }
    }

    /// Returns the Rgba64 quadruplet (red, green, blue, alpha).
    #[must_use]
    pub fn from_rgba64(red: u16, green: u16, blue: u16, alpha: u16) -> Self {
        Self::from_u64(
            u64::from(red) << Shifts::Red
                | u64::from(green) << Shifts::Green
                | u64::from(blue) << Shifts::Blue
                | u64::from(alpha) << Shifts::Alpha,
        )
    }

    /// Constructs a Rgba64 value from the four 8-bit color channels red, green, blue and alpha.
    #[must_use]
    pub fn from_rgba(red: u8, green: u8, blue: u8, alpha: u8) -> Self {
        let mut rgba64 = Self::from_rgba64(
            u16::from(red),
            u16::from(green),
            u16::from(blue),
            u16::from(alpha),
        );
        // Expand the range so that 0x00 maps to 0x0000 and 0xff maps to 0xffff.
        rgba64.rgba |= rgba64.rgba << 8;
        rgba64
    }

    /// Constructs a Rgba64 value from the four 8-bit color channels red, green and blue.
    #[must_use]
    pub fn from_rgb(red: u8, green: u8, blue: u8) -> Self {
        Self::from_rgba(red, green, blue, u8::MAX)
    }

    /// Returns the alpha channel as an 8-bit.
    #[must_use]
    pub fn alpha8(&self) -> u8 {
        div_257(self.alpha())
    }

    /// Returns the alpha channel as an 16-bit.
    #[must_use]
    #[allow(clippy::cast_possible_truncation)]
    pub fn alpha(&self) -> u16 {
        (self.rgba >> Shifts::Alpha) as u16
    }

    /// Returns the blue color component as an 8-bit.
    #[must_use]
    pub fn blue8(&self) -> u8 {
        div_257(self.blue())
    }

    /// Returns the 16-bit blue color component.
    #[must_use]
    #[allow(clippy::cast_possible_truncation)]
    pub fn blue(&self) -> u16 {
        (self.rgba >> Shifts::Blue) as u16
    }

    /// Returns the green color component as an 8-bit.
    #[must_use]
    pub fn green8(&self) -> u8 {
        div_257(self.green())
    }

    /// Returns the 16-bit green color component.
    #[must_use]
    #[allow(clippy::cast_possible_truncation)]
    pub fn green(&self) -> u16 {
        (self.rgba >> Shifts::Green) as u16
    }

    /// Returns whether the color is fully opaque.
    #[must_use]
    pub fn is_opaque(&self) -> bool {
        (self.rgba & Self::alpha_mask()) == Self::alpha_mask()
    }

    /// Returns whether the color is transparent.
    #[must_use]
    pub fn is_transparent(&self) -> bool {
        (self.rgba & Self::alpha_mask()) == 0
    }

    /// Returns the color with the alpha premultiplied.
    #[must_use]
    pub fn premultiplied(&self) -> Self {
        if self.is_opaque() {
            return *self;
        }
        if self.is_transparent() {
            return Self::from_u64(0);
        }

        let a = u64::from(self.alpha());
        let mut br = (self.rgba & 0xffff_0000_ffff_u64) * a;
        let mut ag = ((self.rgba >> 16) & 0xffff_0000_ffff_u64) * a;
        br = br + ((br >> 16) & 0xffff_0000_ffff_u64) + 0x8000_0000_8000_u64;
        ag = ag + ((ag >> 16) & 0xffff_0000_ffff_u64) + 0x8000_0000_8000_u64;

        #[cfg(target_endian = "big")]
        {
            ag = ag & 0xffff0000ffff0000_u64;
            br = (br >> 16) & 0xffff00000000_u64;
            return Self::from_u64(a | br | ag);
        }

        #[cfg(target_endian = "little")]
        {
            br = (br >> 16) & 0xffff_0000_ffff_u64;
            ag &= 0xffff_0000_u64;
            Self::from_u64((a << 48) | br | ag)
        }
    }

    /// Returns the red color component as an 8-bit.
    #[must_use]
    pub fn red8(&self) -> u8 {
        div_257(self.red())
    }

    /// Returns the 16-bit red color component.
    #[must_use]
    #[allow(clippy::cast_possible_truncation)]
    pub fn red(&self) -> u16 {
        (self.rgba >> Shifts::Red) as u16
    }

    /// Sets the alpha of this color to alpha.
    pub fn set_alpha(&mut self, alpha: u16) {
        self.rgba =
            (self.rgba & !(0xffff_u64 << Shifts::Alpha)) | (u64::from(alpha) << Shifts::Alpha);
    }

    /// Sets the blue color component of this color to blue.
    pub fn set_blue(&mut self, blue: u16) {
        self.rgba = (self.rgba & !(0xffff_u64 << Shifts::Blue)) | (u64::from(blue) << Shifts::Blue);
    }

    /// Sets the green color component of this color to green.
    pub fn set_green(&mut self, green: u16) {
        self.rgba =
            (self.rgba & !(0xffff_u64 << Shifts::Green)) | (u64::from(green) << Shifts::Green);
    }

    /// Sets the red color component of this color to red.
    pub fn set_red(&mut self, red: u16) {
        self.rgba = (self.rgba & !(0xffff_u64 << Shifts::Red)) | (u64::from(red) << Shifts::Red);
    }

    /// Sets the internal rgba value.
    pub fn set_u64(&mut self, rgba: u64) {
        self.rgba = rgba;
    }

    /// Returns the color as a 32-bit ARGB value.
    #[must_use]
    pub fn to_argb32(&self) -> u32 {
        (u32::from(self.alpha8()) << 24)
            | (u32::from(self.red8()) << 16)
            | (u32::from(self.green8()) << 8)
            | u32::from(self.blue8())
    }

    /// Returns the color as a 16-bit RGB value.
    #[must_use]
    pub fn to_rgb16(&self) -> u16 {
        (self.red() & 0xf800_u16) | ((self.green() >> 10) << 5) | (self.blue() >> 11)
    }

    /// Returns the color as a 64bit unsigned integer
    #[must_use]
    pub const fn to_u64(&self) -> u64 {
        self.rgba
    }

    /// Returns the color with the alpha unpremultiplied.
    #[must_use]
    pub fn unpremultiplied(&self) -> Self {
        #[cfg(target_pointer_width = "32")]
        return self.unpremultiplied_32bit();

        #[cfg(target_pointer_width = "64")]
        return self.unpremultiplied_64bit();
    }

    #[allow(dead_code)]
    #[allow(clippy::cast_possible_truncation)]
    fn unpremultiplied_32bit(self) -> Self {
        if self.is_opaque() || self.is_transparent() {
            return self;
        }
        let a = u32::from(self.alpha());
        let r = ((u32::from(self.red()) * 0xffff_u32 + a / 2) / a) as u16;
        let g = ((u32::from(self.green()) * 0xffff_u32 + a / 2) / a) as u16;
        let b = ((u32::from(self.blue()) * 0xffff_u32 + a / 2) / a) as u16;
        Self::from_rgba64(r, g, b, a as u16)
    }

    #[allow(dead_code)]
    #[allow(clippy::cast_possible_truncation)]
    fn unpremultiplied_64bit(self) -> Self {
        if self.is_opaque() || self.is_transparent() {
            return self;
        }
        let a = u64::from(self.alpha());
        let fa = (0xffff_0000_8000_u64 + a / 2) / a;
        let r = ((u64::from(self.red()) * fa + 0x8000_0000_u64) >> 32) as u16;
        let g = ((u64::from(self.green()) * fa + 0x8000_0000_u64) >> 32) as u16;
        let b = ((u64::from(self.blue()) * fa + 0x8000_0000_u64) >> 32) as u16;
        Self::from_rgba64(r, g, b, a as u16)
    }

    #[inline]
    fn alpha_mask() -> u64 {
        0xffff_u64 << Shifts::Alpha
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_color() {
        let rgba = Rgba64::from_rgba(255, 238, 170, 153);
        assert_eq!(rgba.red8(), 255);
        assert_eq!(rgba.green8(), 238);
        assert_eq!(rgba.blue8(), 170);
        assert_eq!(rgba.alpha8(), 153);
    }
}
