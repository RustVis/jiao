// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

use core::ops;

/// The Rgba64 struct contains a 64-bit RGB color.
///
/// Rgba64 is a 64-bit data-structure containing four 16-bit color channels: Red, green, blue and alpha.
///
/// Rgba64 can be used as a replacement for `Rgb` when higher precision is needed.
/// In particular a premultiplied Rgba64 can operate on unpremultiplied Rgb
/// without loss of precision except for alpha 0
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Rgba64 {
    rgba: u64,
}

/// Make sure that the representation always has the order: red green blue alpha, independent
/// of byte order.
///
/// This way, vector operations that assume 4 16-bit values see the correct ones.
#[cfg(target_endian = "big")]
#[repr(u8)]
enum Shifts {
    RedShift = 48,
    GreenShift = 32,
    BlueShift = 16,
    AlphaShift = 0,
}
#[cfg(target_endian = "little")]
#[repr(u8)]
enum Shifts {
    RedShift = 0,
    GreenShift = 16,
    BlueShift = 32,
    AlphaShift = 48,
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
    pub fn new() -> Self {
        Self { rgba: 0 }
    }

    /// Constructs a Rgba64 value from the 32bit ARGB value rgb.
    pub fn from_argb32(rgb: u32) -> Self {
        Self::from_rgba(
            (rgb >> 16) as u8,
            (rgb >> 8) as u8,
            rgb as u8,
            (rgb >> 24) as u8,
        )
    }

    /// Returns rgba as a Rgba64 struct.
    pub fn from_u64(rgba: u64) -> Self {
        Self { rgba }
    }

    /// Returns the Rgba64 quadruplet (red, green, blue, alpha).
    pub fn from_rgba64(red: u16, green: u16, blue: u16, alpha: u16) -> Self {
        Self::from_u64(
            (red as u64) << Shifts::RedShift
                | (green as u64) << Shifts::GreenShift
                | (blue as u64) << Shifts::BlueShift
                | (alpha as u64) << Shifts::AlphaShift,
        )
    }

    /// Constructs a Rgba64 value from the four 8-bit color channels red, green, blue and alpha.
    pub fn from_rgba(red: u8, green: u8, blue: u8, alpha: u8) -> Self {
        let mut rgb64 = Self::from_rgba64(red as u16, green as u16, blue as u16, alpha as u16);
        // Expand the range so that 0x00 maps to 0x0000 and 0xff maps to 0xffff.
        rgb64.rgba |= rgb64.rgba << 8;
        return rgb64;
    }

    /// Returns the alpha channel as an 8-bit.
    pub fn alpha8(&self) -> u8 {
        Self::div_257(self.alpha())
    }

    /// Returns the alpha channel as an 8-bit.
    pub fn alpha(&self) -> u16 {
        (self.rgba >> Shifts::RedShift) as u16
    }

    /// Returns the blue color component as an 8-bit.
    pub fn blue8(&self) -> u8 {
        Self::div_257(self.blue())
    }

    /// Returns the 16-bit blue color component.
    pub fn blue(&self) -> u16 {
        (self.rgba >> Shifts::BlueShift) as u16
    }

    /// Returns the green color component as an 8-bit.
    pub fn green8(&self) -> u8 {
        Self::div_257(self.green())
    }

    /// Returns the 16-bit green color component.
    pub fn green(&self) -> u16 {
        (self.rgba >> Shifts::GreenShift) as u16
    }

    /// Returns whether the color is fully opaque.
    pub fn is_opaque(&self) -> bool {
        (self.rgba & Self::alpha_mask()) == Self::alpha_mask()
    }

    /// Returns whether the color is transparent.
    pub fn is_transparent(&self) -> bool {
        (self.rgba & Self::alpha_mask()) == 0
    }

    /// Returns the color with the alpha premultiplied.
    pub fn premultiplied(&self) -> Self {
        if self.is_opaque() {
            return *self;
        }
        if self.is_transparent() {
            return Self::from_u64(0);
        }

        let a = self.alpha() as u64;
        let mut br = (self.rgba & 0xffff0000ffff_u64) * a;
        let mut ag = ((self.rgba >> 16) & 0xffff0000ffff_u64) * a;
        br = br + ((br >> 16) & 0xffff0000ffff_u64) + 0x800000008000_u64;
        ag = ag + ((ag >> 16) & 0xffff0000ffff_u64) + 0x800000008000_u64;

        #[cfg(target_endian = "big")]
        {
            ag = ag & 0xffff0000ffff0000_u64;
            br = (br >> 16) & 0xffff00000000_u64;
            return Self::from_u64(a | br | ag);
        }

        #[cfg(target_endian = "little")]
        {
            br = (br >> 16) & 0xffff0000ffff_u64;
            ag = ag & 0xffff0000_u64;
            return Self::from_u64((a << 48) | br | ag);
        }
    }

    /// Returns the red color component as an 8-bit.
    pub fn red8(&self) -> u8 {
        Self::div_257(self.red())
    }

    /// Returns the 16-bit red color component.
    pub fn red(&self) -> u16 {
        (self.rgba >> Shifts::RedShift) as u16
    }

    /// Sets the alpha of this color to alpha.
    pub fn set_alpha(&mut self, alpha: u16) {
        self.rgba = (self.rgba & !(0xffff_u64 << Shifts::AlphaShift))
            | ((alpha as u64) << Shifts::AlphaShift);
    }

    /// Sets the blue color component of this color to blue.
    pub fn set_blue(&mut self, blue: u16) {
        self.rgba =
            (self.rgba & !(0xffff_u64 << Shifts::BlueShift)) | ((blue as u64) << Shifts::BlueShift);
    }

    /// Sets the green color component of this color to green.
    pub fn set_green(&mut self, green: u16) {
        self.rgba = (self.rgba & !(0xffff_u64 << Shifts::GreenShift))
            | ((green as u64) << Shifts::GreenShift);
    }

    /// Sets the red color component of this color to red.
    pub fn set_red(&mut self, red: u16) {
        self.rgba =
            (self.rgba & !(0xffff_u64 << Shifts::RedShift)) | ((red as u64) << Shifts::RedShift);
    }

    /// Sets the internal rgba value.
    pub fn set_u64(&mut self, rgba: u64) {
        self.rgba = rgba;
    }

    /// Returns the color as a 32-bit ARGB value.
    pub fn to_argb32(&self) -> u32 {
        ((self.alpha8() as u32) << 24)
            | ((self.red8() as u32) << 16)
            | ((self.green8() as u32) << 8)
            | (self.blue8() as u32)
    }

    /// Returns the color as a 16-bit RGB value.
    pub fn to_rgb16(&self) -> u16 {
        (self.red() & 0xf800_u16) | ((self.green() >> 10) << 5) | (self.blue() >> 11)
    }

    /// Returns the color as a 64bit unsigned integer
    pub fn to_u64(&self) -> u64 {
        self.rgba
    }

    /// Returns the color with the alpha unpremultiplied.
    pub fn unpremultiplied(&self) -> Self {
        #[cfg(target_pointer_width = "32")]
        return self.unpremultiplied_32bit();

        #[cfg(target_pointer_width = "64")]
        return self.unpremultiplied_64bit();
    }

    fn unpremultiplied_32bit(&self) -> Self {
        if self.is_opaque() || self.is_transparent() {
            return *self;
        }
        let a = self.alpha() as u32;
        let r = ((self.red() as u32 * 0xffff_u32 + a / 2) / a) as u16;
        let g = ((self.green() as u32 * 0xffff_u32 + a / 2) / a) as u16;
        let b = ((self.blue() as u32 * 0xffff_u32 + a / 2) / a) as u16;
        return Self::from_rgba64(r, g, b, a as u16);
    }

    fn unpremultiplied_64bit(&self) -> Self {
        if self.is_opaque() || self.is_transparent() {
            return *self;
        }
        let a = self.alpha() as u64;
        let fa = (0xffff00008000_u64 + a / 2) / a;
        let r = ((self.red() as u64 * fa + 0x80000000_u64) >> 32) as u16;
        let g = ((self.green() as u64 * fa + 0x80000000_u64) >> 32) as u16;
        let b = ((self.blue() as u64 * fa + 0x80000000_u64) >> 32) as u16;
        return Self::from_rgba64(r, g, b, a as u16);
    }

    #[inline]
    fn alpha_mask() -> u64 {
        0xffff_u64 << Shifts::AlphaShift
    }

    #[inline]
    fn div_257_floor(x: u32) -> u8 {
        ((x - (x >> 8)) >> 8) as u8
    }

    #[inline]
    fn div_257(x: u16) -> u8 {
        Self::div_257_floor((x + 128) as u32)
    }
}
