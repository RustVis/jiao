// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that can be found
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
    #[must_use]
    pub const fn new() -> Self {
        Self {
            image: Vec::new(),
            bounds: IRect::new(),
            row_bytes: 0,
            format: MaskFormat::Bw,
        }
    }

    #[must_use]
    pub const fn from_bounds(bounds: IRect) -> Self {
        Self {
            image: Vec::new(),
            bounds,
            row_bytes: 0,
            format: MaskFormat::Bw,
        }
    }

    /// Creates a new mask by taking ownership over a mask buffer.
    ///
    /// The size needs to match the data provided.
    pub fn from_vec(image: Vec<u8>, bounds: IRect) -> Option<Self> {
        let data_len = bounds.width() as usize * bounds.height() as usize;
        if image.len() != data_len {
            return None;
        }

        Some(Self {
            image,
            bounds,
            row_bytes: 0,
            format: MaskFormat::Bw,
        })
    }

    pub fn image(&self) -> &[u8] {
        &self.image
    }

    pub fn image_mut(&mut self) -> &mut [u8] {
        &mut self.image
    }

    #[must_use]
    pub const fn width(&self) -> i32 {
        self.bounds.width()
    }

    #[must_use]
    pub const fn height(&self) -> i32 {
        self.bounds.height()
    }

    #[must_use]
    pub const fn bounds(&self) -> &IRect {
        &self.bounds
    }

    pub fn set_bounds(&mut self, bounds: IRect) {
        self.bounds = bounds;
    }

    #[must_use]
    pub const fn row_bytes(&self) -> usize {
        self.row_bytes
    }

    pub fn set_row_bytes(&mut self, row_bytes: usize) {
        self.row_bytes = row_bytes;
    }

    #[must_use]
    pub const fn format(&self) -> MaskFormat {
        self.format
    }

    pub fn set_format(&mut self, format: MaskFormat) {
        self.format = format;
    }

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
    #[allow(clippy::match_same_arms)]
    pub fn get_addr(&self, x: i32, y: i32) -> &[u8] {
        debug_assert!(self.format != MaskFormat::Bw);
        debug_assert!(self.bounds.contains(x, y));
        let shift = match self.format {
            MaskFormat::Bw => 0, // not supported
            MaskFormat::A8 => 0,
            MaskFormat::D3 => 0,
            MaskFormat::Argb32 => 2,
            MaskFormat::Lcd16 => 1,
            MaskFormat::Sdf => 0,
        };

        let row = (y - self.bounds.top()) as usize * self.row_bytes;
        let col = ((x - self.bounds.left()) as usize) << shift;
        &self.image[row + col..]
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
    pub fn prepare_destination(&self, radius_x: i32, radius_y: i32) -> Self {
        let mut dst = Self::new();
        dst.format = MaskFormat::A8;

        // dstW = srcW + 2 * radiusX;
        // TODO(Shaohua): Check addition overflow.
        let width = self.bounds.width() + radius_x + radius_x;
        // dstH = srcH + 2 * radiusY;
        let height = self.bounds.height() + radius_y + radius_y;

        // TODO(Shaohua): Check multiply overflow.
        let to_alloc: usize = width as usize * height as usize;

        // We can only deal with masks that fit in INT_MAX and sides that fit in int.
        if to_alloc > i32::MAX as usize {
            dst.bounds.set_empty();
            dst.row_bytes = 0;
            return dst;
        }

        dst.bounds.set_wh(width, height);
        dst.bounds.offset(self.bounds.x(), self.bounds.y());
        dst.bounds.offset(-radius_x, -radius_y);
        dst.row_bytes = width as usize;

        if !self.image.is_empty() {
            dst.image = Self::alloc_image(to_alloc, MaskAllocType::ZeroInit);
        }

        dst
    }
}
