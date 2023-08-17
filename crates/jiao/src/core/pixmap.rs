// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

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
/// Use PixelRef to manage pixel memory; SkPixelRef is safe across threads.
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
        self.pixels = pixel.into_vec();
    }

    /// Changes `ColorSpace` in `ImageInfo`; preserves width, height, `AlphaType`, and
    /// `ColorType` in Image, and leaves pixel address and row bytes unchanged.
    pub fn set_color_space(&mut self, _color_space: &ColorSpace) {
        unimplemented!()
    }

    /** Sets subset width, height, pixel address to intersection of SkPixmap with area,
        if intersection is not empty; and return true. Otherwise, leave subset unchanged
        and return false.

        Failing to read the return value generates a compile time warning.

        @param subset  storage for width, height, pixel address of intersection
        @param area    bounds to intersect with SkPixmap
        @return        true if intersection of SkPixmap and area is not empty
    */
    pub const fn extract_subset(&mut self, _subset: &mut Self, _area: &IRect) -> bool {
        unimplemented!()
    }

    /// Returns width, height, `AlphaType`, `ColorType`, and `ColorSpace`.
    #[must_use]
    pub const info(&self) -> &ImageInfo {
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
    pub const fn dimensiions(&self) -> ISize {
        self.info.dimensiions()
    }

    #[must_use]
    pub const color_type(&self) -> ColorType {
        self.info.color_type()
    }

    #[must_use]
    pub const fn alpha_type(&self) -> AlphaType {
        self.info.alpha_type
    }

    /// Returns `ColorSpace`, the range of colors, associated with `ImageInfo`.
    ///
    /// The reference count of ColorSpace is unchanged.
    /// The returned ColorSpace is immutable.
    #[must_use]
    pub const color_space(&self) -> &Option<ColorSpace> {
        self.info.color_space()
    }

    /// Returns true if `AlphaType` is `AlphaType::Opaque`.
    ///
    /// Does not check if `ColorType` allows alpha, or if any pixel value has
    /// transparency.
    #[must_use]
    pub const fn is_opaque(&self) -> bool {
        self.info.is_opaque()
    }

    /// Returns IRect { 0, 0, width(), height() }.
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
    pub const fn row_bytes_as_pixels(&self) -> usize {
        self.row_bytes >> self.shift_per_pixel()
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
    /// ColorType determines how pixels are encoded, and whether pixel describes alpha.
    /// Returns true for ColorType without alpha in each pixel; for other `ColorType`,
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
    /// Returns black with alpha if `ColorType` is `ColorType::Alpha8`.
    ///
    /// Return None if ColorType is `ColorType::Unknown` or pixel address is empty.
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
    /// Returns black with alpha if `ColorType` is `ColorType::Alpha8`.
    ///
    /// Return None if `ColorType` is `ColorType::Unknown` or pixel address is empty.
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
    pub fn get_color4f(&self, _x: i32, _y: i32) -> Option<Color4F> {
        unimplemented!()
    }

    /// Look up the pixel at (x,y) and return its alpha component, normalized to [0..1].
    ///
    /// This is roughly equivalent to `get_color().alpha(), but can be more efficent
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
    /// Returns None if PixelRef is empty.
    ///
    /// Returns None if `ColorType` is Unknown.
    ///
    /// Performs a lookup of pixel size; for better performance, call
    /// one of: addr8, addr16, addr32, addr64, or addrF16().
    ///
    /// # Parameters
    /// - `x` - column index, zero or greater, and less than width()
    /// - `y` - row index, zero or greater, and less than height()
    #[must_use]
    pub const fn pixels_at(&self, x: i32, y: i32) -> Option<&[u8]> {
        let offset = self.info.compute_offset(x, y, self.row_bytes);
        Some(self.pixels[offset..])
    }

    /// Returns readable base pixel address.
    ///
    /// Result is addressable as unsigned 8-bit bytes.
    /// Return None if ColorType is not Alpha8 or Gray8.
    ///
    /// One byte corresponds to one pixel.
    ///
    /// Returns readable unsigned 8-bit pointer to pixels.
    #[must_use]
    pub const fn addr8(&self) -> Option<&[u8]> {
        if self.info.bytes_per_pixel() == 1 {
            Some(&self.pixels)
        } else {
            None
        }
    }

    /// Returns readable base pixel address.
    ///
    /// Result is addressable as unsigned 16-bit words.
    /// Return None if ColorType is not Rgb565 or Argb4444.
    ///
    /// One word corresponds to one pixel.
    ///
    /// Returns readable unsigned 16-bit pointer to pixels.
    #[must_use]
    pub const fn addr16(&self) -> Option<&[u16]> {
        if self.info.bytes_per_pixel() == 2 {
            Some(&self.pixels)
        } else {
            None
        }
    }

    /// Returns readable base pixel address.
    ///
    /// Result is addressable as unsigned 32-bit words.
    /// Return None if ColorType is not Rgba8888 or Bgra8888.
    ///
    /// One word corresponds to one pixel.
    ///
    /// Returns readable unsigned 32-bit pointer to pixels.
    #[must_use]
    pub const fn addr32(&self) -> Option<&[u32]> {
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
    /// Return None if `ColorType` is not RgbaF16
    ///
    /// One word corresponds to one pixel.
    ///
    /// Returns readable unsigned 64-bit pointer to pixels.
    #[must_use]
    pub const fn addr64(&self) -> Option<&[u64]> {
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
    /// Return None if ColorType is not RgbaF16
    ///
    /// Each word represents one color component encoded as a half float.
    /// Four words correspond to one pixel.
    ///
    /// Returns readable unsigned 16-bit pointer to first component of pixels.
    #[must_use]
    pub const fn addr_f16(&self) -> Option<&[u16]> {
        if !self.info.bytes_per_pixel() {
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
    pub const fn addr8_at(&self, x: i32, y: i32) -> Option<&[u8]> {
        if x < self.info.width() && y < self.info.height() {
            let offset = 9y as usize) * self.row_bytes + ((x as usize) << 0));
            &self.pixels[offset..]
        } else {
            None
        }
    }

    /** Returns readable pixel address at (x, y).

        Input is not validated: out of bounds values of x or y trigger an assert() if
        built with SK_DEBUG defined.

        Will trigger an assert() if SkColorType is not kRGB_565_SkColorType or
        kARGB_4444_SkColorType, and is built with SK_DEBUG defined.

        @param x  column index, zero or greater, and less than width()
        @param y  row index, zero or greater, and less than height()
        @return   readable unsigned 16-bit pointer to pixel at (x, y)
    */
    const uint16_t* addr16(int x, int y) const {
        SkASSERT((unsigned)x < (unsigned)fInfo.width());
        SkASSERT((unsigned)y < (unsigned)fInfo.height());
        return (const uint16_t*)((const char*)this->addr16() + (size_t)y * fRowBytes + (x << 1));
    }

    /** Returns readable pixel address at (x, y).

        Input is not validated: out of bounds values of x or y trigger an assert() if
        built with SK_DEBUG defined.

        Will trigger an assert() if SkColorType is not kRGBA_8888_SkColorType or
        kBGRA_8888_SkColorType, and is built with SK_DEBUG defined.

        @param x  column index, zero or greater, and less than width()
        @param y  row index, zero or greater, and less than height()
        @return   readable unsigned 32-bit pointer to pixel at (x, y)
    */
    const uint32_t* addr32(int x, int y) const {
        SkASSERT((unsigned)x < (unsigned)fInfo.width());
        SkASSERT((unsigned)y < (unsigned)fInfo.height());
        return (const uint32_t*)((const char*)this->addr32() + (size_t)y * fRowBytes + (x << 2));
    }

    /** Returns readable pixel address at (x, y).

        Input is not validated: out of bounds values of x or y trigger an assert() if
        built with SK_DEBUG defined.

        Will trigger an assert() if SkColorType is not kRGBA_F16_SkColorType and is built
        with SK_DEBUG defined.

        @param x  column index, zero or greater, and less than width()
        @param y  row index, zero or greater, and less than height()
        @return   readable unsigned 64-bit pointer to pixel at (x, y)
    */
    const uint64_t* addr64(int x, int y) const {
        SkASSERT((unsigned)x < (unsigned)fInfo.width());
        SkASSERT((unsigned)y < (unsigned)fInfo.height());
        return (const uint64_t*)((const char*)this->addr64() + (size_t)y * fRowBytes + (x << 3));
    }

    /** Returns readable pixel address at (x, y).

        Input is not validated: out of bounds values of x or y trigger an assert() if
        built with SK_DEBUG defined.

        Will trigger an assert() if SkColorType is not kRGBA_F16_SkColorType and is built
        with SK_DEBUG defined.

        Each unsigned 16-bit word represents one color component encoded as a half float.
        Four words correspond to one pixel.

        @param x  column index, zero or greater, and less than width()
        @param y  row index, zero or greater, and less than height()
        @return   readable unsigned 16-bit pointer to pixel component at (x, y)
    */
    const uint16_t* addrF16(int x, int y) const {
        SkASSERT(kRGBA_F16_SkColorType     == fInfo.colorType() ||
                 kRGBA_F16Norm_SkColorType == fInfo.colorType());
        return reinterpret_cast<const uint16_t*>(this->addr64(x, y));
    }

    /** Returns writable base pixel address.

        @return  writable generic base pointer to pixels
    */
    void* writable_addr() const { return const_cast<void*>(fPixels); }

    /** Returns writable pixel address at (x, y).

        Input is not validated: out of bounds values of x or y trigger an assert() if
        built with SK_DEBUG defined. Returns zero if SkColorType is kUnknown_SkColorType.

        @param x  column index, zero or greater, and less than width()
        @param y  row index, zero or greater, and less than height()
        @return   writable generic pointer to pixel
    */
    void* writable_addr(int x, int y) const {
        return const_cast<void*>(this->addr(x, y));
    }

    /** Returns writable pixel address at (x, y). Result is addressable as unsigned
        8-bit bytes. Will trigger an assert() if SkColorType is not kAlpha_8_SkColorType
        or kGray_8_SkColorType, and is built with SK_DEBUG defined.

        One byte corresponds to one pixel.

        @param x  column index, zero or greater, and less than width()
        @param y  row index, zero or greater, and less than height()
        @return   writable unsigned 8-bit pointer to pixels
    */
    uint8_t* writable_addr8(int x, int y) const {
        return const_cast<uint8_t*>(this->addr8(x, y));
    }

    /** Returns writable_addr pixel address at (x, y). Result is addressable as unsigned
        16-bit words. Will trigger an assert() if SkColorType is not kRGB_565_SkColorType
        or kARGB_4444_SkColorType, and is built with SK_DEBUG defined.

        One word corresponds to one pixel.

        @param x  column index, zero or greater, and less than width()
        @param y  row index, zero or greater, and less than height()
        @return   writable unsigned 16-bit pointer to pixel
    */
    uint16_t* writable_addr16(int x, int y) const {
        return const_cast<uint16_t*>(this->addr16(x, y));
    }

    /** Returns writable pixel address at (x, y). Result is addressable as unsigned
        32-bit words. Will trigger an assert() if SkColorType is not
        kRGBA_8888_SkColorType or kBGRA_8888_SkColorType, and is built with SK_DEBUG
        defined.

        One word corresponds to one pixel.

        @param x  column index, zero or greater, and less than width()
        @param y  row index, zero or greater, and less than height()
        @return   writable unsigned 32-bit pointer to pixel
    */
    uint32_t* writable_addr32(int x, int y) const {
        return const_cast<uint32_t*>(this->addr32(x, y));
    }

    /** Returns writable pixel address at (x, y). Result is addressable as unsigned
        64-bit words. Will trigger an assert() if SkColorType is not
        kRGBA_F16_SkColorType and is built with SK_DEBUG defined.

        One word corresponds to one pixel.

        @param x  column index, zero or greater, and less than width()
        @param y  row index, zero or greater, and less than height()
        @return   writable unsigned 64-bit pointer to pixel
    */
    uint64_t* writable_addr64(int x, int y) const {
        return const_cast<uint64_t*>(this->addr64(x, y));
    }

    /** Returns writable pixel address at (x, y). Result is addressable as unsigned
        16-bit words. Will trigger an assert() if SkColorType is not
        kRGBA_F16_SkColorType and is built with SK_DEBUG defined.

        Each word represents one color component encoded as a half float.
        Four words correspond to one pixel.

        @param x  column index, zero or greater, and less than width()
        @param y  row index, zero or greater, and less than height()
        @return   writable unsigned 16-bit pointer to first component of pixel
    */
    uint16_t* writable_addrF16(int x, int y) const {
        return reinterpret_cast<uint16_t*>(writable_addr64(x, y));
    }

    /** Copies a SkRect of pixels to dstPixels. Copy starts at (0, 0), and does not
        exceed SkPixmap (width(), height()).

        dstInfo specifies width, height, SkColorType, SkAlphaType, and
        SkColorSpace of destination. dstRowBytes specifics the gap from one destination
        row to the next. Returns true if pixels are copied. Returns false if
        dstInfo address equals nullptr, or dstRowBytes is less than dstInfo.minRowBytes().

        Pixels are copied only if pixel conversion is possible. If SkPixmap colorType() is
        kGray_8_SkColorType, or kAlpha_8_SkColorType; dstInfo.colorType() must match.
        If SkPixmap colorType() is kGray_8_SkColorType, dstInfo.colorSpace() must match.
        If SkPixmap alphaType() is kOpaque_SkAlphaType, dstInfo.alphaType() must
        match. If SkPixmap colorSpace() is nullptr, dstInfo.colorSpace() must match. Returns
        false if pixel conversion is not possible.

        Returns false if SkPixmap width() or height() is zero or negative.

        @param dstInfo      destination width, height, SkColorType, SkAlphaType, SkColorSpace
        @param dstPixels    destination pixel storage
        @param dstRowBytes  destination row length
        @return             true if pixels are copied to dstPixels
    */
    bool readPixels(const SkImageInfo& dstInfo, void* dstPixels, size_t dstRowBytes) const {
        return this->readPixels(dstInfo, dstPixels, dstRowBytes, 0, 0);
    }

    /** Copies a SkRect of pixels to dstPixels. Copy starts at (srcX, srcY), and does not
        exceed SkPixmap (width(), height()).

        dstInfo specifies width, height, SkColorType, SkAlphaType, and
        SkColorSpace of destination. dstRowBytes specifics the gap from one destination
        row to the next. Returns true if pixels are copied. Returns false if
        dstInfo address equals nullptr, or dstRowBytes is less than dstInfo.minRowBytes().

        Pixels are copied only if pixel conversion is possible. If SkPixmap colorType() is
        kGray_8_SkColorType, or kAlpha_8_SkColorType; dstInfo.colorType() must match.
        If SkPixmap colorType() is kGray_8_SkColorType, dstInfo.colorSpace() must match.
        If SkPixmap alphaType() is kOpaque_SkAlphaType, dstInfo.alphaType() must
        match. If SkPixmap colorSpace() is nullptr, dstInfo.colorSpace() must match. Returns
        false if pixel conversion is not possible.

        srcX and srcY may be negative to copy only top or left of source. Returns
        false if SkPixmap width() or height() is zero or negative. Returns false if:
        abs(srcX) >= Pixmap width(), or if abs(srcY) >= Pixmap height().

        @param dstInfo      destination width, height, SkColorType, SkAlphaType, SkColorSpace
        @param dstPixels    destination pixel storage
        @param dstRowBytes  destination row length
        @param srcX         column index whose absolute value is less than width()
        @param srcY         row index whose absolute value is less than height()
        @return             true if pixels are copied to dstPixels
    */
    bool readPixels(const SkImageInfo& dstInfo, void* dstPixels, size_t dstRowBytes, int srcX,
                    int srcY) const;

    /** Copies a SkRect of pixels to dst. Copy starts at (srcX, srcY), and does not
        exceed SkPixmap (width(), height()). dst specifies width, height, SkColorType,
        SkAlphaType, and SkColorSpace of destination.  Returns true if pixels are copied.
        Returns false if dst address equals nullptr, or dst.rowBytes() is less than
        dst SkImageInfo::minRowBytes.

        Pixels are copied only if pixel conversion is possible. If SkPixmap colorType() is
        kGray_8_SkColorType, or kAlpha_8_SkColorType; dst.info().colorType must match.
        If SkPixmap colorType() is kGray_8_SkColorType, dst.info().colorSpace must match.
        If SkPixmap alphaType() is kOpaque_SkAlphaType, dst.info().alphaType must
        match. If SkPixmap colorSpace() is nullptr, dst.info().colorSpace must match. Returns
        false if pixel conversion is not possible.

        srcX and srcY may be negative to copy only top or left of source. Returns
        false SkPixmap width() or height() is zero or negative. Returns false if:
        abs(srcX) >= Pixmap width(), or if abs(srcY) >= Pixmap height().

        @param dst   SkImageInfo and pixel address to write to
        @param srcX  column index whose absolute value is less than width()
        @param srcY  row index whose absolute value is less than height()
        @return      true if pixels are copied to dst
    */
    bool readPixels(const SkPixmap& dst, int srcX, int srcY) const {
        return this->readPixels(dst.info(), dst.writable_addr(), dst.rowBytes(), srcX, srcY);
    }

    /** Copies pixels inside bounds() to dst. dst specifies width, height, SkColorType,
        SkAlphaType, and SkColorSpace of destination.  Returns true if pixels are copied.
        Returns false if dst address equals nullptr, or dst.rowBytes() is less than
        dst SkImageInfo::minRowBytes.

        Pixels are copied only if pixel conversion is possible. If SkPixmap colorType() is
        kGray_8_SkColorType, or kAlpha_8_SkColorType; dst SkColorType must match.
        If SkPixmap colorType() is kGray_8_SkColorType, dst SkColorSpace must match.
        If SkPixmap alphaType() is kOpaque_SkAlphaType, dst SkAlphaType must
        match. If SkPixmap colorSpace() is nullptr, dst SkColorSpace must match. Returns
        false if pixel conversion is not possible.

        Returns false if SkPixmap width() or height() is zero or negative.

        @param dst  SkImageInfo and pixel address to write to
        @return     true if pixels are copied to dst
    */
    bool readPixels(const SkPixmap& dst) const {
        return this->readPixels(dst.info(), dst.writable_addr(), dst.rowBytes(), 0, 0);
    }

    /** Copies SkBitmap to dst, scaling pixels to fit dst.width() and dst.height(), and
        converting pixels to match dst.colorType() and dst.alphaType(). Returns true if
        pixels are copied. Returns false if dst address is nullptr, or dst.rowBytes() is
        less than dst SkImageInfo::minRowBytes.

        Pixels are copied only if pixel conversion is possible. If SkPixmap colorType() is
        kGray_8_SkColorType, or kAlpha_8_SkColorType; dst SkColorType must match.
        If SkPixmap colorType() is kGray_8_SkColorType, dst SkColorSpace must match.
        If SkPixmap alphaType() is kOpaque_SkAlphaType, dst SkAlphaType must
        match. If SkPixmap colorSpace() is nullptr, dst SkColorSpace must match. Returns
        false if pixel conversion is not possible.

        Returns false if SkBitmap width() or height() is zero or negative.

        @param dst            SkImageInfo and pixel address to write to
        @return               true if pixels are scaled to fit dst

        example: https://fiddle.skia.org/c/@Pixmap_scalePixels
    */
    bool scalePixels(const SkPixmap& dst, const SkSamplingOptions&) const;

    /** Writes color to pixels bounded by subset; returns true on success.
        Returns false if colorType() is kUnknown_SkColorType, or if subset does
        not intersect bounds().

        @param color   sRGB unpremultiplied color to write
        @param subset  bounding integer SkRect of written pixels
        @return        true if pixels are changed

        example: https://fiddle.skia.org/c/@Pixmap_erase
    */
    bool erase(SkColor color, const SkIRect& subset) const;

    /** Writes color to pixels inside bounds(); returns true on success.
        Returns false if colorType() is kUnknown_SkColorType, or if bounds()
        is empty.

        @param color  sRGB unpremultiplied color to write
        @return       true if pixels are changed
    */
    bool erase(SkColor color) const { return this->erase(color, this->bounds()); }

    /** Writes color to pixels bounded by subset; returns true on success.
        if subset is nullptr, writes colors pixels inside bounds(). Returns false if
        colorType() is kUnknown_SkColorType, if subset is not nullptr and does
        not intersect bounds(), or if subset is nullptr and bounds() is empty.

        @param color   unpremultiplied color to write
        @param subset  bounding integer SkRect of pixels to write; may be nullptr
        @return        true if pixels are changed
    */
    bool erase(const SkColor4f& color, const SkIRect* subset = nullptr) const;
}
