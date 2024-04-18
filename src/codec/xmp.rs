// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that can be found
// in the LICENSE file.

use crate::codec::gainmap_info::GainmapInfo;
use crate::core::data::Data;

/// An interface to extract information from XMP metadata.
pub struct Xmp {}

impl Xmp {
    /// Create from XMP data.
    #[must_use]
    pub fn from_data(_xmp_data: Data) -> Self {
        todo!()
    }

    /// Create from standard XMP + extended XMP data.
    ///
    /// see XMP Specification Part 3: Storage in files, Section 1.1.3.1: Extended XMP in JPEG
    #[must_use]
    pub fn from_extended(_xmp_standard: Data, _xmp_extended: Data) -> Self {
        todo!()
    }

    /// Extract HDRGM gainmap parameters.
    #[must_use]
    #[inline]
    pub fn get_gainmap_info_hdrgm(&self, _info: &mut GainmapInfo) -> bool {
        todo!()
    }

    /// Extract `HDRGainMap` gainmap parameters.
    #[must_use]
    #[inline]
    pub fn get_gainmap_info_hdr_gain_map(&self, _info: &mut GainmapInfo) -> bool {
        todo!()
    }

    /// If this includes `GContainer` metadata and the `GContainer` contains an item with semantic
    /// `GainMap` and Mime of image/jpeg, then return true, and populate |offset| and |size| with
    /// that item's offset (from the end of the primary JPEG image's `EndOfImage`), and the size of
    /// the gainmap.
    #[must_use]
    #[inline]
    pub fn get_container_gainmap_location(&self, _offset: &mut usize, _size: &mut usize) -> bool {
        todo!()
    }

    /// Return the GUID of an Extended XMP if present, or null otherwise.
    #[must_use]
    #[inline]
    pub fn get_extended_xmp_guid(&self) -> String {
        todo!()
    }
}
