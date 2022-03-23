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
pub struct Rgba64(u64);

/// Make sure that the representation always has the order: red green blue alpha, independent
/// of byte order.
///
/// This way, vector operations that assume 4 16-bit values see the correct ones.
#[cfg(target_endian = "big")]
enum Shifts {
    RedShift = 48,
    GreenShift = 32,
    BlueShift = 16,
    AlphaShift = 0,
}
#[cfg(target_endian = "little")]
enum Shifts {
    RedShift = 0,
    GreenShift = 16,
    BlueShift = 32,
    AlphaShift = 48,
}

impl ops::Shl<Shifts> for u64 {
    type Output = Self;

    fn shl(self, shifts: Shifts) -> Self::Output {
        self << shifts as u8
    }
}

impl Rgba64 {
    /// Returns the Rgba64 quadruplet (red, green, blue, alpha).
    pub fn from(red: u16, green: u16, blue: u16, alpha: u16) -> Self {
        unimplemented!()
    }

    /// Constructs a Rgba64 value from the 32bit ARGB value rgb.
    pub fn from_argb32(rgb: u32) -> Self {
        unimplemented!()
    }

    /// Returns c as a Rgba64 struct.
    pub fn from_u64(c: u64) -> Self {
        Self(c)
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
        unimplemented!()
    }

    /// Returns the alpha channel as an 8-bit.
    pub fn alpha8(&self) -> u8 {
        unimplemented!()
    }

    /// Returns the alpha channel as an 8-bit.
    pub fn alpha(&self) -> u16 {
        unimplemented!()
    }

    /// Returns the blue color component as an 8-bit.
    pub fn blue8(&self) -> u8 {
        unimplemented!()
    }

    /// Returns the 16-bit blue color component.
    pub fn blue(&self) -> u16 {
        unimplemented!()
    }

    /// Returns the green color component as an 8-bit.
    pub fn green8(&self) -> u8 {
        unimplemented!()
    }

    /// Returns the 16-bit green color component.
    pub fn green(&self) -> u16 {
        unimplemented!()
    }

    /// Returns whether the color is fully opaque.
    pub fn is_opaque(&self) -> bool {
        unimplemented!()
    }

    /// Returns whether the color is transparent.
    pub fn is_transparent(&self) -> bool {
        unimplemented!()
    }

    /// Returns the color with the alpha premultiplied.
    pub fn premultiplied(&self) -> Self {
        unimplemented!()
    }

    /// Returns the red color component as an 8-bit.
    pub fn red8(&self) -> u8 {
        unimplemented!()
    }

    /// Returns the 16-bit red color component.
    pub fn red(&self) -> u16 {
        unimplemented!()
    }

    /// Sets the alpha of this color to alpha.
    pub fn set_alpha(&mut self, alpha: u16) {
        unimplemented!()
    }

    /// Sets the blue color component of this color to blue.
    pub fn set_blue(&mut self, blue: u16) {
        unimplemented!()
    }

    /// Sets the green color component of this color to green.
    pub fn set_green(&mut self, green: u16) {
        unimplemented!()
    }

    /// Sets the red color component of this color to red.
    pub fn set_red(&mut self, red: u16) {
        unimplemented!()
    }

    pub fn set_u64(&mut self, rgba: u64) {
        unimplemented!()
    }

    /// Returns the color as a 32-bit ARGB value.
    pub fn to_argb32(&self) -> u32 {
        unimplemented!()
    }

    /// Returns the color as a 16-bit RGB value.
    pub fn to_rgb16(&self) -> u16 {
        unimplemented!()
    }

    /// Returns the color as a 64bit unsigned integer
    pub fn to_u64(&self) -> u64 {
        unimplemented!()
    }

    /// Returns the color with the alpha unpremultiplied.
    pub fn unpremultiplied(&self) -> Self {
        unimplemented!()
    }
}
