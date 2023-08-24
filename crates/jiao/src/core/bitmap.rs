// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

use std::rc::Rc;

use crate::core::alpha_type::AlphaType;
use crate::core::color_space::ColorSpace;
use crate::core::color_type::ColorType;
use crate::core::image_info::ImageInfo;
use crate::core::pixel_ref::PixelRef;
use crate::core::pixmap::Pixmap;

/// Bitmap describes a two-dimensional raster pixel array.
///
/// Bitmap is built on `ImageInfo`, containing integer width and height, `ColorType`
/// and `AlphaType` describing the pixel format, and `ColorSpace` describing
/// the range of colors.
///
/// Bitmap points to `PixelRef`, which describes the physical array of pixels.
/// `ImageInfo` bounds may be located anywhere fully inside `PixelRef` bounds.
///
/// Bitmap can be drawn using Canvas. Bitmap can be a drawing destination for Canvas
/// draw member functions. Bitmap flexibility as a pixel container limits some
/// optimizations available to the target platform.
///
/// If pixel array is primarily read-only, use `Image` for better performance.
/// If pixel array is primarily written to, use `Surface` for better performance.
///
/// Declaring `Bitmap` const prevents altering `ImageInfo`: the Bitmap height, width,
/// and so on cannot change.
/// It does not affect `PixelRef`: a caller may write its pixels.
/// Declaring Bitmap const affects Bitmap configuration, not its contents.
///
/// Bitmap is not thread safe. Each thread must have its own copy of Bitmap fields,
/// although threads may share the underlying pixel array.
pub struct Bitmap {
    pixel_ref: Option<PixelRef>,
    pixmap: Pixmap,
    mips: Mipmap,
}

impl Bitmap {
    /// Creates an empty Bitmap without pixels, with `ColorType::Unknown`,
    /// `AlphaType::Unknown`, and with a width and height of zero.
    ///
    /// `PixelRef` origin is set to (0, 0).
    ///
    /// Use `set_info()` to associate `ColorType`, `AlphaType`, width, and height
    /// after Bitmap has been created.
    #[must_use]
    pub fn new() -> Self {
        unimplemented!()
    }

    /// Swaps the fields of the two bitmaps.
    ///
    /// # Parameters
    /// - `other` - Bitmap exchanged with original
    pub fn swap(&mut self, _other: &mut Self) {
        unimplemented!()
    }

    /// Returns a constant reference to the Pixmap holding the Bitmap pixel
    /// address, row bytes, and ImageInfo.
    ///
    /// Returns reference to SkPixmap describing this SkBitmap.
    #[must_use]
    pub const fn pixmap(&self) -> &Pixmap {
        &self.pixmap
    }

    /// Returns width, height, `AlphaType`, `ColorType`, and `ColorSpace`.
    ///
    /// Returns reference to `ImageInfo`
    #[must_use]
    pub const fn info(&self) -> &ImageInfo {
        self.pixmap.info()
    }

    /// Returns pixel count in each row.
    ///
    /// Should be equal or less than `row_bytes() / info().bytes_per_pixel()`.
    ///
    /// May be less than `pixel_ref().width()`.
    /// Will not exceed `pixel_ref().width() - pixel_ref_origin().x()`.
    ///
    /// Returns pixel width in ImageInfo.
    #[must_use]
    pub const fn width(&self) -> i32 {
        self.pixmap.width()
    }

    /// Returns pixel row count.
    ///
    /// Maybe be less than `pixel_ref().height()`.
    /// `Will not exceed `pixel_ref().height() - pixel_ref_origin().y()`.
    ///
    /// Returns pixel height in ImageInfo.
    #[must_use]
    pub const fn height(&self) -> i32 {
        self.pixmap.height()
    }

    #[must_use]
    pub const fn color_type(&self) -> ColorType {
        self.pixmap.color_type()
    }

    #[must_use]
    pub const fn alpha_type(&self) -> AlphaType {
        self.pixmap.alpha_type()
    }

    /// Returns `ColorSpace`, the range of colors, associated with ImageInfo.
    ///
    /// The reference count of `ColorSpace` is unchanged.
    /// The returned ColorSpace is immutable.
    ///
    /// Returns `ColorSpace` in `ImageInfo`, or None.
    pub const fn color_space(&self) -> &Option<ColorSpace> {
        unimplemented!()
    }

    /// Returns smart pointer to ColorSpace, the range of colors, associated with
    /// `ImageInfo`.
    ///
    /// The smart pointer tracks the number of objects sharing this `ColorSpace`
    /// reference so the memory is released when the owners destruct.
    ///
    /// The returned `ColorSpace` is immutable.
    pub fn ref_color_space(&self) -> Rc<ColorSpace> {
        unimplemented!()
    }

    /// Returns number of bytes per pixel required by `ColorType`.
    ///
    /// Returns zero if color type is Unknown.
    ///
    /// Returns bytes in pixel
    #[must_use]
    pub const fn bytes_per_pixel(&self) -> i32 {
        self.pixmap.info().bytes_per_pixel()
    }

    /// Returns number of pixels that fit on row.
    ///
    /// Should be greater than or equal to width().
    ///
    /// Returns maximum pixels per row.
    #[must_use]
    pub const fn row_bytes_as_pixels(&self) -> i32 {
        self.pixmap.row_bytes_as_pixels()
    }

    /// Returns bit shift converting row bytes to row pixels.
    ///
    /// Returns zero for Unknown.
    ///
    /// Returns one of: 0, 1, 2, 3; left shift to convert pixels to bytes.
    #[must_use]
    pub const fn shift_per_pixel(&self) -> i32 {
        self.pixmap.shift_per_pixel()
    }

    /// Returns true if either width() or height() are zero.
    ///
    /// Does not check if `PixelRef` is nullptr; call `draws_nothing() to check width(),
    /// height(), and `PixelRef`.
    ///
    /// Returns true if dimensions do not enclose area.
    #[must_use]
    pub const fn is_empty(&self) -> bool {
        self.pixmap.info().is_empty()
    }

    /// Returns true if `PixelRef` is nullptr.
    ///
    /// Does not check if width() or height() are zero; call `draws_nothing()` to check
    /// width(), height(), and `PixelRef`.
    ///
    /// Returns true if no `PixelRef` is associated
    #[must_use]
    pub const fn is_null(&self) -> bool {
        self.pixel_ref.is_none()
    }

    /// Returns true if width() or height() are zero, or if PixelRef is nullptr.
    ///
    /// If true, Bitmap has no effect when drawn or drawn into.
    ///
    /// Returns true if drawing has no effect
    #[must_use]
    pub const fn draws_nothing(&self) -> bool {
        self.is_empty() || self.is_null()
    }

    /// Returns row bytes, the interval from one pixel row to the next.
    ///
    /// Row bytes is at least as large as: `width() * info().bytes_per_pixel()`.
    ///
    /// Returns zero if `color_type()` is Unknown, or if row bytes supplied to
    /// `set_info()` is not large enough to hold a row of pixels.
    ///
    /// Returns byte length of pixel row.
    #[must_use]
    pub const fn row_bytes(&self) -> usize {
        self.pixmap.row_bytes()
    }

    /// Sets `AlphaType`, if `alpha_type` is compatible with `ColorType`.
    ///
    /// Returns true unless `alpha_type` is Unknown and current `AlphaType`
    /// is not Unknown.
    ///
    /// Returns true if `ColorType` is Unknown. `alpha_type` is ignored, and
    /// `AlphaType` remains Unknown.
    ///
    /// Returns true if `ColorType` is Rgb565 or Gray8.
    /// `alpha_type` is ignored, and `AlphaType` remains Opaque.
    ///
    /// If `ColorType` is Argb4444, Rgba8888, Bgra8888, or RgbaF16: returns true unless
    /// `alpha_type` is Unknown and `AlphaType` is not Unknown.
    /// If `AlphaType` is Unknown, `alpha_type` is ignored.
    ///
    /// If `ColorType` is Alpha8, returns true unless `alpha_type` is Unknown and
    /// `AlphaType` is not Unknown. If `AlphaType` is Unknown, `alpha_type` is ignored.
    /// If `alpha_type` is Unpremul, it is treated as Premul.
    ///
    /// This changes `AlphaType` in `PixelRef`; all bitmaps sharing `PixelRef`
    /// are affected.
    ///
    /// Returns true if `AlphaType` is set
    pub fn set_alpha_type(&mut self, _alpha_type: AlphaType) -> bool {
        unimplemented!()
    }

    /// Returns pixel address, the base address corresponding to the pixel origin.
    ///
    /// Returns pixel address.
    pub fn get_pixels(&self) -> &[u8] {
        self.pixmap.pixels()
    }

    /// Returns minimum memory required for pixel storage.
    ///
    /// - Does not include unused memory on last row when `row_bytes_as_pixels()` exceeds width().
    /// - Returns `usize::MAX` if result does not fit in usize.
    /// - Returns zero if height() or width() is 0.
    /// - Returns `height() times row_bytes()` if `color_type()` is Unknown.
    ///
    /// Returns size in bytes of image buffer.
    #[must_use]
    pub const fn compute_byte_size(&self) -> usize {
        self.pixmap.compute_byte_size()
    }

    /// Returns true if pixels can not change.
    ///
    /// Most immutable Bitmap checks trigger an assert only on debug builds.
    ///
    /// Returns true if pixels are immutable.
    #[must_use]
    pub const fn is_immutable(&self) -> bool {
        unimplemented!()
    }

    /// Sets internal flag to mark Bitmap as immutable.
    ///
    /// Once set, pixels can not change.
    /// Any other bitmap sharing the same PixelRef are also marked as immutable.
    /// Once PixelRef is marked immutable, the setting cannot be cleared.
    ///
    /// Writing to immutable Bitmap pixels triggers an assert on debug builds.
    pub fn set_immutable(&mut self) {
        unimplemented!()
    }

    /// Returns true if `AlphaType` is set to hint that all pixels are opaque; their
    /// alpha value is implicitly or explicitly 1.0.
    ///
    /// If true, and all pixels are not opaque, it may draw incorrectly.
    ///
    /// Does not check if `ColorType` allows alpha, or if any pixel value has
    /// transparency.
    ///
    /// @return  true if `ImageInfo` `AlphaType` is Opaque.
    #[must_use]
    pub const fn is_opaque(&self) -> bool {
        self.alpha_type().is_opaque()
    }

    /// Resets to its initial state; all fields are set to zero, as if Bitmap had
    /// been initialized by `new()`.
    ///
    /// Sets width, height, row bytes to zero; pixel address to nullptr;
    /// `ColorType` to Unknown; and `AlphaType` to Unknown.
    ///
    /// If `PixelRef` is allocated, its reference count is decreased by one,
    /// releasing its memory if Bitmap is the sole owner.
    pub fn reset(&mut self) {
        unimplemented!()
    }

    #[must_use]
    pub fn addr8_at(&self, x: i32, y: i32) -> Option<&[u8]> {
        self.pixmap.addr8_at(x, y)
    }
}
