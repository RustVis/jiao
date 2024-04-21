// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that can be found
// in the LICENSE file.

use crate::core::alpha_type::AlphaType;
use crate::core::color::ColorChannelFlag;

/// `ColorType` describes how pixel bits encode color.
///
/// A pixel may be an alpha mask, a grayscale, RGB, or ARGB.
///
/// `N32` selects the native 32-bit ARGB format for the current configuration.
/// This can lead to inconsistent results across platforms, so use with caution.
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

    /// pixel with 10 bits each for blue, green, red, alpha; in 64-bit word, extended range
    Bgra10101010Xr,

    /// pixel with 10 used bits (most significant) followed by 6 unused bits for red, green, blue, alpha; in 64-bit word
    Rgba10x6,

    /// pixel with grayscale level in 8-bit byte
    Gray8,

    /// pixel with half floats in `[0, 1]` for red, green, blue, alpha; in 64-bit word
    RgbaF16Norm,

    /// pixel with half floats for red, green, blue, alpha; in 64-bit word
    RgbaF16,

    /// pixel using C float for red, green, blue, alpha; in 128-bit word
    RgbaF32,

    /// The following 6 colortypes are just for reading from - not for rendering to
    /// pixel with a uint8_t for red and green
    R8G8Unorm,

    /// pixel with a half float for alpha
    A16Float,

    /// pixel with a half float for red and green
    R16G16Float,

    /// pixel with a little endian uint16_t for alpha
    A16Unorm,

    /// pixel with a little endian uint16_t for red and green
    R16G16Unorm,

    /// pixel with a little endian uint16_t for red, green, blue and alpha
    R16G16B16A16Unorm,

    Srgba8888,

    R8Unorm,
}

// TODO(Shaohua): Check endian type with PMCOLOR_BYTE_ORDER(B,G,R,A)
/// native 32-bit BGRA encoding
#[cfg(target_endian = "big")]
pub const N32: ColorType = ColorType::Bgra8888;
#[cfg(target_endian = "little")]
/// native 32-bit RGBA encoding
pub const N32: ColorType = ColorType::Rgba8888;

pub const LAST_COLOR_TYPE: ColorType = ColorType::R8Unorm;

pub const COLOR_TYPE_COUNT: usize = LAST_COLOR_TYPE as usize + 1;

impl ColorType {
    /// Returns the number of bytes required to store a pixel, including unused padding.
    /// Returns zero if type is Unknown or invalid.
    #[must_use]
    #[allow(clippy::match_same_arms)]
    // TODO(Shaohua): Remove these methods
    pub const fn bytes_per_pixel(self) -> i32 {
        match self {
            Self::Unknown => 0,
            Self::Alpha8 => 1,
            Self::Rgb565 => 2,
            Self::Argb4444 => 2,
            Self::Rgba8888 => 4,
            Self::Bgra8888 => 4,
            Self::Rgb888x => 4,
            Self::Rgba1010102 => 4,
            Self::Rgb101010x => 4,
            Self::Bgra1010102 => 4,
            Self::Bgr101010x => 4,
            Self::Bgr101010xXr => 4,
            Self::Gray8 => 1,
            Self::RgbaF16Norm => 8,
            Self::RgbaF16 => 8,
            Self::RgbaF32 => 16,
            Self::R8G8Unorm => 2,
            Self::A16Unorm => 2,
            Self::R16G16Unorm => 4,
            Self::A16Float => 2,
            Self::R16G16Float => 4,
            Self::R16G16B16A16Unorm => 8,
            Self::Srgba8888 => 4,
            Self::R8Unorm => 1,
            Self::Bgra10101010Xr | Self::Rgba10x6 => todo!(),
        }
    }

    #[must_use]
    #[allow(clippy::match_same_arms)]
    pub const fn shift_per_pixel(self) -> i32 {
        match self {
            Self::Unknown => 0,
            Self::Alpha8 => 0,
            Self::Rgb565 => 1,
            Self::Argb4444 => 1,
            Self::Rgba8888 => 2,
            Self::Rgb888x => 2,
            Self::Bgra8888 => 2,
            Self::Rgba1010102 => 2,
            Self::Rgb101010x => 2,
            Self::Bgra1010102 => 2,
            Self::Bgr101010x => 2,
            Self::Bgr101010xXr => 2,
            Self::Gray8 => 0,
            Self::RgbaF16Norm => 3,
            Self::RgbaF16 => 3,
            Self::RgbaF32 => 4,
            Self::R8G8Unorm => 1,
            Self::A16Unorm => 1,
            Self::R16G16Unorm => 2,
            Self::A16Float => 1,
            Self::R16G16Float => 2,
            Self::R16G16B16A16Unorm => 3,
            Self::Srgba8888 => 2,
            Self::R8Unorm => 0,
            Self::Bgra10101010Xr | Self::Rgba10x6 => todo!(),
        }
    }

    /// Returns true if `ColorType` always decodes alpha to 1.0, making the pixel
    /// fully opaque.
    ///
    /// If true, `ColorType` does not reserve bits to encode alpha.
    /// True if alpha is always set to 1.0
    #[must_use]
    pub const fn is_always_opaque(self) -> bool {
        !self.channel_flag().contains(ColorChannelFlag::Alpha)
    }

    #[must_use]
    pub fn is_alpha_only(self) -> bool {
        self.channel_flag() == ColorChannelFlag::Alpha
    }

    #[must_use]
    #[allow(clippy::match_same_arms)]
    pub const fn channel_flag(&self) -> ColorChannelFlag {
        match self {
            Self::Unknown => ColorChannelFlag::empty(),
            Self::Alpha8 => ColorChannelFlag::Alpha,
            Self::Rgb565 => ColorChannelFlag::RGB,
            Self::Argb4444 => ColorChannelFlag::RGBA,
            Self::Rgba8888 => ColorChannelFlag::RGBA,
            Self::Rgb888x => ColorChannelFlag::RGB,
            Self::Bgra8888 => ColorChannelFlag::RGBA,
            Self::Rgba1010102 => ColorChannelFlag::RGBA,
            Self::Rgb101010x => ColorChannelFlag::RGB,
            Self::Bgra1010102 => ColorChannelFlag::RGBA,
            Self::Bgr101010x => ColorChannelFlag::RGB,
            Self::Bgr101010xXr => ColorChannelFlag::RGB,
            Self::Gray8 => ColorChannelFlag::Gray,
            Self::RgbaF16Norm => ColorChannelFlag::RGBA,
            Self::RgbaF16 => ColorChannelFlag::RGBA,
            Self::RgbaF32 => ColorChannelFlag::RGBA,
            Self::R8G8Unorm => ColorChannelFlag::RG,
            Self::A16Unorm => ColorChannelFlag::Alpha,
            Self::R16G16Unorm => ColorChannelFlag::RG,
            Self::A16Float => ColorChannelFlag::Alpha,
            Self::R16G16Float => ColorChannelFlag::RG,
            Self::R16G16B16A16Unorm => ColorChannelFlag::RGBA,
            Self::Srgba8888 => ColorChannelFlag::RGBA,
            Self::R8Unorm => ColorChannelFlag::Red,
            Self::Bgra10101010Xr | Self::Rgba10x6 => todo!(),
        }
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
    pub(crate) fn validate_alpha_type(
        self,
        alpha_type: AlphaType,
        canonical: &mut AlphaType,
    ) -> bool {
        match self {
            Self::Unknown => *canonical = AlphaType::Unknown,
            Self::Alpha8 | Self::A16Unorm | Self::A16Float => {
                if alpha_type == AlphaType::Unknown {
                    return false;
                }
                if alpha_type == AlphaType::Unpremul {
                    *canonical = AlphaType::Premul;
                }
            }
            Self::Argb4444
            | Self::Rgba8888
            | Self::Srgba8888
            | Self::Bgra8888
            | Self::Rgba1010102
            | Self::Bgra1010102
            | Self::RgbaF16Norm
            | Self::RgbaF16
            | Self::RgbaF32
            | Self::R16G16B16A16Unorm => {
                if alpha_type == AlphaType::Unknown {
                    return false;
                }
            }
            Self::Gray8
            | Self::R8G8Unorm
            | Self::R16G16Unorm
            | Self::R16G16Float
            | Self::Rgb565
            | Self::Rgb888x
            | Self::Rgb101010x
            | Self::Bgr101010x
            | Self::Bgr101010xXr
            | Self::R8Unorm => *canonical = AlphaType::Opaque,
            Self::Bgra10101010Xr | Self::Rgba10x6 => todo!(),
        }
        true
    }

    #[must_use]
    pub(crate) const fn is_normalized(self) -> bool {
        match self {
            Self::Unknown
            | Self::Alpha8
            | Self::Rgb565
            | Self::Argb4444
            | Self::Rgba8888
            | Self::Rgb888x
            | Self::Bgra8888
            | Self::Rgba1010102
            | Self::Rgb101010x
            | Self::Bgra1010102
            | Self::Bgr101010x
            | Self::Gray8
            | Self::RgbaF16Norm
            | Self::R8G8Unorm
            | Self::A16Unorm
            | Self::A16Float
            | Self::R16G16Unorm
            | Self::R16G16B16A16Unorm
            | Self::Srgba8888
            | Self::R8Unorm => true,
            Self::Bgr101010xXr | Self::RgbaF16 | Self::RgbaF32 | Self::R16G16Float => false,
            Self::Bgra10101010Xr | Self::Rgba10x6 => todo!(),
        }
    }

    #[must_use]
    #[allow(clippy::cast_sign_loss)]
    pub(crate) const fn min_row_bytes(self, width: i32) -> usize {
        (width * self.bytes_per_pixel()) as usize
    }

    #[must_use]
    #[allow(clippy::cast_sign_loss)]
    pub(crate) fn compute_offset(self, x: i32, y: i32, row_bytes: usize) -> usize {
        if self == Self::Unknown {
            return 0;
        }
        y as usize * row_bytes + (x << self.shift_per_pixel()) as usize
    }

    #[must_use]
    pub(crate) const fn max_bits_per_channel(self) -> i32 {
        match self {
            Self::Unknown => 0,
            Self::Argb4444 => 4,

            Self::Rgb565 => 6,

            Self::Alpha8
            | Self::Rgba8888
            | Self::Rgb888x
            | Self::Bgra8888
            | Self::Gray8
            | Self::R8G8Unorm
            | Self::Srgba8888
            | Self::R8Unorm => 8,

            Self::Rgba1010102
            | Self::Rgb101010x
            | Self::Bgra1010102
            | Self::Bgr101010x
            | Self::Bgr101010xXr => 10,

            Self::RgbaF16Norm
            | Self::A16Unorm
            | Self::A16Float
            | Self::R16G16Unorm
            | Self::R16G16B16A16Unorm
            | Self::RgbaF16
            | Self::R16G16Float => 16,

            Self::RgbaF32 => 32,
            Self::Bgra10101010Xr | Self::Rgba10x6 => todo!(),
        }
    }
}
