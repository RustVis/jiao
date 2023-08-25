// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

use std::mem;

use crate::core::bitmap::Bitmap;
use crate::core::color_space::ColorSpace;
use crate::core::image_info::ImageInfo;
use crate::core::pixmap::Pixmap;
use crate::core::size::{ISize, Size};

/// Mipmap will generate mipmap levels when given a base mipmap level image.
///
/// Any function which deals with mipmap levels indices will start with index 0
/// being the first mipmap level which was generated.
/// Said another way, it does not include the base level in its range.
pub struct Mipmap {
    color_space: Option<ColorSpace>,
    // managed by the baseclass, may be null due to onDataChanged.
    levels: Vec<Level>,
}

/// We use a block of (possibly discardable) memory to hold an array of Level structs,
/// followed by the pixel data for each level.
///
/// On 32-bit platforms, Level would naturally be 4 byte aligned, so the pixel data
/// could end up with 4 byte alignment.
/// If the pixel data is F16, it must be 8 byte aligned.
/// To ensure this, keep the Level struct 8 byte aligned as well.
// TODO(Shaohua): aligned(8)
#[derive(Debug, Clone)]
pub struct Level {
    pixmap: Pixmap,
    scale: Size, // < 1.0
}

impl Mipmap {
    /// Allocate and fill-in a mipmap.
    ///
    /// If `compute_contents` is false, we just allocated and compute the sizes/rowbytes,
    /// but leave the pixel-data uninitialized.
    pub fn from_bitmap(_src: &Bitmap, _compute_contents: bool) -> Self {
        unimplemented!()
    }

    /// Determines how many levels a Mipmap will have without creating that mipmap.
    ///
    /// This does not include the base mipmap level that the user provided when
    /// creating the Mipmap.
    #[must_use]
    pub fn compute_level_count(_base_width: i32, _base_height: i32) -> i32 {
        unimplemented!()
    }

    // Determines the size of a given mipmap level.
    // |level| is an index into the generated mipmap levels. It does not include
    // the base level. So index 0 represents mipmap level 1.
    #[must_use]
    pub fn compute_level_size(_base_width: i32, _base_height: i32, _level: i32) -> ISize {
        unimplemented!()
    }

    /// Computes the fractional level based on the scaling in X and Y.
    #[must_use]
    pub fn compute_level_by_scale(_scale_size: Size) -> f32 {
        unimplemented!()
    }

    #[must_use]
    pub fn extract_level(&self, _scale_size: Size) -> Option<Level> {
        unimplemented!()
    }

    /// countLevels returns the number of mipmap levels generated (which does not
    /// include the base mipmap level).
    #[must_use]
    pub fn count_levels(&self) -> usize {
        unimplemented!()
    }

    /// |index| is an index into the generated mipmap levels.
    ///
    /// It does not include the base level.
    /// So index 0 represents mipmap level 1.
    #[must_use]
    pub fn get_level(&self, _index: usize) -> Option<Level> {
        unimplemented!()
    }

    #[must_use]
    pub fn valid_for_root_level(&self, _image_info: &ImageInfo) -> bool {
        unimplemented!()
    }
}

impl Mipmap {
    #[must_use]
    fn on_data_change(&mut self, mut new_data: Vec<Level>) -> Vec<Level> {
        mem::swap(&mut self.levels, &mut new_data);
        new_data
    }

    #[must_use]
    fn with_size(_size: usize) -> Self {
        unimplemented!()
    }

    #[must_use]
    fn alloc_levels_size(_level_count: usize, _pixel_size: usize) -> usize {
        unimplemented!()
    }
}
