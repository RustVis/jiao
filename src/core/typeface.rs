// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that can be found
// in the LICENSE file.

use crate::core::rect::Rect;
use crate::core::font_style::FontStyle;

pub type TypefaceId = u32;

/// Machine endian.
pub type FontTableTag = u32;

/// The Typeface class specifies the typeface and intrinsic style of a font.
///
/// This is used in the paint, along with optionally algorithmic settings like
/// `text_size`, `text_skew_x`, `text_scale_x`, `FakeBoldText`, to specify
/// how text appears when drawn (and measured).
///
/// Typeface objects are immutable, and so they can be shared between threads.
#[derive(Debug, Clone, PartialEq)]
pub struct Typeface {
    unique_id: TypefaceId,
    style: FontStyle,
    bounds: Rect,
    is_fixed_pitch: bool,

    trait_impl: Box<dyn Typefacetrait>,
}

/// Style specifies the intrinsic style attributes of a given typeface.
#[repr(u8)]
#[derive(Debug, Default, Clone, Copy, Eq, PartialEq, Hash)]
pub enum Style {
    #[default]
    Normal = 0,
    Bold   = 0x01,
    Italic = 0x02,
    // helpers
    BoldItalic = 0x03,
}

impl Typeface {
    /// Returns the typeface's intrinsic style attributes. */
    #[must_use]
    #[inline]
    pub const fn font_sytle(&self) -> FontStyle {
        self.style
    }

    /// Returns true if style() has the kBold bit set. 
    #[must_use]
    #[inline]
    pub const fn is_bold(&self) -> bool {
        self.style.weight().is_bold()
    }

    /// Returns true if style() has the kItalic bit set.
    #[must_use]
    #[inline]
    pub const fn is_italic(&self) -> bool {
        self.style.slant().is_italic()
    }

    /// Returns true if the typeface claims to be fixed-pitch.
    ///
    /// This is a style bit, advance widths may vary even if this returns true.
    #[must_use]
    #[inline]
    pub const fn is_fixed_pitch(&self) -> bool {
        self.is_fixed_pitch
    }

    /** Copy into 'coordinates' (allocated by the caller) the design variation coordinates.
     *
     *  @param coordinates the buffer into which to write the design variation coordinates.
     *  @param coordinateCount the number of entries available through 'coordinates'.
     *
     *  @return The number of axes, or -1 if there is an error.
     *  If 'coordinates != nullptr' and 'coordinateCount >= numAxes' then 'coordinates' will be
     *  filled with the variation coordinates describing the position of this typeface in design
     *  variation space. It is possible the number of axes can be retrieved but actual position
     *  cannot.
     */
    // TODO(Shaohua):
    int getVariationDesignPosition(SkFontArguments::VariationPosition::Coordinate coordinates[],
                                   int coordinateCount) const;

    /** Copy into 'parameters' (allocated by the caller) the design variation parameters.
     *
     *  @param parameters the buffer into which to write the design variation parameters.
     *  @param coordinateCount the number of entries available through 'parameters'.
     *
     *  @return The number of axes, or -1 if there is an error.
     *  If 'parameters != nullptr' and 'parameterCount >= numAxes' then 'parameters' will be
     *  filled with the variation parameters describing the position of this typeface in design
     *  variation space. It is possible the number of axes can be retrieved but actual parameters
     *  cannot.
     */
    int getVariationDesignParameters(SkFontParameters::Variation::Axis parameters[],
                                     int parameterCount) const;

    /// Return a 32bit value for this typeface, unique for the underlying font data.
    ///
    /// Will never return 0.
    #[must_use]
    #[inline]
    pub const fn unique_id(&self) -> TypefaceId {
        self.unique_id
    }

    /// Return the uniqueID for the specified typeface.
    ///
    /// If the face is null, resolve it to the default font and return its uniqueID.
    /// Will never return 0.
    #[must_use]
    pub const fn new_unique_id() -> TypefaceId {
        unimplemented!()
    }

    /** Creates a new reference to the typeface that most closely matches the
        requested familyName and fontStyle. This method allows extended font
        face specifiers as in the SkFontStyle type. Will never return null.

        @param familyName  May be NULL. The name of the font family.
        @param fontStyle   The style of the typeface.
        @return reference to the closest-matching typeface. Call must call
              unref() when they are done.
    */
    static sk_sp<SkTypeface> MakeFromName(const char familyName[], SkFontStyle fontStyle);

    /** Return a new typeface based on this typeface but parameterized as specified in the
        SkFontArguments. If the SkFontArguments does not supply an argument for a parameter
        in the font then the value from this typeface will be used as the value for that
        argument. If the cloned typeface would be exaclty the same as this typeface then
        this typeface may be ref'ed and returned. May return nullptr on failure.
    */
    sk_sp<SkTypeface> makeClone(const SkFontArguments&) const;

    /**
     *  A typeface can serialize just a descriptor (names, etc.), or it can also include the
     *  actual font data (which can be large). This enum controls how serialize() decides what
     *  to serialize.
     */
    enum class SerializeBehavior {
        kDoIncludeData,
        kDontIncludeData,
        kIncludeDataIfLocal,
    };

    /**
     *  Given an array of UTF32 character codes, return their corresponding glyph IDs.
     *
     *  @param chars pointer to the array of UTF32 chars
     *  @param number of chars and glyphs
     *  @param glyphs returns the corresponding glyph IDs for each character.
     */
    void unicharsToGlyphs(const SkUnichar uni[], int count, SkGlyphID glyphs[]) const;

    int textToGlyphs(const void* text, size_t byteLength, SkTextEncoding encoding,
                     SkGlyphID glyphs[], int maxGlyphCount) const;

    /**
     *  Return the glyphID that corresponds to the specified unicode code-point
     *  (in UTF32 encoding). If the unichar is not supported, returns 0.
     *
     *  This is a short-cut for calling unicharsToGlyphs().
     */
    SkGlyphID unicharToGlyph(SkUnichar unichar) const;

    /**
     *  Return the number of glyphs in the typeface.
     */
    int countGlyphs() const;

    // Table getters -- may fail if the underlying font format is not organized
    // as 4-byte tables.

    /** Return the number of tables in the font. */
    int countTables() const;

    /** Copy into tags[] (allocated by the caller) the list of table tags in
     *  the font, and return the number. This will be the same as CountTables()
     *  or 0 if an error occured. If tags == NULL, this only returns the count
     *  (the same as calling countTables()).
     */
    int getTableTags(SkFontTableTag tags[]) const;

    /** Given a table tag, return the size of its contents, or 0 if not present
     */
    size_t getTableSize(SkFontTableTag) const;

    /** Copy the contents of a table into data (allocated by the caller). Note
     *  that the contents of the table will be in their native endian order
     *  (which for most truetype tables is big endian). If the table tag is
     *  not found, or there is an error copying the data, then 0 is returned.
     *  If this happens, it is possible that some or all of the memory pointed
     *  to by data may have been written to, even though an error has occured.
     *
     *  @param tag  The table tag whose contents are to be copied
     *  @param offset The offset in bytes into the table's contents where the
     *  copy should start from.
     *  @param length The number of bytes, starting at offset, of table data
     *  to copy.
     *  @param data storage address where the table contents are copied to
     *  @return the number of bytes actually copied into data. If offset+length
     *  exceeds the table's size, then only the bytes up to the table's
     *  size are actually copied, and this is the value returned. If
     *  offset > the table's size, or tag is not a valid table,
     *  then 0 is returned.
     */
    size_t getTableData(SkFontTableTag tag, size_t offset, size_t length,
                        void* data) const;

    /**
     *  Return an immutable copy of the requested font table, or nullptr if that table was
     *  not found. This can sometimes be faster than calling getTableData() twice: once to find
     *  the length, and then again to copy the data.
     *
     *  @param tag  The table tag whose contents are to be copied
     *  @return an immutable copy of the table's data, or nullptr.
     */
    sk_sp<SkData> copyTableData(SkFontTableTag tag) const;

    /**
     *  Return the units-per-em value for this typeface, or zero if there is an
     *  error.
     */
    int getUnitsPerEm() const;

    /**
     *  Given a run of glyphs, return the associated horizontal adjustments.
     *  Adjustments are in "design units", which are integers relative to the
     *  typeface's units per em (see getUnitsPerEm).
     *
     *  Some typefaces are known to never support kerning. Calling this method
     *  with all zeros (e.g. getKerningPairAdustments(NULL, 0, NULL)) returns
     *  a boolean indicating if the typeface might support kerning. If it
     *  returns false, then it will always return false (no kerning) for all
     *  possible glyph runs. If it returns true, then it *may* return true for
     *  somne glyph runs.
     *
     *  If count is non-zero, then the glyphs parameter must point to at least
     *  [count] valid glyph IDs, and the adjustments parameter must be
     *  sized to at least [count - 1] entries. If the method returns true, then
     *  [count-1] entries in the adjustments array will be set. If the method
     *  returns false, then no kerning should be applied, and the adjustments
     *  array will be in an undefined state (possibly some values may have been
     *  written, but none of them should be interpreted as valid values).
     */
    bool getKerningPairAdjustments(const SkGlyphID glyphs[], int count,
                                   int32_t adjustments[]) const;

    struct LocalizedString {
        SkString fString;
        SkString fLanguage;
    };
    class LocalizedStrings {
    public:
        LocalizedStrings() = default;
        virtual ~LocalizedStrings() { }
        virtual bool next(LocalizedString* localizedString) = 0;
        void unref() { delete this; }

    private:
        LocalizedStrings(const LocalizedStrings&) = delete;
        LocalizedStrings& operator=(const LocalizedStrings&) = delete;
    };
    /**
     *  Returns an iterator which will attempt to enumerate all of the
     *  family names specified by the font.
     *  It is the caller's responsibility to unref() the returned pointer.
     */
    LocalizedStrings* createFamilyNameIterator() const;

    /**
     *  Return the family name for this typeface. It will always be returned
     *  encoded as UTF8, but the language of the name is whatever the host
     *  platform chooses.
     */
    void getFamilyName(SkString* name) const;

    /**
     *  Return the PostScript name for this typeface.
     *  Value may change based on variation parameters.
     *  Returns false if no PostScript name is available.
     */
    bool getPostScriptName(SkString* name) const;

    /**
     *  Return a stream for the contents of the font data, or NULL on failure.
     *  If ttcIndex is not null, it is set to the TrueTypeCollection index
     *  of this typeface within the stream, or 0 if the stream is not a
     *  collection.
     *  The caller is responsible for deleting the stream.
     */
    std::unique_ptr<SkStreamAsset> openStream(int* ttcIndex) const;

    /**
     * Return a stream for the contents of the font data.
     * Returns nullptr on failure or if the font data isn't already available in stream form.
     * Use when the stream can be used opportunistically but the calling code would prefer
     * to fall back to table access if creating the stream would be expensive.
     * Otherwise acts the same as openStream.
     */
    std::unique_ptr<SkStreamAsset> openExistingStream(int* ttcIndex) const;

    /**
     *  Return a scalercontext for the given descriptor. It may return a
     *  stub scalercontext that will not crash, but will draw nothing.
     */
    std::unique_ptr<SkScalerContext> createScalerContext(const SkScalerContextEffects&,
                                                         const SkDescriptor*) const;

    /**
     *  Return a rectangle (scaled to 1-pt) that represents the union of the bounds of all
     *  of the glyphs, but each one positioned at (0,). This may be conservatively large, and
     *  will not take into account any hinting or other size-specific adjustments.
     */
    SkRect getBounds() const;

    // PRIVATE / EXPERIMENTAL -- do not call
    void filterRec(SkScalerContextRec* rec) const {
        this->onFilterRec(rec);
    }
    // PRIVATE / EXPERIMENTAL -- do not call
    void getFontDescriptor(SkFontDescriptor* desc, bool* isLocal) const {
        this->onGetFontDescriptor(desc, isLocal);
    }
    // PRIVATE / EXPERIMENTAL -- do not call
    void* internal_private_getCTFontRef() const {
        return this->onGetCTFontRef();
    }

    /* Skia reserves all tags that begin with a lower case letter and 0 */
    using FactoryId = SkFourByteTag;
    static void Register(
            FactoryId id,
            sk_sp<SkTypeface> (*make)(std::unique_ptr<SkStreamAsset>, const SkFontArguments&));


    /// Returns true if the typeface's glyph masks may refer to the foreground paint foreground color.
    ///
    /// This is needed to determine caching requirements. Usually true for typefaces
    /// that contain a COLR table.
    fn glyph_mask_needs_current_color(&self) -> bool {
        unimplemented!()
    }

    /// Retrieve detailed typeface metrics.
    ///
    /// Used by the PDF backend.
    #[must_use]
    fn get_advanced_metrics(&self) -> AdvancedTypefaceMetrics {
        unimplemented!()
    }

    #[must_use]
    fn get_default(_style: Style) -> Self {
        unimplemented!()
    }
}

pub trait TypefaceTrait {
    fn from_style(style: &FontStyle, is_fixed_pitch: bool) -> Box<dyn Self>;

    virtual sk_sp<SkTypeface> onMakeClone(const SkFontArguments&) const = 0;

    /** Sets the fixedPitch bit. If used, must be called in the constructor. */
    void setIsFixedPitch(bool isFixedPitch) { fIsFixedPitch = isFixedPitch; }
    /** Sets the font style. If used, must be called in the constructor. */
    void setFontStyle(SkFontStyle style) { fStyle = style; }

    // Must return a valid scaler context. It can not return nullptr.
    virtual std::unique_ptr<SkScalerContext> onCreateScalerContext(const SkScalerContextEffects&,
                                                                   const SkDescriptor*) const = 0;
    virtual void onFilterRec(SkScalerContextRec*) const = 0;
    friend class SkScalerContext;  // onFilterRec

    //  Subclasses *must* override this method to work with the PDF backend.
    virtual std::unique_ptr<SkAdvancedTypefaceMetrics> onGetAdvancedMetrics() const = 0;
    // For type1 postscript fonts only, set the glyph names for each glyph.
    // destination array is non-null, and points to an array of size this->countGlyphs().
    // Backends that do not suport type1 fonts should not override.
    virtual void getPostScriptGlyphNames(SkString*) const = 0;

    // The mapping from glyph to Unicode; array indices are glyph ids.
    // For each glyph, give the default Unicode value, if it exists.
    // dstArray is non-null, and points to an array of size this->countGlyphs().
    virtual void getGlyphToUnicodeMap(SkUnichar* dstArray) const = 0;

    virtual std::unique_ptr<SkStreamAsset> onOpenStream(int* ttcIndex) const = 0;

    virtual std::unique_ptr<SkStreamAsset> onOpenExistingStream(int* ttcIndex) const;

    virtual bool onGlyphMaskNeedsCurrentColor() const = 0;

    virtual int onGetVariationDesignPosition(
        SkFontArguments::VariationPosition::Coordinate coordinates[],
        int coordinateCount) const = 0;

    virtual int onGetVariationDesignParameters(
        SkFontParameters::Variation::Axis parameters[], int parameterCount) const = 0;

    fn on_chars_to_glyphs(chars: &[char], glyphs: &mut [GlyphId]);
    fn on_count_glyphs(&self) -> i32;

    fn on_get_upem(&self) -> i32;
    fn on_get_kerning_pair_adjustments(&self, glyphs: &[GlyphId], count: i32, adjustments: &mut [i32]) -> bool;

    /// Returns the family name of the typeface as known by its font manager.
    ///
    /// This name may or may not be produced by the family name iterator.
    fn on_get_family_name(&self) -> String;
    fn on_get_post_script_name(&self) -> Option<String>;

    /// Returns an iterator over the family names in the font.
    fn on_create_family_name_iterator(&self) -> &[&str];

    fn on_get_table_tags(&self, tags: &mut [FontTableTag]) -> i32;
    fn on_get_table_data(&self, tag: FontTableTag, offset: usize, length: usize, data: &Data) -> usize;
    fn on_copy_table_data(&self, tag: FontTableTag) -> Option<Data>;

    fn on_compute_bounds(&self, rect: &mut Rect) -> bool;

    fn on_get_ct_font_ref(&self) -> Option<()> {
        None
    }
}
