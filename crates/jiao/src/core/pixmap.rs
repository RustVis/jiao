// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

use crate::core::alpha_type::AlphaType;
use crate::core::color::{Color, Color4F};
use crate::core::color_space::ColorSpace;
use crate::core::color_type::ColorType;
use crate::core::image_info::ImageInfo;
use crate::core::irect::IRect;
use crate::core::sampling_options::SamplingOptions;
use crate::core::size::ISize;

/// Pixmap provides a utility to pair `ImageInfo` with pixels and row bytes.
///
/// Pixmap is a low level class which provides convenience functions to access
/// raster destinations. Canvas can not draw Pixmap, nor does Pixmap provide
/// a direct drawing destination.
///
/// Use Bitmap to draw pixels referenced by Pixmap; use Surface to draw into
/// pixels referenced by Pixmap.
///
/// Pixmap does not try to manage the lifetime of the pixel memory.
/// Use `PixelRef` to manage pixel memory; `PixelRef` is safe across threads.
#[derive(Debug, Default, Clone)]
pub struct Pixmap {
    row_bytes: usize,
    info: ImageInfo,
    pixels: Vec<u8>,
}

impl Pixmap {
    /// Creates an empty Pixmap without pixels, with `ColorType::Unknown`, with
    /// `AlphaType::Unknown`, and with a width and height of zero.
    ///
    /// Use `reset()` to associate pixels, `ColorType`, `AlphaType`, width, and height
    /// after Pixmap has been created.
    ///
    /// Returns empty Pixmap.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates Pixmap from info width, height, `AlphaType`, and `ColorType`.
    ///
    /// buf points to pixels slice. `row_bytes` should be `info.width() * info.bytes_per_pixel()`,
    /// or larger.
    ///
    /// No parameter checking is performed; it is up to the caller to ensure that
    /// addr and `row_bytes` agree with info.
    ///
    /// The memory lifetime of pixels is managed by the caller. When Pixmap goes
    /// out of scope, addr is unaffected.
    ///
    /// Pixmap may be later modified by reset() to change its size, pixel type, or
    /// storage.
    ///
    /// # Parameters
    /// - `info` - width, height, `AlphaType`, `ColorType` of `ImageInfo`
    /// - `row_bytes` - size of one row of addr; width times pixel size, or larger
    /// - `pixel` - point to pixels allocated by caller
    ///
    /// Returns initialized Pixmap
    #[must_use]
    pub fn from(info: ImageInfo, row_bytes: usize, pixels: &[u8]) -> Self {
        Self {
            row_bytes,
            info,
            pixels: Vec::from(pixels),
        }
    }

    /// Sets width, height, row bytes to zero; pixel to empty; `ColorType` to
    /// `ColorType::Unknown`; and `AlphaType` to `AlphaType::Unknown`.
    ///
    /// The prior pixels are unaffected; it is up to the caller to release pixels
    /// memory if desired.
    pub fn reset(&mut self) {
        unimplemented!()
    }

    /// Sets width, height, `AlphaType`, and `ColorType` from info.
    ///
    /// Sets row bytes from `row_bytes`, which should be `info.width() * info.bytes_per_pixel()`,
    /// or larger.
    ///
    /// # Parameters
    /// - `info` - width, height, `AlphaType`, `ColorType` of `ImageInfo`
    /// - `row_bytes` - size of one row of addr; width times pixel size, or larger
    pub fn set(&mut self, info: ImageInfo, row_bytes: usize, pixels: &[u8]) {
        self.info = info;
        self.row_bytes = row_bytes;
        self.pixels = pixels.to_vec();
    }

    /// Changes `ColorSpace` in `ImageInfo`; preserves width, height, `AlphaType`, and
    /// `ColorType` in Image, and leaves pixel address and row bytes unchanged.
    pub fn set_color_space(&mut self, _color_space: &ColorSpace) {
        unimplemented!()
    }

    /// Sets subset width, height, pixel address to intersection of Pixmap with area,
    /// if intersection is not empty; and return true.
    /// Otherwise, leave subset unchanged and return false.
    ///
    /// Failing to read the return value generates a compile time warning.
    ///
    /// # Parameters
    /// - `subset` - storage for width, height, pixel address of intersection
    /// - `area` - bounds to intersect with Pixmap
    ///
    /// Returns true if intersection of Pixmap and area is not empty.
    pub fn extract_subset(&mut self, _subset: &mut Self, _area: &IRect) -> bool {
        unimplemented!()
    }

    /// Returns width, height, `AlphaType`, `ColorType`, and `ColorSpace`.
    #[must_use]
    pub const fn info(&self) -> &ImageInfo {
        &self.info
    }

    /// Returns row bytes, the interval from one pixel row to the next.
    /// Row bytes is at least as large as: `width() * info().bytes_per_pixel()`.
    ///
    /// Returns zero if `color_type()` is `ColorType::Unknown`.
    ///
    /// It is up to the Bitmap creator to ensure that row bytes is a useful value.
    #[must_use]
    pub const fn row_bytes(&self) -> usize {
        self.row_bytes
    }

    /// Returns pixel slice.
    #[must_use]
    pub fn pixels(&self) -> &[u8] {
        &self.pixels
    }

    /// Returns pixel count in each pixel row.
    ///
    /// Should be equal or less than: `row_bytes() / info().bytes_per_pixel()`.
    #[must_use]
    pub const fn width(&self) -> i32 {
        self.info.width()
    }

    #[must_use]
    pub const fn height(&self) -> i32 {
        self.info.height()
    }

    /// Return the dimensions of the pixmap (from its `ImageInfo`)
    #[must_use]
    pub const fn dimensions(&self) -> ISize {
        self.info.dimensions()
    }

    #[must_use]
    pub const fn color_type(&self) -> ColorType {
        self.info.color_type()
    }

    #[must_use]
    pub const fn alpha_type(&self) -> AlphaType {
        self.info.alpha_type()
    }

    /// Returns `ColorSpace`, the range of colors, associated with `ImageInfo`.
    ///
    /// The reference count of `ColorSpace` is unchanged.
    /// The returned `ColorSpace` is immutable.
    #[must_use]
    pub const fn color_space(&self) -> &Option<ColorSpace> {
        self.info.color_space()
    }

    /// Returns true if `AlphaType` is Opaque.
    ///
    /// Does not check if `ColorType` allows alpha, or if any pixel value has
    /// transparency.
    #[must_use]
    pub fn is_opaque(&self) -> bool {
        self.info.is_opaque()
    }

    /// Returns `IRect { 0, 0, width(), height() }`.
    ///
    /// Returns integral rectangle from origin to width() and height()
    #[must_use]
    pub const fn bounds(&self) -> IRect {
        IRect::from_wh(self.width(), self.height())
    }

    /// Returns number of pixels that fit on row.
    ///
    /// Should be greater than or equal to width().
    #[must_use]
    pub const fn row_bytes_as_pixels(&self) -> i32 {
        (self.row_bytes >> self.shift_per_pixel()) as i32
    }

    /// Returns bit shift converting row bytes to row pixels.
    ///
    /// Returns zero for `ColorType::Unknown`.
    ///
    /// Returns one of: 0, 1, 2, 3; left shift to convert pixels to bytes.
    #[must_use]
    pub const fn shift_per_pixel(&self) -> i32 {
        self.info.shift_per_pixel()
    }

    /// Returns minimum memory required for pixel storage.
    ///
    /// Does not include unused memory on last row when `row_bytes_as_pixels()` exceeds width().
    ///
    /// Returns `usize::MAX` if result does not fit in usize.
    /// Returns zero if height() or width() is 0.
    /// Returns `height() * row_bytes()` if `color_type()` is `ColorType::Unknown`.
    #[must_use]
    pub const fn compute_byte_size(&self) -> usize {
        self.info.compute_byte_size(self.row_bytes)
    }

    /// Returns true if all pixels are opaque.
    ///
    /// `ColorType` determines how pixels are encoded, and whether pixel describes alpha.
    /// Returns true for `ColorType` without alpha in each pixel; for other `ColorType`,
    /// returns true if all pixels have alpha values equivalent to 1.0 or greater.
    ///
    /// - For `ColorType::Rgb565` or `ColorType::Gray8`, always returns true.
    /// - For `ColorType::Alpha8`, `ColorType::Bgra8888`, `ColorType::Rgba8888`, returns true
    /// if all pixel alpha values are 255.
    /// - For `ColorType::Argb4444_`, returns true if all pixel alpha values are 15.
    /// - For `ColorType::RgbaF16`, returns true if all pixel alpha values are 1.0 or greater.
    /// - Returns false for `ColorType::Unknown`.
    ///
    /// Returns true if all pixels have opaque values or `ColorType` is opaque
    #[must_use]
    pub const fn compute_is_opaque(&self) -> bool {
        unimplemented!()
    }

    /// Returns pixel at (x, y) as unpremultiplied color.
    ///
    /// Returns black with alpha if `ColorType` is Alpha8.
    ///
    /// Return None if `ColorType` is Unknown or pixel address is empty.
    ///
    /// `ColorSpace` in `ImageInfo` is ignored. Some color precision may be lost in the
    /// conversion to unpremultiplied color; original pixel data may have additional precision.
    ///
    /// # Parameters
    /// - `x` - column index, zero or greater, and less than width()
    /// - `y` - row index, zero or greater, and less than height()
    ///
    /// Returns pixel converted to unpremultiplied color
    #[must_use]
    pub fn get_color(&self, _x: i32, _y: i32) -> Option<Color> {
        unimplemented!()
    }

    /// Returns pixel at (x, y) as unpremultiplied color as an `Color4F`.
    ///
    /// Returns black with alpha if `ColorType` is Alpha8.
    ///
    /// Return None if `ColorType` is Unknown or pixel address is empty.
    ///
    /// `ColorSpace` in `ImageInfo` is ignored. Some color precision may be lost in the
    /// conversion to unpremultiplied color; original pixel data may have additional
    /// precision, though this is less likely than for `get_color()`.
    /// Rounding errors may occur if the underlying type has lower precision.
    ///
    /// # Parameters
    /// - `x` - column index, zero or greater, and less than width()
    /// - `y` - row index, zero or greater, and less than height()
    ///
    /// Returns pixel converted to unpremultiplied float color
    #[must_use]
    pub fn get_color4f(&self, _x: i32, _y: i32) -> Option<Color4F> {
        unimplemented!()
    }

    /// Look up the pixel at (x,y) and return its alpha component, normalized to [0..1].
    ///
    /// This is roughly equivalent to `get_color().alpha()`, but can be more efficent
    /// (and more precise if the pixels store more than 8 bits per component).
    ///
    /// # Parameters
    /// - `x` - column index, zero or greater, and less than width()
    /// - `y` - row index, zero or greater, and less than height()
    ///
    /// Returns alpha converted to normalized float
    #[must_use]
    pub const fn get_alphaf(&self, _x: i32, _y: i32) -> f32 {
        unimplemented!()
    }

    /// Returns readable pixel address at (x, y).
    ///
    /// Returns None if `PixelRef` is empty.
    ///
    /// Returns None if `ColorType` is Unknown.
    ///
    /// Performs a lookup of pixel size; for better performance, call
    /// one of: addr8, addr16, addr32, addr64, or `addr_f16()`.
    ///
    /// # Parameters
    /// - `x` - column index, zero or greater, and less than width()
    /// - `y` - row index, zero or greater, and less than height()
    #[must_use]
    pub fn pixels_at(&self, x: i32, y: i32) -> Option<&[u8]> {
        let offset = self.info.compute_offset(x, y, self.row_bytes);
        Some(&self.pixels[offset..])
    }

    /// Returns readable base pixel address.
    ///
    /// Result is addressable as unsigned 8-bit bytes.
    /// Return None if `ColorType` is not Alpha8 or Gray8.
    ///
    /// One byte corresponds to one pixel.
    ///
    /// Returns readable unsigned 8-bit pointer to pixels.
    #[must_use]
    pub fn addr8(&self) -> Option<&[u8]> {
        if self.info.bytes_per_pixel() == 1 {
            Some(&self.pixels)
        } else {
            None
        }
    }

    /// Returns readable base pixel address.
    ///
    /// Result is addressable as unsigned 16-bit words.
    /// Return None if `ColorType` is not Rgb565 or Argb4444.
    ///
    /// One word corresponds to one pixel.
    ///
    /// Returns readable unsigned 16-bit pointer to pixels.
    #[must_use]
    pub fn addr16(&self) -> Option<&[u8]> {
        if self.info.bytes_per_pixel() == 2 {
            Some(&self.pixels)
        } else {
            None
        }
    }

    /// Returns readable base pixel address.
    ///
    /// Result is addressable as unsigned 32-bit words.
    /// Return None if `ColorType` is not Rgba8888 or Bgra8888.
    ///
    /// One word corresponds to one pixel.
    ///
    /// Returns readable unsigned 32-bit pointer to pixels.
    #[must_use]
    pub fn addr32(&self) -> Option<&[u8]> {
        if self.info.bytes_per_pixel() == 4 {
            Some(&self.pixels)
        } else {
            None
        }
    }

    /// Returns readable base pixel address.
    ///
    /// Result is addressable as unsigned 64-bit words.
    ///
    /// Return None if `ColorType` is not `RgbaF16`.
    ///
    /// One word corresponds to one pixel.
    ///
    /// Returns readable unsigned 64-bit pointer to pixels.
    #[must_use]
    pub fn addr64(&self) -> Option<&[u8]> {
        if self.info.bytes_per_pixel() == 8 {
            Some(&self.pixels)
        } else {
            None
        }
    }

    /// Returns readable base pixel address.
    ///
    /// Result is addressable as unsigned 16-bit words.
    ///
    /// Return None if `ColorType` is not `RgbaF16`.
    ///
    /// Each word represents one color component encoded as a half float.
    /// Four words correspond to one pixel.
    ///
    /// Returns readable unsigned 16-bit pointer to first component of pixels.
    #[must_use]
    pub fn addr_f16(&self) -> Option<&[u8]> {
        match self.info.color_type() {
            ColorType::RgbaF16 | ColorType::RgbaF16Norm => (),
            _ => return None,
        };

        if self.info.bytes_per_pixel() == 8 {
            return None;
        }
        match self.info.color_type() {
            ColorType::RgbaF16 | ColorType::RgbaF16Norm => Some(&self.pixels),
            _ => None,
        }
    }

    /// Returns readable pixel address at (x, y).
    ///
    /// Input is not validated: out of bounds values of x or y return None.
    ///
    /// Return None if `ColorType` is not Alpha8 or Gray8.
    ///
    /// # Parameters
    /// - `x` - column index, zero or greater, and less than width()
    /// - `y` - row index, zero or greater, and less than height()
    ///
    /// Returns readable unsigned 8-bit pointer to pixel at (x, y)
    #[must_use]
    #[allow(clippy::cast_sign_loss)]
    pub fn addr8_at(&self, x: i32, y: i32) -> Option<&[u8]> {
        match self.info.color_type() {
            ColorType::Alpha8 | ColorType::Gray8 => (),
            _ => return None,
        };

        if x < self.info.width() && y < self.info.height() {
            let offset = (y as usize) * self.row_bytes + (x as usize);
            Some(&self.pixels[offset..])
        } else {
            None
        }
    }

    /// Returns readable pixel address at (x, y).
    ///
    /// Input is not validated.
    ///
    /// Returns None if `ColorType` is not Rgb565 or Argb4444.
    ///
    /// # Parameters
    /// - `x` - column index, zero or greater, and less than width()
    /// - `y` - row index, zero or greater, and less than height()
    ///
    /// Returns readable unsigned 16-bit pointer to pixel at (x, y)
    #[must_use]
    #[allow(clippy::cast_sign_loss)]
    pub fn addr16_at(&self, x: i32, y: i32) -> Option<&[u8]> {
        match self.info.color_type() {
            ColorType::Rgb565 | ColorType::Argb4444 => (),
            _ => return None,
        };
        if x < self.info.width() || y < self.info.height() {
            let offset = (y as usize) * self.row_bytes + ((x as usize) << 1);
            Some(&self.pixels[offset..])
        } else {
            None
        }
    }

    /// Returns readable pixel address at (x, y).
    ///
    ///
    /// Return None if `ColorType` is not Rgba8888 or Bgra8888.
    ///
    /// # Parameters
    /// - `x` - column index, zero or greater, and less than width()
    /// - `y` - row index, zero or greater, and less than height()
    ///
    /// Returns readable unsigned 32-bit pointer to pixel at (x, y)
    #[must_use]
    #[allow(clippy::cast_sign_loss)]
    pub fn addr32_at(&self, x: i32, y: i32) -> Option<&[u8]> {
        match self.info.color_type() {
            ColorType::Rgba8888 | ColorType::Bgra8888 => (),
            _ => return None,
        }
        if x < self.info.width() || y < self.info.height() {
            let offset = (y as usize) * self.row_bytes + ((x as usize) << 2);
            Some(&self.pixels[offset..])
        } else {
            None
        }
    }

    /// Returns readable pixel address at (x, y).
    ///
    /// Returns None if `ColorType` is not `RgbaF16`.
    ///
    /// # Parameters
    /// - `x` - column index, zero or greater, and less than width()
    /// - `y` - row index, zero or greater, and less than height()
    ///
    /// Returns readable unsigned 64-bit pointer to pixel at (x, y)
    #[must_use]
    #[allow(clippy::cast_sign_loss)]
    pub fn addr64_at(&self, x: i32, y: i32) -> Option<&[u8]> {
        match self.info.color_type() {
            ColorType::RgbaF16 => (),
            _ => return None,
        }
        if x < self.info.width() || y < self.info.height() {
            let offset = (y as usize) * self.row_bytes + ((x as usize) << 3);
            Some(&self.pixels[offset..])
        } else {
            None
        }
    }

    /// Returns readable pixel address at (x, y).
    ///
    /// Returns None if `ColorType` is not `RgbaF16`.
    ///
    /// Each unsigned 16-bit word represents one color component encoded as a half float.
    /// Four words correspond to one pixel.
    ///
    /// # Parameters
    /// - `x` - column index, zero or greater, and less than width()
    /// - `y` - row index, zero or greater, and less than height()
    ///
    /// Returns readable unsigned 16-bit pointer to pixel component at (x, y)
    #[must_use]
    #[allow(clippy::cast_sign_loss)]
    pub fn addr_f16_at(&self, x: i32, y: i32) -> Option<&[u8]> {
        match self.info.color_type() {
            ColorType::RgbaF16 | ColorType::RgbaF16Norm => (),
            _ => return None,
        }
        if x < self.info.width() || y < self.info.height() {
            let offset = (y as usize) * self.row_bytes + ((x as usize) << 4);
            Some(&self.pixels[offset..])
        } else {
            None
        }
    }

    /// Returns writable base pixel address.
    pub fn addr_mut(&mut self) -> &mut [u8] {
        &mut self.pixels
    }

    /// Returns writable pixel address at (x, y).
    ///
    /// Returns None if `ColorType` is Unknown.
    ///
    /// # Parameters
    /// - `x` - column index, zero or greater, and less than width()
    /// - `y` - row index, zero or greater, and less than height()
    ///
    /// Returns writable generic pointer to pixel.
    #[allow(clippy::cast_sign_loss)]
    pub fn addr_mut_at(&mut self, x: i32, y: i32) -> Option<&mut [u8]> {
        if self.info.color_type() == ColorType::Unknown {
            return None;
        }
        if x < self.info.width() || y < self.info.height() {
            let offset = (y as usize) * self.row_bytes + (x as usize);
            Some(&mut self.pixels[offset..])
        } else {
            None
        }
    }

    /// Returns writable pixel address at (x, y).
    ///
    /// Result is addressable as unsigned 8-bit bytes.
    /// Return None if `ColorType` is not Alpha8 or Gray8.
    ///
    /// One byte corresponds to one pixel.
    ///
    /// # Parameters
    /// - `x` - column index, zero or greater, and less than width()
    /// - `y` - row index, zero or greater, and less than height()
    ///
    /// Returns writable unsigned 8-bit pointer to pixels.
    #[allow(clippy::cast_sign_loss)]
    pub fn addr8_mut_at(&mut self, x: i32, y: i32) -> Option<&mut [u8]> {
        match self.info.color_type() {
            ColorType::Alpha8 | ColorType::Gray8 => (),
            _ => return None,
        }
        if x < self.info.width() || y < self.info.height() {
            let offset = (y as usize) * self.row_bytes + (x as usize);
            Some(&mut self.pixels[offset..])
        } else {
            None
        }
    }

    /// Returns writable pixel address at (x, y).
    ///
    /// Result is addressable as unsigned 16-bit words.
    ///
    /// Returns None if `ColorType` is not Rgb565 or Argb4444.
    ///
    /// One word corresponds to one pixel.
    ///
    /// # Parameters
    /// - `x` - column index, zero or greater, and less than width()
    /// - `y` - row index, zero or greater, and less than height()
    ///
    /// Returns writable unsigned 16-bit pointer to pixel.
    #[allow(clippy::cast_sign_loss)]
    pub fn addr16_mut_at(&mut self, x: i32, y: i32) -> Option<&mut [u8]> {
        match self.info.color_type() {
            ColorType::Rgb565 | ColorType::Argb4444 => (),
            _ => return None,
        }
        if x < self.info.width() || y < self.info.height() {
            let offset = (y as usize) * self.row_bytes + ((x as usize) << 1);
            Some(&mut self.pixels[offset..])
        } else {
            None
        }
    }

    /// Returns writable pixel address at (x, y).
    ///
    /// Result is addressable as unsigned 32-bit words.
    ///
    /// Returns None if `ColorType` is not Rgba8888 or Bgra8888.
    ///
    /// One word corresponds to one pixel.
    ///
    /// # Parameters
    /// - `x` - column index, zero or greater, and less than width()
    /// - `y` - row index, zero or greater, and less than height()
    ///
    /// Returns writable unsigned 32-bit pointer to pixel.
    #[allow(clippy::cast_sign_loss)]
    pub fn addr32_mut_at(&mut self, x: i32, y: i32) -> Option<&mut [u8]> {
        match self.info.color_type() {
            ColorType::Rgba8888 | ColorType::Bgra8888 => (),
            _ => return None,
        }
        if x < self.info.width() || y < self.info.height() {
            let offset = (y as usize) * self.row_bytes + ((x as usize) << 2);
            Some(&mut self.pixels[offset..])
        } else {
            None
        }
    }

    /// Returns writable pixel address at (x, y).
    ///
    /// Result is addressable as unsigned 64-bit words.
    ///
    /// Returns None if `ColorType` is not `RgbaF16`.
    ///
    /// One word corresponds to one pixel.
    ///
    /// # Parameters
    /// - `x` - column index, zero or greater, and less than width()
    /// - `y` - row index, zero or greater, and less than height()
    ///
    /// Returns writable unsigned 64-bit pointer to pixel.
    #[allow(clippy::cast_sign_loss)]
    pub fn addr64_mut_at(&mut self, x: i32, y: i32) -> Option<&mut [u8]> {
        match self.info.color_type() {
            ColorType::RgbaF16 => (),
            _ => return None,
        }
        if x < self.info.width() || y < self.info.height() {
            let offset = (y as usize) * self.row_bytes + ((x as usize) << 4);
            Some(&mut self.pixels[offset..])
        } else {
            None
        }
    }

    /// Returns writable pixel address at (x, y).
    ///
    /// Result is addressable as unsigned 16-bit words.
    ///
    /// Returns None if `ColorType` is not `RgbaF16`.
    ///
    /// Each word represents one color component encoded as a half float.
    /// Four words correspond to one pixel.
    ///
    /// # Parameters
    /// - `x` - column index, zero or greater, and less than width()
    /// - `y` - row index, zero or greater, and less than height()
    ///
    /// Returns writable unsigned 16-bit pointer to first component of pixel.
    #[allow(clippy::cast_sign_loss)]
    pub fn addr_f16_mut_at(&mut self, x: i32, y: i32) -> Option<&mut [u8]> {
        match self.info.color_type() {
            ColorType::RgbaF16 | ColorType::RgbaF16Norm => (),
            _ => return None,
        }
        if x < self.info.width() || y < self.info.height() {
            let offset = (y as usize) * self.row_bytes + ((x as usize) << 4);
            Some(&mut self.pixels[offset..])
        } else {
            None
        }
    }

    /// Copies a Rect of pixels to `dst_pixels`.
    ///
    /// Copy starts at (0, 0), and does not exceed Pixmap (width(), height()).
    ///
    /// `dst_info` specifies width, height, `ColorType`, `AlphaType`, and `ColorSpace` of destination.
    /// `dst_row_bytes` specifics the gap from one destination row to the next.
    /// Returns true if pixels are copied.
    /// Returns false if `dst_info` address equals nullptr, or `dst_row_bytes` is
    /// less than `dst_info.min_row_bytes()`.
    ///
    /// Pixels are copied only if pixel conversion is possible.
    /// - If Pixmap `color_type()` is Gray8, or Alpha8; `dst_info.color_type()` must match.
    /// - If Pixmap `color_type()` is Gray8, `dst_info.color_space()` must match.
    /// - If Pixmap `alpha_type()` is Opaque, `dst_info.alpha_type()` must match.
    /// - If Pixmap `color_space()` is None, `dst_info.color_space()` must match.
    ///
    /// Returns false if pixel conversion is not possible.
    /// Returns false if Pixmap width() or height() is zero or negative.
    ///
    /// # Parameters
    /// - `dst_info` - destination width, height, `ColorType`, `AlphaType`, `ColorSpace`
    /// - `dst_pixels` - destination pixel storage
    /// - `dst_row_bytes` - destination row length
    ///
    /// Returns true if pixels are copied to `dst_pixels`.
    pub fn read_pixels_with_info(
        &self,
        dst_info: &ImageInfo,
        dst_pixels: &mut [u8],
        dst_row_bytes: usize,
    ) -> bool {
        self.read_pixels_with_info_at(dst_info, dst_pixels, dst_row_bytes, 0, 0)
    }

    /// Copies a Rect of pixels to `dst_pixels`.
    ///
    /// Copy starts at `(src_x, src_y)`, and does not exceed Pixmap (width(), height()).
    ///
    /// `dst_info` specifies width, height, `ColorType`, `AlphaType`, and `ColorSpace` of destination.
    ///
    /// `dst_row_bytes` specifics the gap from one destination row to the next.
    /// Returns true if pixels are copied.
    /// Returns false if `dst_info` address equals nullptr, or `dst_row_bytes` is
    /// less than `dst_info.min_row_bytes()`.
    ///
    /// Pixels are copied only if pixel conversion is possible.
    /// - If Pixmap `color_type()` is Gray8, or Alpha8; `dst_info.color_type()` must match.
    /// - If Pixmap `color_type()` is Gray8, `dst_info.color_space()` must match.
    /// - If Pixmap `alpha_type()` is Opaque, `dst_info.alpha_type()` must match.
    /// - If Pixmap `color_space()` is nullptr, `dst_info.color_space()` must match.
    ///
    /// Returns false if pixel conversion is not possible.
    ///
    /// `src_x` and `src_y` may be negative to copy only top or left of source.
    /// Returns false if Pixmap width() or height() is zero or negative.
    /// Returns false if: `abs(src_x) >= Pixmap width()`, or if `abs(src_y) >= Pixmap height()`.
    ///
    /// # Parameters
    /// - `dst_info` - destination width, height, `ColorType`, `AlphaType`, `ColorSpace`
    /// - `dst_pixels` - destination pixel storage
    /// - `dst_row_bytes` - destination row length
    /// - `src_x` - column index whose absolute value is less than width()
    /// - `src_y` - row index whose absolute value is less than height()
    ///
    /// Returns true if pixels are copied to `dst_pixels`.
    pub fn read_pixels_with_info_at(
        &self,
        _dst_info: &ImageInfo,
        _dst_pixels: &mut [u8],
        _dst_row_bytes: usize,
        _src_x: i32,
        _src_y: i32,
    ) -> bool {
        unimplemented!()
    }

    /// Copies a Rect of pixels to dst.
    ///
    /// Copy starts at `(src_x, src_y)`, and does not exceed Pixmap (width(), height()).
    /// dst specifies width, height, `ColorType`, `AlphaType`, and `ColorSpace` of destination.
    ///
    /// Returns true if pixels are copied.
    /// Returns false if dst address equals nullptr, or `dst.row_bytes()` is less than
    /// dst `ImageInfo::min_row_bytes`.
    ///
    /// Pixels are copied only if pixel conversion is possible.
    /// - If Pixmap `color_type()` is Gray8, or Alpha8; `dst.info().color_type` must match.
    /// - If Pixmap `color_type()` is Gray8, `dst.info().color_space` must match.
    /// - If Pixmap `alpha_type()` is Opaque, `dst.info().alpha_type` must match.
    /// - If Pixmap `color_space()` is nullptr, `dst.info().color_space` must match.
    ///
    /// Returns false if pixel conversion is not possible.
    ///
    /// `src_x` and `src_y` may be negative to copy only top or left of source.
    /// Returns false Pixmap width() or height() is zero or negative.
    /// Returns false if: `abs(srcX) >= Pixmap width()`, or if `abs(srcY) >= Pixmap height()`.
    ///
    /// # Parameters
    /// - `dst` - `ImageInfo` and pixel address to write to
    /// - `src_x` - column index whose absolute value is less than width()
    /// - `src_y` - row index whose absolute value is less than height()
    ///
    /// Returns true if pixels are copied to dst.
    pub fn read_pixels_at(&self, dst: &mut Self, src_x: i32, src_y: i32) -> bool {
        let info = dst.info.clone();
        let row_bytes = dst.row_bytes;
        self.read_pixels_with_info_at(&info, dst.addr_mut(), row_bytes, src_x, src_y)
    }

    /// Copies pixels inside bounds() to dst. dst specifies width, height, `ColorType`,
    /// `AlphaType`, and `ColorSpace` of destination.
    ///
    /// Returns true if pixels are copied.
    /// Returns false if dst address equals nullptr, or `dst.row_bytes()` is less than
    /// dst `ImageInfo::min_row_bytes`.
    ///
    /// Pixels are copied only if pixel conversion is possible.
    /// - If Pixmap `color_type()` is Gray8, or Alpha8; dst `ColorType` must match.
    /// - If Pixmap `color_type()` is Gray8, dst `ColorSpace` must match.
    /// - If Pixmap `alpha_type()` is Opaque, dst `AlphaType` must match.
    /// - If Pixmap `color_space()` is nullptr, dst `ColorSpace` must match.
    ///
    /// Returns false if pixel conversion is not possible.
    ///
    /// Returns false if Pixmap width() or height() is zero or negative.
    ///
    /// # Parameters
    /// - `dst` - `ImageInfo` and pixel address to write to
    ///
    /// Returns true if pixels are copied to dst.
    pub fn read_pixels(&mut self, dst: &mut Self) -> bool {
        let info = dst.info.clone();
        let row_bytes = dst.row_bytes;
        self.read_pixels_with_info_at(&info, dst.addr_mut(), row_bytes, 0, 0)
    }

    /// Copies Bitmap to dst, scaling pixels to fit dst.width() and dst.height(), and
    /// converting pixels to match `dst.color_type()` and `dst.alpha_type()`.
    ///
    /// Returns true if pixels are copied.
    /// Returns false if dst address is nullptr, or `dst.row_bytes()` is
    /// less than dst `ImageInfo::min_row_bytes`.
    ///
    /// Pixels are copied only if pixel conversion is possible.
    /// - If Pixmap `color_type()` is Gray8, or Alpha8; dst `ColorType` must match.
    /// - If Pixmap `color_type()` is Gray8, dst `ColorSpace` must match.
    /// - If Pixmap `alpha_type()` is Opaque, dst `AlphaType` must match.
    /// - If Pixmap `color_space()` is nullptr, dst `ColorSpace` must match.
    ///
    /// Returns false if pixel conversion is not possible.
    /// Returns false if Bitmap width() or height() is zero or negative.
    ///
    /// #Parameters
    /// - `dst` - `ImageInfo` and pixel address to write to
    /// Returns true if pixels are scaled to fit dst.
    pub fn scale_pixels(&self, _dst: &mut Self, _options: &SamplingOptions) -> bool {
        unimplemented!()
    }

    /// Writes color to pixels bounded by subset; returns true on success.
    ///
    /// Returns false if `color_type()` is Unknown, or if subset does not intersect bounds().
    ///
    /// # Parameters
    /// - `color` - `sRGB` unpremultiplied color to write
    /// - `subset` - bounding integer Rect of written pixels
    ///
    /// Returns true if pixels are changed.
    pub fn erase_with_subset(&mut self, _color: Color, _subset: &IRect) -> bool {
        unimplemented!()
    }

    /// Writes color to pixels inside bounds(); returns true on success.
    ///
    /// Returns false if `color_type()` is Unknown, or if bounds() is empty.
    ///
    /// # Parameters
    /// - `color` - `sRGB` unpremultiplied color to write
    ///
    /// Returns true if pixels are changed.
    pub fn erase(&mut self, color: Color) -> bool {
        self.erase_with_subset(color, &self.bounds())
    }

    /// Writes color to pixels bounded by subset; returns true on success.
    ///
    /// if subset is nullptr, writes colors pixels inside bounds().
    /// Returns false if `color_type()` is Unknown, if subset is not nullptr and
    /// does not intersect bounds(), or if subset is nullptr and bounds() is empty.
    ///
    /// # Parameters
    /// - `color` - unpremultiplied color to write
    /// - `subset` - bounding integer Rect of pixels to write; may be nullptr
    ///
    /// Returns true if pixels are changed.
    pub fn erase_with_color4f(&mut self, _color: &Color4F, _subset: Option<&IRect>) -> bool {
        unimplemented!()
    }
}
