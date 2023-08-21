// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

use crate::codec::encoded_origin::EncodedOrigin;
use crate::core::image_info::YuvColorSpace;
use crate::core::matrix::Matrix;
use crate::core::size::ISize;

pub const MAX_PLANES: usize = 4;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum YuvaChannels {
    Y,
    U,
    V,
    A,
}
pub const YUVA_CHANNEL_COUNT: usize = 4;

#[derive(Debug, Clone)]
pub struct YuvaLocation {}

pub type YuvaLocations = [YuvaLocation; YUVA_CHANNEL_COUNT];

/// Specifies how YUV (and optionally A) are divided among planes.
///
/// Planes are separated by underscores in the enum value names.
/// Within each plane the pixmap/texture channels are mapped to the YUVA channels
/// in the order specified, e.g. for `Y_UV` Y is in channel 0 of plane 0,
/// U is in channel 0 of plane 1, and V is in channel 1 of plane 1.
/// Channel ordering within a pixmap/texture given the channels it contains:
/// - A:                       0:A
/// - Luminance/Gray:          0:Gray
/// - Luminance/Gray + Alpha:  0:Gray, 1:A
/// - RG                       0:R,    1:G
/// - RGB                      0:R,    1:G, 2:B
/// - RGBA                     0:R,    1:G, 2:B, 3:A
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum PlaneConfig {
    Unknown,

    /// Plane 0: Y, Plane 1: U,  Plane 2: V
    Y_U_V,

    /// Plane 0: Y, Plane 1: V,  Plane 2: U
    Y_V_U,

    /// Plane 0: Y, Plane 1: UV
    Y_UV,

    /// Plane 0: Y, Plane 1: VU
    Y_VU,

    /// Plane 0: YUV
    YUV,

    /// Plane 0: UYV
    UYV,

    /// Plane 0: Y, Plane 1: U,  Plane 2: V, Plane 3: A
    Y_U_V_A,

    /// Plane 0: Y, Plane 1: V,  Plane 2: U, Plane 3: A
    Y_V_U_A,

    /// Plane 0: Y, Plane 1: UV, Plane 2: A
    Y_UV_A,

    /// Plane 0: Y, Plane 1: VU, Plane 2: A
    Y_VU_A,

    /// Plane 0: YUVA
    YUVA,

    /// Plane 0: UYVA
    UYVA,
}

impl Default for PlaneConfig {
    fn default() -> Self {
        Self::Unknown
    }
}

impl PlaneConfig {
    /// Number of planes for a given `PlaneConfig`.
    #[must_use]
    #[allow(clippy::match_same_arms)]
    pub const fn num_planes(self) -> i32 {
        match self {
            Self::Unknown => 0,
            Self::Y_U_V => 3,
            Self::Y_V_U => 3,
            Self::Y_UV => 2,
            Self::Y_VU => 2,
            Self::YUV => 1,
            Self::UYV => 1,
            Self::Y_U_V_A => 4,
            Self::Y_V_U_A => 4,
            Self::Y_UV_A => 3,
            Self::Y_VU_A => 3,
            Self::YUVA => 1,
            Self::UYVA => 1,
        }
    }

    /// Number of Y, U, V, A channels in the ith plane for a given `PlaneConfig` (or 0 if i is invalid).
    #[must_use]
    #[allow(clippy::match_same_arms)]
    pub const fn num_channels_in_plane(self, index: i32) -> i32 {
        match self {
            Self::Unknown => 0,

            Self::Y_U_V | Self::Y_V_U => {
                if index >= 0 && index < 3 {
                    1
                } else {
                    0
                }
            }
            Self::Y_UV | Self::Y_VU => match index {
                0 => 1,
                1 => 2,
                _ => 0,
            },
            Self::YUV | Self::UYV => {
                if index == 0 {
                    3
                } else {
                    0
                }
            }
            Self::Y_U_V_A | Self::Y_V_U_A => {
                if index >= 0 && index < 4 {
                    1
                } else {
                    0
                }
            }
            Self::Y_UV_A | Self::Y_VU_A => match index {
                0 => 1,
                1 => 2,
                2 => 1,
                _ => 0,
            },
            Self::YUVA | Self::UYVA => {
                if index == 0 {
                    4
                } else {
                    0
                }
            }
        }
    }

    /// SubsamplingFactors(Subsampling) if planedIdx refers to a U/V plane and otherwise {1, 1} if
    /// inputs are valid. Invalid inputs consist of incompatible PlaneConfig/Subsampling/planeIdx
    /// combinations. {0, 0} is returned for invalid inputs.
    #[must_use]
    pub const fn subsampling_factors(self, _sample: Subsampling, _plane_idx: i32) -> (i32, i32) {
        unimplemented!()
    }

    /// Given a `PlaneConfig` and a set of channel flags for each plane, convert to
    /// `YuvaLocations` representation.
    ///
    /// Fails if channel flags aren't valid for the `PlaneConfig` (i.e. don't have
    /// enough channels in a plane) by returning an invalid set of locations (plane indices are -1).
    #[must_use]
    pub fn get_yuva_locations(self, _plane_channel_flags: &[u32]) -> YuvaLocations {
        unimplemented!()
    }

    /// Does the `PlaneConfig` have alpha values?
    #[must_use]
    pub const fn has_alpha(self) -> bool {
        unimplemented!()
    }
}

/// UV subsampling is also specified in the enum value names using J:a:b notation (e.g. 4:2:0 is
/// 1/2 horizontal and 1/2 vertical resolution for U and V).
///
/// If alpha is present it is not sub-sampled. Note that Subsampling values other than
/// k444 are only valid with `PlaneConfig` values that have U and V in different planes
/// than Y (and A, if present).
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum Subsampling {
    Unknown,

    /// No subsampling. UV values for each Y.
    K444,

    /// 1 set of UV values for each 2x1 block of Y values.
    K422,

    /// 1 set of UV values for each 2x2 block of Y values.
    K420,

    /// 1 set of UV values for each 1x2 block of Y values.
    K440,

    /// 1 set of UV values for each 4x1 block of Y values.
    K411,

    /// 1 set of UV values for each 4x2 block of Y values.
    K410,
}

impl Default for Subsampling {
    fn default() -> Self {
        Self::Unknown
    }
}

impl Subsampling {
    /// ratio of Y/A values to U/V values in x and y.
    #[must_use]
    pub fn subsampling_factors(self) -> (i32, i32) {
        unimplemented!()
    }
}

/// Describes how subsampled chroma values are sited relative to luma values.
///
/// Currently only centered siting is supported but will expand to support additional sitings.
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum Siting {
    /// Subsampled chroma value is sited at the center of the block of corresponding luma values.
    Centered,
}

/// Specifies the structure of planes for a YUV image with optional alpha.
///
/// The actual planar data is not part of this structure and depending on usage
/// is in external textures or pixmaps.
#[derive(Debug, Clone)]
pub struct YuvaInfo {
    dimensions: ISize,

    plane_config: PlaneConfig,
    subsampling: Subsampling,

    yuv_color_space: YuvColorSpace,

    /// YUVA data often comes from formats like JPEG that support EXIF orientation.
    ///
    /// Code that operates on the raw YUV data often needs to know that orientation.
    origin: EncodedOrigin,

    siting_x: Siting,
    siting_y: Siting,
}

impl YuvaInfo {
    /// Given image dimensions, a planer configuration, subsampling, and origin,
    /// determine the expected size of each plane.
    ///
    /// Returns the number of expected planes. planeDimensions[0] through planeDimensions[<ret>]
    /// are written.
    /// The input image dimensions are as displayed (after the planes have been transformed
    /// to the intended display orientation).
    /// The plane dimensions are output as the planes are stored in memory (may be
    /// rotated from image dimensions).
    pub fn plane_dimensions_impl(
        _image_dimensions: ISize,
        _plane_config: PlaneConfig,
        _sample: Subsampling,
        _origin: EncodedOrigin,
        _plane_dimensions: &mut [ISize; MAX_PLANES],
    ) -> i32 {
        unimplemented!()
    }

    #[must_use]
    pub const fn new() -> Self {
        Self {
            dimensions: ISize::new(),
            plane_config: PlaneConfig::Unknown,
            subsampling: Subsampling::Unknown,
            yuv_color_space: YuvColorSpace::Identity,
            origin: EncodedOrigin::TopLeft,
            siting_x: Siting::Centered,
            siting_y: Siting::Centered,
        }
    }

    /**
     * 'dimensions' should specify the size of the full resolution image (after planes have been
     * oriented to how the image is displayed as indicated by 'origin').
     */
    #[must_use]
    pub const fn from(
        dimensions: ISize,
        plane_config: PlaneConfig,
        subsampling: Subsampling,
        yuv_color_space: YuvColorSpace,
        origin: EncodedOrigin,
        siting_x: Siting,
        siting_y: Siting,
    ) -> Self {
        Self {
            dimensions,
            plane_config,
            subsampling,
            yuv_color_space,
            origin,
            siting_x,
            siting_y,
        }
    }

    #[must_use]
    pub const fn plane_config(&self) -> PlaneConfig {
        self.plane_config
    }

    #[must_use]
    pub const fn subsampling(&self) -> Subsampling {
        self.subsampling
    }

    #[must_use]
    pub const fn plane_subsampling_factors(&self, plane_idx: i32) -> (i32, i32) {
        self.plane_config
            .subsampling_factors(self.subsampling, plane_idx)
    }

    /// Dimensions of the full resolution image (after planes have been oriented to how the image
    /// is displayed as indicated by fOrigin).
    #[must_use]
    pub const fn dimensions(&self) -> ISize {
        self.dimensions
    }

    #[must_use]
    pub const fn width(&self) -> i32 {
        self.dimensions.width()
    }

    #[must_use]
    pub const fn height(&self) -> i32 {
        self.dimensions.height()
    }

    #[must_use]
    pub const fn yuv_color_space(&self) -> YuvColorSpace {
        self.yuv_color_space
    }

    #[must_use]
    pub const fn siting_x(&self) -> Siting {
        self.siting_x
    }

    #[must_use]
    pub const fn siting_y(&self) -> Siting {
        self.siting_y
    }

    #[must_use]
    pub const fn origin(&self) -> EncodedOrigin {
        self.origin
    }

    #[must_use]
    pub fn origin_matrix(&self) -> Matrix {
        //EncodedOriginToMatrix(fOrigin, this->width(), this->height())
        unimplemented!()
    }

    #[must_use]
    pub const fn has_alpha(&self) -> bool {
        self.plane_config.has_alpha()
    }

    #[must_use]
    pub const fn num_planes(&self) -> i32 {
        self.plane_config.num_planes()
    }

    #[must_use]
    pub const fn num_channels_in_plane(&self, index: i32) -> i32 {
        self.plane_config.num_channels_in_plane(index)
    }

    /// Returns the number of planes and initializes planeDimensions[0]..planeDimensions[<ret>] to
    /// the expected dimensions for each plane.
    ///
    /// Dimensions are as stored in memory, before transformation to image display space
    /// as indicated by origin().
    pub fn plane_dimensions(&self, plane_dimensions: &mut [ISize; MAX_PLANES]) -> i32 {
        Self::plane_dimensions_impl(
            self.dimensions,
            self.plane_config,
            self.subsampling,
            self.origin,
            plane_dimensions,
        )
    }

    /// Given a per-plane row bytes, determine size to allocate for all planes.
    ///
    /// Optionally retrieves the per-plane byte sizes in planeSizes if not null.
    /// If total size overflows will return `usize::MAX` and set all `plane_sizes` to `usize::MAX`.
    pub fn compute_total_bytes(
        &self,
        _row_bytes: &[usize; MAX_PLANES],
        _plane_sizes: &mut [usize; MAX_PLANES],
    ) -> usize {
        unimplemented!()
    }

    /// Given a set of channel flags for each plane, converts `plane_config` to
    /// `YuvaLocations` representation.
    ///
    /// Fails if the channel flags aren't valid for the `PlaneConfig`
    /// (i.e. don't have enough channels in a plane) by returning
    /// default initialized locations (all plane indices are -1).
    #[must_use]
    pub fn to_yuva_locations(&self, _channel_flags: &[u32]) -> YuvaLocations {
        unimplemented!()
    }

    /// Makes a `YuvaInfo` that is identical to this one but with the passed Subsampling.
    ///
    /// If the passed Subsampling is not k444 and this info's `PlaneConfig`
    /// is not compatible with chroma subsampling (because Y is in the same plane as UV)
    /// then the result will be an invalid `YuvaInfo`.
    #[must_use]
    pub const fn from_subsampling(_sample: Subsampling) -> Self {
        unimplemented!()
    }

    /// Makes a `YuvaInfo` that is identical to this one but with the passed dimensions.
    ///
    /// If the passed dimensions is empty then the result will be an invalid `YuvaInfo`.
    #[must_use]
    pub const fn from_dimensions(_dimensions: ISize) -> Self {
        unimplemented!()
    }

    #[must_use]
    pub fn is_valid(&self) -> bool {
        self.plane_config != PlaneConfig::Unknown
    }
}
