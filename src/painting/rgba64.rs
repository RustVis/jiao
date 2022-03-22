// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

/// The Rgba64 struct contains a 64-bit RGB color.
///
/// Rgba64 is a 64-bit data-structure containing four 16-bit color channels: Red, green, blue and alpha.
///
/// Rgba64 can be used as a replacement for `Rgb` when higher precision is needed.
/// In particular a premultiplied Rgba64 can operate on unpremultiplied Rgb
/// without loss of precision except for alpha 0
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Rgba64(u64);

impl Rgba64 {
    /// Returns the Rgba64 quadruplet (red, green, blue, alpha).
    pub fn from(red: u16, green: u16, blue: u16, alpha: u16) -> Self {
        unimplemented!()
    }

    /// Returns c as a Rgba64 struct.
    pub fn from_rgba64(c: u64) -> Self {
        Self(c)
    }

    /// Constructs a Rgba64 value from the 32bit ARGB value rgb.
    pub fn from_argb32(rgb: u32) -> Self {
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
}
