// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

use crate::core::alpha_type::AlphaType;
use crate::core::color_space::ColorSpace;
use crate::core::color_type::{self, ColorType};
use crate::core::size::ISize;

/// `YUVColorSpace` describes color range of YUV pixels.
///
/// The color mapping from YUV to RGB varies depending on the source.
/// YUV pixels may be generated by JPEG images, standard video streams,
/// or high definition video streams. Each has its own mapping from YUV to RGB.
///
/// JPEG YUV values encode the full range of 0 to 255 for all three components.
/// Video YUV values often range from 16 to 235 for Y and from 16 to 240 for U and V (limited).
/// Details of encoding and conversion to RGB are described in `YCbCr` color space.
///
/// The identity colorspace exists to provide a utility mapping from Y to R, U to G and V to B.
/// It can be used to visualize the YUV planes or to explicitly post process the YUV channels.

#[repr(u8)]
pub enum YUVColorSpace {
    /// describes full range
    JpegFull,

    /// describes SDTV range
    Rec601Limited,

    /// describes HDTV range
    Rec709Full,

    Rec709Limited,

    /// describes UHDTV range, non-constant-luminance
    Bt2020_8bitFull,

    Bt2020_8bitLimited,

    Bt2020_10bitFull,

    Bt2020_10bitLimited,

    Bt2020_12bitFull,

    Bt2020_12bitLimited,

    /// maps Y->R, U->G, V->B
    Identity,
}

/// `ColorInfo` describes pixel and encoding.
///
/// `ImageInfo` can be created from `ColorInfo` by providing dimensions.
///
/// It encodes how pixel bits describe alpha, transparency; color components red, blue,
/// and green; and `ColorSpace`, the range and linearity of colors.
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ColorInfo {
    color_space: Option<ColorSpace>,
    color_type: ColorType,
    alpha_type: AlphaType,
}

impl Default for ColorInfo {
    fn default() -> Self {
        Self::new()
    }
}

impl ColorInfo {
    #[must_use]
    pub const fn new() -> Self {
        Self {
            color_space: None,
            color_type: ColorType::Unknown,
            alpha_type: AlphaType::Unknown,
        }
    }

    /// Creates `ColorInfo` from `ColorType`, `AlphaType` and `ColorSpace`.
    /// `ColorSpace` defaults to `sRGB`.
    #[must_use]
    pub const fn make(
        color_type: ColorType,
        alpha_type: AlphaType,
        color_space: Option<ColorSpace>,
    ) -> Self {
        Self {
            color_space,
            color_type,
            alpha_type,
        }
    }

    /// Creates `ColorInfo` with same `ColorType`, `ColorSpace` as self, with `AlphaType` changed.
    ///
    /// Created `ColorInfo` contains `new_alpha_type` even if it is incompatible with
    /// `ColorType`, in which case `AlphaType` in `ColorInfo` is ignored.
    #[must_use]
    pub fn make_alpha_type(&self, new_alpha_type: AlphaType) -> Self {
        Self {
            color_space: self.color_space.clone(),
            color_type: self.color_type,
            alpha_type: new_alpha_type,
        }
    }

    /// Creates new `ColorInfo` with same `AlphaType`, `ColorSpace` as self, with `ColorType`
    /// changed.
    #[must_use]
    pub fn make_color_type(&self, new_color_type: ColorType) -> Self {
        Self {
            color_space: self.color_space.clone(),
            color_type: new_color_type,
            alpha_type: self.alpha_type,
        }
    }

    /// Creates `ColorInfo` with same `AlphaType`, `ColorType` as self, with `ColorSpace` changed.
    #[must_use]
    pub const fn make_color_space(&self, new_color_space: Option<ColorSpace>) -> Self {
        Self {
            color_space: new_color_space,
            color_type: self.color_type,
            alpha_type: self.alpha_type,
        }
    }

    #[must_use]
    pub const fn color_space(&self) -> &Option<ColorSpace> {
        &self.color_space
    }

    #[must_use]
    pub const fn color_type(&self) -> ColorType {
        self.color_type
    }

    #[must_use]
    pub const fn alpha_type(&self) -> AlphaType {
        self.alpha_type
    }

    #[must_use]
    pub fn is_opaque(&self) -> bool {
        self.alpha_type.is_opaque() || self.color_type.is_always_opaque()
    }

    #[must_use]
    pub const fn gamma_close_to_srgb(&self) -> bool {
        if let Some(color_space) = &self.color_space {
            color_space.gamma_close_to_srgb()
        } else {
            false
        }
    }

    /// Returns number of bytes per pixel required by `ColorType`.
    ///
    /// Returns zero if `color_type()` is `ColorType::Unknown`.
    #[must_use]
    pub const fn bytes_per_pixel(&self) -> i32 {
        self.color_type.bytes_per_pixel()
    }

    /// Returns bit shift converting row bytes to row pixels.
    ///
    /// Returns zero for `ColorType::Unknown`.
    ///
    /// One of: 0, 1, 2, 3, 4; left shift to convert pixels to bytes
    #[must_use]
    pub const fn shift_per_pixel(&self) -> i32 {
        self.color_type.shift_per_pixel()
    }

    /// Returns true if contains a valid `color_type` and `alpha_type`.
    pub(crate) fn is_valid(&self) -> bool {
        self.color_type != ColorType::Unknown && self.alpha_type != AlphaType::Unknown
    }
}

/// `ImageInfo` describes pixel dimensions and encoding.
///
/// `Bitmap`, `Image`, `PixMap`, and `Surface` can be created from `ImageInfo`.
///
/// `ImageInfo` can be retrieved from `Bitmap` and `Pixmap`, but not from `Image` and
/// `Surface`.
/// For example, `Image` and `Surface` implementations may defer pixel depth,
/// so may not completely specify `ImageInfo`.
///
/// `ImageInfo` contains dimensions, the pixel integral width and height.
/// It encodes how pixel bits describe alpha, transparency; color components red, blue,
/// and green; and `ColorSpace`, the range and linearity of colors.
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ImageInfo {
    color_info: ColorInfo,
    dimensions: ISize,
}

impl Default for ImageInfo {
    fn default() -> Self {
        Self::new()
    }
}

impl ImageInfo {
    /// Creates an empty `ImageInfo` with `ColorType::Unknown`, `AlphaType::Unknown`,
    /// a width and height of zero, and no `ColorSpace`.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            color_info: ColorInfo::new(),
            dimensions: ISize::make(0, 0),
        }
    }

    /// Creates `ImageInfo` from integral dimensions width and height, `ColorType`,
    /// `AlphaType`, and optionally `ColorSpace`.
    ///
    /// If `ColorSpace` is None and `ImageInfo` is part of drawing source: `ColorSpace`
    /// defaults to `sRGB`, mapping into `Surface` `ColorSpace`.
    ///
    /// # Parameters
    /// Parameters are not validated to see if their values are legal, or that the
    /// combination is supported.
    ///
    /// - `width` - pixel column count; must be zero or greater
    /// - `height` - pixel row count; must be zero or greater
    /// - `cs` - range of colors; may be None
    #[must_use]
    pub const fn make(
        width: i32,
        height: i32,
        ct: ColorType,
        at: AlphaType,
        cs: Option<ColorSpace>,
    ) -> Self {
        Self {
            color_info: ColorInfo::make(ct, at, cs),
            dimensions: ISize::make(width, height),
        }
    }

    #[must_use]
    pub const fn make_dimensions(
        dimensions: ISize,
        ct: ColorType,
        at: AlphaType,
        cs: Option<ColorSpace>,
    ) -> Self {
        Self {
            color_info: ColorInfo::make(ct, at, cs),
            dimensions,
        }
    }

    /// Creates `ImageInfo` from integral dimensions and `ColorInfo`.
    ///
    /// # Parameters
    /// Parameters are not validated to see if their values are legal, or that the
    /// combination is supported.
    ///
    /// - `dimensions` - pixel column and row count; must be zeros or greater
    /// - `color_info` - the pixel encoding consisting of `ColorType`, `AlphaType`, and
    ///                  `ColorSpace` (which may be nullptr)
    #[must_use]
    pub const fn make_color_info(dimensions: ISize, color_info: ColorInfo) -> Self {
        Self {
            color_info,
            dimensions,
        }
    }

    /// Creates `ImageInfo` from integral dimensions width and height, `ColorType` N32,
    /// `AlphaType`, and optionally `ColorSpace`.
    ///
    /// `ColorType` N32 will equal either `ColorType::Bgra8888` or `ColorType::Rgba8888`,
    /// whichever is optimal.
    ///
    /// If `ColorSpace` is None and `ImageInfo` is part of drawing source: `ColorSpace`
    /// defaults to `sRGB`, mapping into `Surface` `ColorSpace`.
    ///
    /// # Parameters
    /// Parameters are not validated to see if their values are legal, or that the
    /// combination is supported.
    ///
    /// - `width` - pixel column count; must be zero or greater
    /// - `height` -  pixel row count; must be zero or greater
    /// - `cs` - range of colors; may be None
    #[must_use]
    pub const fn make_n32(width: i32, height: i32, at: AlphaType, cs: Option<ColorSpace>) -> Self {
        Self {
            color_info: ColorInfo::make(color_type::N32, at, cs),
            dimensions: ISize::make(width, height),
        }
    }

    /// Creates `ImageInfo` from integral dimensions width and height, `ColorType` N32,
    /// `AlphaType`, with `sRGB` `ColorSpace`.
    ///
    /// # Parameters
    /// Parameters are not validated to see if their values are legal, or that the
    /// combination is supported.
    ///
    /// - `width` - pixel column count; must be zero or greater
    /// - `height` - pixel column count; must be zero or greater
    #[must_use]
    pub const fn make_s32(width: i32, height: i32, at: AlphaType) -> Self {
        Self {
            // TODO(Shaohua): Set color space.
            color_info: ColorInfo::make(color_type::N32, at, None),
            dimensions: ISize::make(width, height),
        }
    }

    /// Creates `ImageInfo` from integral dimensions width and height, `ColorType` N32,
    /// `AlphaType::Premul`, with optional `ColorSpace`.
    ///
    /// If `ColorSpace` is None and `ImageInfo` is part of drawing source: `ColorSpace`
    /// defaults to `sRGB`, mapping into `Surface` `ColorSpace`.
    ///
    /// # Parameters
    /// Parameters are not validated to see if their values are legal, or that the
    /// combination is supported.
    ///
    /// - `width` - pixel column count; must be zero or greater
    /// - `height` -  pixel row count; must be zero or greater
    /// - `cs` - range of colors; may be None
    #[must_use]
    pub const fn make_n32_premul(width: i32, height: i32, cs: Option<ColorSpace>) -> Self {
        Self {
            color_info: ColorInfo::make(ColorType::Alpha8, AlphaType::Premul, cs),
            dimensions: ISize::make(width, height),
        }
    }

    /// Creates `ImageInfo` from integral dimensions width and height, `ColorType::Alpha8`,
    /// `AlphaType::Premul`, with `ColorSpace` set to None.
    ///
    /// # Parameters
    /// - `width` - pixel column count; must be zero or greater
    /// - `height` - pixel row count; must be zero or greater
    #[must_use]
    pub const fn make_a8(width: i32, height: i32) -> Self {
        Self {
            color_info: ColorInfo::make(ColorType::Alpha8, AlphaType::Premul, None),
            dimensions: ISize::make(width, height),
        }
    }

    /// Creates `ImageInfo` from integral dimensions width and height, `ColorType::Unknown`,
    /// `AlphaType::Unknown`, with `ColorSpace` set to None.
    ///
    /// Returned `ImageInfo` as part of source does not draw, and as part of destination
    /// can not be drawn to.
    ///
    /// # Parameters
    /// - `width` - pixel column count; must be zero or greater
    /// - `height` - pixel row count; must be zero or greater
    #[must_use]
    pub const fn make_unknown(width: i32, height: i32) -> Self {
        Self {
            color_info: ColorInfo::make(ColorType::Unknown, AlphaType::Unknown, None),
            dimensions: ISize::make(width, height),
        }
    }

    /// Creates `ImageInfo` from integral dimensions width and height set to zero,
    /// `ColorType::Unknown`, `AlphaType::Unknown`, with `ColorSpace` set to None.
    #[must_use]
    pub const fn make_unknown_empty() -> Self {
        Self::make_unknown(0, 0)
    }
}
