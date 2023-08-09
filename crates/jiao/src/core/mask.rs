// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

#![allow(clippy::cast_sign_loss)]
#![allow(clippy::module_name_repetitions)]

use crate::base::align::Align;
use crate::core::irect::IRect;

#[repr(u8)]
#[derive(Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum MaskFormat {
    /// 1bit per pixel mask (e.g. monochrome)
    Bw,

    /// 8bits per pixel mask (e.g. antialiasing)
    A8,

    /// 3 8bit per pixl planes: alpha, mul, add
    D3,

    /// For `PMColor`
    Argb32,

    /// 565 alpha for r/g/b
    Lcd16,

    /// 8bits representing signed distance field
    Sdf,
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum MaskCreateMode {
    /// compute bounds and return
    JustComputeBounds,

    /// render into preallocate mask
    JustRenderImage,

    /// compute bounds, alloc image and render into it
    ComputeBoundsAndRenderImage,
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum MaskAllocType {
    Uninit,
    ZeroInit,
}

/// Mask is used to describe alpha bitmaps, either 1bit, 8bit, or
/// the 3-channel 3D format.
///
/// These are passed to `MaskFilter` objects.
pub struct Mask {
    image: Vec<u8>,
    bounds: IRect,
    row_bytes: usize,
    format: MaskFormat,
}

impl Mask {
    /// Returns true if the mask is empty: i.e. it has an empty bounds.
    #[must_use]
    pub const fn is_empty(&self) -> bool {
        self.bounds.is_empty()
    }

    /// Return the byte size of the mask, assuming only 1 plane.
    ///
    /// Does not account for `MaskFormat::D3D`.
    /// For that, use `compute_total_image_size()`.
    /// If there is an overflow of 32bits, then returns 0.
    #[must_use]
    pub const fn compute_image_size(&self) -> usize {
        // TODO(Shaohua): Check mul overflow.
        if self.bounds.height() > 0 {
            self.bounds.height() as usize * self.row_bytes
        } else {
            0
        }
    }

    /// Return the byte size of the mask, taking into account any extra planes (e.g. D3).
    /// If there is an overflow of 32bits, then returns 0.
    #[must_use]
    pub fn compute_total_image_size(&self) -> usize {
        let mut size = self.compute_image_size();
        if self.format == MaskFormat::D3 {
            // TODO(Shaohua): Check mul overflow.
            size *= 3;
        }
        size
    }

    /// Returns the address of the byte that holds the specified bit.
    ///
    /// Asserts that the mask is `MaskFormat::Bw`, and that x,y are in range.
    /// x,y are in the same coordiate space as bounds.
    pub fn get_addr1(&self, x: i32, y: i32) -> &[u8] {
        debug_assert!(self.format == MaskFormat::Bw);
        debug_assert!(self.bounds.contains(x, y));
        let row = (y - self.bounds.top()) as usize * self.row_bytes;
        let col = ((x - self.bounds.left()) >> 3) as usize;
        &self.image[row + col..]
    }

    /// Returns the address of the specified byte.
    ///
    /// Asserts that the mask is `MaskFormat::A8`, and that x,y are in range.
    /// x,y are in the same coordiate space as fBounds.
    pub fn get_addr8(&self, x: i32, y: i32) -> &[u8] {
        debug_assert!(self.format == MaskFormat::A8 || self.format == MaskFormat::Sdf);
        debug_assert!(self.bounds.contains(x, y));
        let row = (y - self.bounds.top()) as usize * self.row_bytes;
        let col = (x - self.bounds.left()) as usize;
        &self.image[row + col..]
    }

    /// Return the address of the specified 16bit mask.
    /// The mask's format is `MaskFormat::Lcd16`, and that (x,y) are contained
    /// in the mask's bounds.
    pub fn get_addr_lcd16(&self, x: i32, y: i32) -> &[u8] {
        debug_assert!(self.format == MaskFormat::Lcd16);
        debug_assert!(self.bounds.contains(x, y));
        let row = (y - self.bounds.top()) as usize * self.row_bytes;
        let col = (x - self.bounds.left()) as usize;
        &self.image[row + col..]
    }

    /// Return the address of the specified 32bit mask.
    ///
    /// The mask's format is 32bits, and that (x,y) are contained in the mask's bounds.
    #[must_use]
    pub fn get_addr32(&self, x: i32, y: i32) -> &[u8] {
        debug_assert!(self.format == MaskFormat::Argb32);
        debug_assert!(self.bounds.contains(x, y));
        let row = ((y - self.bounds.top()) as usize) * self.row_bytes;
        let col = (x - self.bounds.left()) as usize;
        &self.image[row + col..]
    }

    /// Returns the address of the specified pixel, computing the pixel-size
    /// at runtime based on the mask format.
    ///
    /// This will be slightly slower than using one of the routines where
    /// the format is implied by the name e.g. `get_addr8()` or `get_addr32()`.
    ///
    /// x,y must be contained by the mask's bounds.
    ///
    /// This should not be called with `MaskFormat::Bw`, as it will give unspecified
    /// results.
    pub fn get_addr(_x: i32, _y: i32) -> u8 {
        unimplemented!()
    }

    #[must_use]
    pub fn alloc_image(size: usize, _alloc_type: MaskAllocType) -> Vec<u8> {
        let aligned_size: usize = size.align4();
        vec![0; aligned_size]
    }

    pub fn free_image(image: Vec<u8>) {
        drop(image);
    }

    /// Returns initial destination mask data padded by `radius_x` and `radius_y`
    pub fn prepare_destination(_radius_x: i32, _radius_y: i32, _src: &Self) -> Self {
        unimplemented!()
    }
}
