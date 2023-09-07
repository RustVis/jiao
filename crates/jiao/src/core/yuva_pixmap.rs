// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by LGPL-3.0 License that can be found
// in the LICENSE file.

#![allow(clippy::module_name_repetitions)]
#![allow(clippy::cast_sign_loss)]

use bit_set::BitSet;

use crate::core::color_type::ColorType;
use crate::core::data::Data;
use crate::core::image_info::{ImageInfo, YuvColorSpace};
use crate::core::pixmap::Pixmap;
use crate::core::yuva_info::{PlaneConfig, YuvaInfo, YuvaLocations, MAX_PLANES};

/// Data type for Y, U, V, and possibly A channels independent of how values are packed into planes.
#[repr(u8)]
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum DataType {
    /// 8 bit unsigned normalized
    Unorm8 = 0,

    /// 16 bit unsigned normalized
    Unorm16,

    /// 16 bit (half) floating point
    Float16,

    /// 10 bit unorm for Y, U, and V. 2 bit unorm for alpha (if present).
    Unorm10Unorm2,
}

impl From<u8> for DataType {
    #[allow(clippy::match_same_arms)]
    fn from(val: u8) -> Self {
        match val {
            0 => Self::Unorm8,
            1 => Self::Unorm16,
            2 => Self::Float16,
            3 => Self::Unorm10Unorm2,
            _ => Self::Unorm8,
        }
    }
}

pub const DATA_TYPE_CNT: usize = 4;

impl Default for DataType {
    fn default() -> Self {
        Self::Unorm8
    }
}

impl DataType {
    /// Gets the default `ColorType` to use with `num_channels` channels, each represented as `DataType`.
    ///
    /// Returns `ColorType::Unknown` if no such color type.
    #[must_use]
    #[allow(clippy::match_same_arms)]
    pub fn default_color_type(self, num_channels: i32) -> ColorType {
        match num_channels {
            1 => match self {
                Self::Unorm8 => ColorType::Gray8,
                Self::Unorm16 => ColorType::A16Unorm,
                Self::Float16 => ColorType::A16Float,
                Self::Unorm10Unorm2 => ColorType::Unknown,
            },
            2 => match self {
                Self::Unorm8 => ColorType::R8G8Unorm,
                Self::Unorm16 => ColorType::R16G16Unorm,
                Self::Float16 => ColorType::R16G16Float,
                Self::Unorm10Unorm2 => ColorType::Unknown,
            },
            3 => match self {
                // None of these are tightly packed. The intended use case is for interleaved YUVA
                // planes where we're forcing opaqueness by ignoring the alpha values.
                // There are "x" rather than "A" variants for Unorm8 and Unorm10_Unorm2 but we don't
                // choose them because 1) there is no inherent advantage and 2) there is better support
                // in the GPU backend for the "A" versions.
                Self::Unorm8 => ColorType::Rgba8888,
                Self::Unorm16 => ColorType::R16G16B16A16Unorm,
                Self::Float16 => ColorType::RgbaF16,
                Self::Unorm10Unorm2 => ColorType::Rgba1010102,
            },
            4 => match self {
                Self::Unorm8 => ColorType::Rgba8888,
                Self::Unorm16 => ColorType::R16G16B16A16Unorm,
                Self::Float16 => ColorType::RgbaF16,
                Self::Unorm10Unorm2 => ColorType::Rgba1010102,
            },
            _ => {
                log::info!("Invalid num_channels: {num_channels}");
                ColorType::Unknown
            }
        }
    }

    /// If the `ColorType` is supported for YUVA pixmaps this will return the number of YUVA channels
    /// that can be stored in a plane of this color type and what the `DataType` is of those channels.
    ///
    /// If the `ColorType` is not supported as a YUVA plane the number of channels is reported as 0
    /// and the `DataType` returned should be ignored.
    #[must_use]
    pub fn num_channels_and_data_type(_color_type: ColorType) -> (i32, Self) {
        unimplemented!()
    }
}

#[derive(Debug, Clone)]
pub struct SupportedDataTypes {
    // The bit for DataType dt with n channels is at index DATA_TYPE_CNT * (n - 1) + dt.
    bits: BitSet,
}

impl SupportedDataTypes {
    /// All legal combinations of `PlaneConfig` and `DataType` are supported.
    #[must_use]
    #[allow(clippy::cast_possible_truncation)]
    pub fn all() -> Self {
        let mut bits = BitSet::with_capacity(DATA_TYPE_CNT * 4);
        for c in 1..=4 {
            for dt in 0..DATA_TYPE_CNT {
                let data_type: DataType = (dt as u8).into();
                if data_type.default_color_type(c) != ColorType::Unknown {
                    let other = 1 << (dt + DATA_TYPE_CNT * (c as usize - 1));
                    let mut other_bits = BitSet::new();
                    other_bits.insert(other);
                    bits.union_with(&other_bits);
                }
            }
        }

        Self { bits }
    }

    /// Checks whether there is a supported combination of color types for planes structured
    /// as indicated by `PlaneConfig` with channel data types as indicated by `DataType`.
    #[must_use]
    pub fn supported(&self, config: PlaneConfig, data_type: DataType) -> bool {
        let n = config.num_planes();
        for i in 0..n {
            let c = config.num_channels_in_plane(i);
            debug_assert!((1..=4).contains(&c));

            let offset = data_type as usize + ((c - 1) as usize * DATA_TYPE_CNT);
            if self.bits.contains(offset) {
                return false;
            }
        }
        true
    }

    /// Update to add support for pixmaps with `num_channel` channels where each channel is
    /// represented as `DataType`.
    pub fn enable_data_type(&mut self, _data_type: DataType, _num_channels: i32) {
        unimplemented!();
    }
}

/// `YuvaInfo` combined with per-plane `ColorTypes` and row bytes.
///
/// Fully specifies the Pixmaps for a YUVA image without the actual pixel memory and data.
#[derive(Debug, Default, Clone)]
pub struct YuvaPixmapInfo {
    yuva_info: YuvaInfo,
    plane_infos: [ImageInfo; MAX_PLANES],
    row_bytes: [usize; MAX_PLANES],
    data_type: DataType,
}

impl YuvaPixmapInfo {
    /// Initializes the `YuvaPixmapInfo` from a `YuvaInfo` with per-plane color types and row bytes.
    ///
    /// This will be invalid if the `color_types` aren't compatible with the `YuvaInfo`
    /// or if a rowBytes entry is not valid for the plane dimensions and color type.
    /// Color type and row byte values beyond the number of planes in `YuvaInfo` are ignored.
    /// All `ColorTypes` must have the same `DataType` or this will be invalid.
    ///
    /// If `row_bytes` is nullptr then ` bpp * width` is assumed for each plane.
    #[must_use]
    pub fn from_info(
        _info: &YuvaInfo,
        _color_types: &[ColorType; MAX_PLANES],
        _row_bytes: &[usize; MAX_PLANES],
    ) -> Self {
        unimplemented!()
    }

    /// Like above but uses `default_color_type()` to determine each plane's `ColorType`.
    ///
    /// If rowBytes is nullptr then bpp*width is assumed for each plane.
    #[must_use]
    pub fn from_default_color_type(
        _info: &YuvaInfo,
        _data_type: DataType,
        _row_bytes: &[usize; MAX_PLANES],
    ) -> Self {
        unimplemented!()
    }

    #[must_use]
    pub const fn yuva_info(&self) -> &YuvaInfo {
        &self.yuva_info
    }

    #[must_use]
    pub const fn yuv_color_space(&self) -> YuvColorSpace {
        self.yuva_info.yuv_color_space()
    }

    /// The number of Pixmap planes, 0 if this `YuvaPixmapInfo` is invalid.
    #[must_use]
    pub const fn num_planes(&self) -> i32 {
        self.yuva_info.num_planes()
    }

    /// The per-YUV[A] channel data type. */
    #[must_use]
    pub const fn data_type(&self) -> DataType {
        self.data_type
    }

    /// Row bytes for the ith plane.
    ///
    /// Returns zero if `index >= num_planes()` or this `YuvaPixmapInfo` is invalid.
    #[must_use]
    pub const fn row_bytes(&self, index: i32) -> usize {
        self.row_bytes[index as usize]
    }

    /// Image info for the ith plane, or default `ImageInfo` if `i >= num_planes()`
    #[must_use]
    pub const fn plane_info(&self, index: i32) -> &ImageInfo {
        &self.plane_infos[index as usize]
    }

    /// Determine size to allocate for all planes.
    ///
    /// Optionally retrieves the per-plane sizes in `plane_sizes` if not null.
    /// If total size overflows will return `usize::MAX` and set all `plane_sizes` to `usize::MAX`.
    /// Returns 0 and fills `planes_sizes` with 0 if this `YuvaPixmapInfo` is not valid.
    #[must_use]
    pub fn compute_total_bytes(&self, _plane_sizes: &mut [usize; MAX_PLANES]) -> usize {
        unimplemented!()
    }

    // Takes an allocation that is assumed to be at least computeTotalBytes() in size and configures
    // the first numPlanes() entries in pixmaps array to point into that memory. The remaining
    // entries of pixmaps are default initialized. Fails if this YuvaPixmapInfo not valid.
    //bool initPixmapsFromSingleAllocation(void* memory, Pixmap pixmaps[kMaxPlanes]) const;

    /// Returns true if this has been configured with a non-empty dimensioned `YuvaInfo` with
    /// compatible color types and row bytes.
    #[must_use]
    pub fn is_valid(&self) -> bool {
        self.yuva_info.is_valid()
    }

    /// Is this valid and does it use color types allowed by the passed `SupportedDataTypes`?
    #[must_use]
    pub const fn is_supported(&self, _supported: &SupportedDataTypes) -> bool {
        unimplemented!()
    }
}

/// Helper to store Pixmap planes as described by a `YuvaPixmapInfo`.
///
/// Can be responsible for allocating/freeing memory for pixmaps or use external memory.
#[derive(Debug, Clone)]
pub struct YuvaPixmaps {
    planes: [Pixmap; MAX_PLANES],
    data: Option<Data>,
    yuva_info: YuvaInfo,
    data_type: DataType,
}

impl YuvaPixmaps {
    /// Use storage in Data as backing store for pixmaps' pixels.
    ///
    /// Data is retained by the `YuvaPixmaps`.
    #[must_use]
    pub const fn from_data(_info: &YuvaPixmapInfo, _data: &Option<Data>) -> Self {
        unimplemented!()
    }

    #[must_use]
    pub const fn from_pixmaps(
        _info: &YuvaInfo,
        _data_type: DataType,
        _pixmaps: &[Pixmap; MAX_PLANES],
    ) -> Self {
        unimplemented!()
    }

    #[must_use]
    pub fn recommended_rgba_color_type(_data_type: DataType) -> ColorType {
        unimplemented!()
    }

    /// Allocate space for pixmaps' pixels in the `YuvaPixmaps`. */
    #[must_use]
    pub fn allocate(_info: &YuvaPixmapInfo) -> Self {
        unimplemented!()
    }

    // Use passed in memory as backing store for pixmaps' pixels. Caller must ensure memory remains
    // allocated while pixmaps are in use. There must be at least
    // YuvaPixmapInfo::computeTotalBytes() allocated starting at memory.
    //static YUVAPixmaps FromExternalMemory(const YuvaPixmapInfo&, void* memory);

    /// Wraps existing Pixmaps.
    ///
    /// The `YuvaPixmaps` will have no ownership of the Pixmaps' pixel memory
    /// so the caller must ensure it remains valid.
    /// Will return an invalid `YuvaPixmaps` if the `YuvaInfo` isn't compatible with
    /// the Pixmap array (number of planes, plane dimensions,
    /// sufficient color channels in planes, ...).
    #[must_use]
    pub const fn from_external_pixmaps(_info: &YuvaInfo, _pixmaps: &[Pixmap; MAX_PLANES]) -> Self {
        unimplemented!()
    }

    /// Does have initialized pixmaps compatible with its `YuvaInfo`.
    #[must_use]
    pub const fn is_valid(&self) -> bool {
        !self.yuva_info.dimensions().is_empty()
    }

    #[must_use]
    pub const fn yuva_info(&self) -> &YuvaInfo {
        &self.yuva_info
    }

    #[must_use]
    pub const fn data_type(&self) -> DataType {
        self.data_type
    }

    #[must_use]
    pub const fn pixmaps_info(&self) -> YuvaPixmapInfo {
        unimplemented!()
    }

    /// Number of pixmap planes or 0 if this `YuvaPixmaps` is invalid.
    #[must_use]
    pub const fn num_planes(&self) -> i32 {
        if self.is_valid() {
            self.yuva_info.num_planes()
        } else {
            0
        }
    }

    /// Access the Pixmap planes.
    ///
    /// They are default initialized if this is not a valid `YuvaPixmaps`.
    #[must_use]
    pub const fn planes(&self) -> &[Pixmap; MAX_PLANES] {
        &self.planes
    }

    /// Get the ith Pixmap plane.
    ///
    /// Pixmap will be default initialized if `i >= num_planes` or this `YuvaPixmaps` is invalid.
    #[must_use]
    pub const fn plane(&self, index: i32) -> &Pixmap {
        &self.planes[index as usize]
    }

    /// Computes a `YuvaLocations` representation of the planar layout.
    ///
    /// The result is guaranteed to be valid if `is_valid()`.
    #[must_use]
    pub const fn to_yuva_locations(&self) -> YuvaLocations {
        unimplemented!()
    }

    /// Does this Pixmaps own the backing store of the planes?
    #[must_use]
    pub const fn owns_storage(&self) -> bool {
        self.data.is_some()
    }
}
