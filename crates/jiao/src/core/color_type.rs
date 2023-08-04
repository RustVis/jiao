// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

use crate::core::alpha_type::AlphaType;

/// `ColorType` describes how pixel bits encode color.
///
/// A pixel may be an alpha mask, a grayscale, RGB, or ARGB.
#[repr(u8)]
#[derive(Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
pub enum ColorType {
    /// uninitialized
    Unknown,

    /// pixel with alpha in 8-bit byte
    Alpha8,

    /// pixel with 5 bits red, 6 bits green, 5 bits blue, in 16-bit word
    Rgb565,

    /// pixel with 4 bits for alpha, red, green, blue; in 16-bit word
    Argb4444,

    /// pixel with 8 bits for red, green, blue, alpha; in 32-bit word
    Rgba8888,

    /// pixel with 8 bits each for red, green, blue; in 32-bit word
    Rgb888x,

    /// pixel with 8 bits for blue, green, red, alpha; in 32-bit word
    Bgra8888,

    /// 10 bits for red, green, blue; 2 bits for alpha; in 32-bit word
    Rgba1010102,

    /// 10 bits for blue, green, red; 2 bits for alpha; in 32-bit word
    Bgra1010102,

    /// pixel with 10 bits each for red, green, blue; in 32-bit word
    Rgb101010x,

    /// pixel with 10 bits each for blue, green, red; in 32-bit word
    Bgr101010x,

    /// pixel with 10 bits each for blue, green, red; in 32-bit word, extended range
    Bgr101010xXr,

    /// pixel with grayscale level in 8-bit byte
    Gray8,

    /// pixel with half floats in [0,1] for red, green, blue, alpha; in 64-bit word
    RgbaF16Norm,

    /// pixel with half floats for red, green, blue, alpha; in 64-bit word
    RgbaF16,
    /// pixel using C float for red, green, blue, alpha; in 128-bit word
    RgbaF32,

    // The following 6 colortypes are just for reading from - not for rendering to
    /// pixel with a uint8_t for red and green
    R8g8Unorm,

    /// pixel with a half float for alpha
    A16Float,

    /// pixel with a half float for red and green
    R16g16Float,

    /// pixel with a little endian uint16_t for alpha
    A16Unorm,
    /// pixel with a little endian uint16_t for red and green
    R16g16Unorm,
    /// pixel with a little endian uint16_t for red, green, blue and alpha
    R16g16b16a16Unorm,

    Srgba8888,
    R8Unorm,
}

// TODO(Shaohua):
//N32          = kBGRA_8888,// native 32-bit BGRA encoding
//N32          = kRGBA_8888,// native 32-bit RGBA encoding

impl ColorType {
    /// Returns the number of bytes required to store a pixel, including unused padding.
    /// Returns zero if type is Unknown or invalid.
    #[must_use]
    pub const fn bytes_per_pixel(self) -> i32 {
        unimplemented!()
    }

    /// Returns true if `ColorType` always decodes alpha to 1.0, making the pixel
    /// fully opaque.
    ///
    /// If true, `ColorType` does not reserve bits to encode alpha.
    /// True if alpha is always set to 1.0
    #[must_use]
    pub const fn is_always_opaque(self) -> bool {
        unimplemented!()
    }

    /// Returns true if canonical can be set to a valid `AlphaType` for `color_type`.
    ///
    /// If there is more than one valid canonical `AlphaType`, set to `alpha_type`, if valid.
    /// If true is returned and canonical is not nullptr, store valid `AlphaType`.
    ///
    /// Returns true if valid `AlphaType` can be associated with `color_type`.
    ///
    /// Returns false only if `alpha_type` is `Unknown`, color type is not Unknown,
    /// and `ColorType` is not always opaque.
    ///
    /// If false is returned, canonical is ignored.
    #[must_use]
    pub fn validate_alpha_type(self, _alpha_type: AlphaType, _canonical: &mut AlphaType) -> bool {
        unimplemented!()
    }
}
