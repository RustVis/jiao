// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that can be found
// in the LICENSE file.

//! `AlphaType` describes how to interpret the alpha component of a pixel.
//!
//! A pixel may be opaque, or alpha, describing multiple levels of transparency.
//!
//! In simple blending, alpha weights the draw color and the destination
//! color to create a new color.
//! If alpha describes a weight from zero to one:
//!
//! new color = draw color * alpha + destination color * (1 - alpha)
//!
//! In practice alpha is encoded in two or more bits, where 1.0 equals all bits set.
//!
//! RGB may have alpha included in each component value; the stored
//! value is the original RGB multiplied by alpha.
//! Premultiplied color components improve performance.

#[repr(u8)]
#[derive(Debug, Default, Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
pub enum AlphaType {
    /// uninitialized
    #[default]
    Unknown,

    /// pixel is opaque
    ///
    /// Opaque is a hint that the `ColorType` is opaque, or that all
    /// alpha values are set to their 1.0 equivalent. If `AlphaType` is
    /// Opaque, and `ColorType` is not opaque, then the result of
    /// drawing any pixel with a alpha value less than 1.0 is undefined.
    Opaque,

    /// pixel components are premultiplied by alpha
    Premul,

    /// pixel components are independent of alpha
    Unpremul,
}

impl AlphaType {
    /// Returns true if self equals `AlphaType::Opaque`.
    ///
    /// `AlphaType::Opaque` is a hint that the `ColorType` is opaque, or that all
    /// alpha values are set to their 1.0 equivalent.
    /// If `AlphaType` is `AlphaType::Opaque`, and `ColorType` is not opaque,
    /// then the result of drawing any pixel with a alpha value less than 1.0 is undefined.
    #[must_use]
    pub fn is_opaque(self) -> bool {
        self == Self::Opaque
    }
}

// This module is used to bypass limitations of struct generic in rust.
pub mod alpha_type_mod {
    #[derive(Debug, Default, Clone, PartialEq)]
    pub struct Unknown();

    #[derive(Debug, Default, Clone, PartialEq)]
    pub struct Opaque();

    #[derive(Debug, Default, Clone, PartialEq)]
    pub struct Premul();

    #[derive(Debug, Default, Clone, PartialEq)]
    pub struct Unpremul();
}
