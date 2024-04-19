// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that can be found
// in the LICENSE file.

#![allow(clippy::struct_excessive_bools)]

use super::encoded_origin::EncodedOrigin;
use crate::core::data::Data;

pub struct ExifMetadata {
    // The input data.
    data: Data,

    // The origin property.
    origin_present: bool,
    origin_value: EncodedOrigin,

    // The HDR headroom property.
    hdr_headroom_present: bool,
    hdr_headroom_value: f32,

    // Resolution.
    resolution_unit_present: bool,
    resolution_unit_value: u16,
    x_resolution_present: bool,
    x_resolution_value: f32,
    y_resolution_present: bool,
    y_resolution_value: f32,

    // Size in pixels.
    pixel_x_dimension_present: bool,
    pixel_x_dimension_value: u32,
    pixel_y_dimension_present: bool,
    pixel_y_dimension_value: u32,
}

impl Default for ExifMetadata {
    fn default() -> Self {
        Self {
            data: Data::default(),
            origin_present: false,
            origin_value: EncodedOrigin::TopLeft,

            hdr_headroom_present: false,
            hdr_headroom_value: 1.0,

            resolution_unit_present: false,
            resolution_unit_value: 0,
            x_resolution_present: false,
            x_resolution_value: 0.0,
            y_resolution_present: false,
            y_resolution_value: 0.0,

            pixel_x_dimension_present: false,
            pixel_x_dimension_value: 0,
            pixel_y_dimension_present: false,
            pixel_y_dimension_value: 0,
        }
    }
}

impl ExifMetadata {
    /// Parse the metadata specified in |data|.
    #[must_use]
    pub fn from_data(data: Data) -> Option<Self> {
        let little_endian = false;
        let ifd_offset: u32 = 0;
        //if !TiffImageFileDirectory::parse_header(&data, &mut little_endian, &mut ifd_offset) {
        //CodecPrintf("Failed to parse Exif header.\n");
        //return None;
        //}

        let mut obj = Self {
            data,
            ..Default::default()
        };

        obj.parse_ifd(ifd_offset, little_endian, /*isRoot=*/ true);
        Some(obj)
    }

    /// If the image encoded origin is specified, populate |out| and return true.
    ///
    /// Otherwise return false.
    #[must_use]
    #[inline]
    pub fn get_origin(&self, out: &mut EncodedOrigin) -> bool {
        if self.origin_present {
            *out = self.origin_value;
        }
        self.origin_present
    }

    /// If the HDR headroom is specified, populate |out| and return true.
    ///
    /// Otherwise return false.
    #[must_use]
    #[inline]
    pub fn get_hdr_headroom(&self, out: &mut f32) -> bool {
        if self.hdr_headroom_present {
            *out = self.hdr_headroom_value;
        }

        self.hdr_headroom_present
    }

    /// If resolution unit, x, or y is specified, populate |out| and return true.
    ///
    /// Otherwise return false.
    #[must_use]
    #[inline]
    pub fn get_resolution_unit(&self, out: &mut u16) -> bool {
        if self.resolution_unit_present {
            *out = self.resolution_unit_value;
        }
        self.resolution_unit_present
    }

    #[must_use]
    #[inline]
    pub fn get_x_resolution(&self, out: &mut f32) -> bool {
        if self.x_resolution_present {
            *out = self.x_resolution_value;
        }
        self.x_resolution_present
    }

    #[must_use]
    #[inline]
    pub fn get_y_resolution(&self, out: &mut f32) -> bool {
        if self.y_resolution_present {
            *out = self.y_resolution_value;
        }
        self.y_resolution_present
    }

    /// If pixel dimension x or y is specified, populate |out| and return true.
    ///
    /// Otherwise return false.
    #[must_use]
    #[inline]
    pub fn get_pixel_x_dimension(&self, out: &mut u32) -> bool {
        if self.pixel_x_dimension_present {
            *out = self.pixel_x_dimension_value;
        }
        self.pixel_x_dimension_present
    }

    #[must_use]
    #[inline]
    pub fn get_pixel_y_dimension(&self, out: &mut u32) -> bool {
        if self.pixel_y_dimension_present {
            *out = self.pixel_y_dimension_value;
        }
        self.pixel_y_dimension_present
    }
}

const SUB_IFD_OFFSET_TAG: u16 = 0x8769;
const ORIGIN_TAG: u16 = 0x112;
const MARKER_NOTE_TAG: u16 = 0x927c;

// Physical resolution.
const X_RESOLUTION_TAG: u16 = 0x011a;
const Y_RESOLUTION_TAG: u16 = 0x011b;
const RESOLUTION_UNIT_TAG: u16 = 0x0128;

// Size in pixels. Also sometimes called ImageWidth and ImageHeight.
const PIXEL_X_DIMENSION_TAG: u16 = 0xa002;
const PIXEL_Y_DIMENSION_TAG: u16 = 0xa003;

impl ExifMetadata {
    // Helper functions and constants for parsing the data.
    fn parse_ifd(&mut self, _ifd_offset: u32, _little_endian: bool, _is_root: bool) {
        todo!()
    }
}
