// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

#[repr(u8)]
#[derive(Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum TextEncoding {
    /// uses bytes to represent UTF-8 or ASCII
    UTF8,

    /// uses two byte words to represent most of Unicode
    UTF16,

    /// uses four byte words to represent all of Unicode
    UTF32,

    /// uses two byte words to represent glyph indices
    GlyphID,
}

impl Default for TextEncoding {
    fn default() -> Self {
        Self::UTF8
    }
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum FontHinting {
    /// glyph outlines unchanged
    None,

    /// minimal modification to improve constrast
    Slight,

    /// glyph outlines modified to improve constrast
    Normal,

    /// modifies glyph outlines for maximum constrast
    Full,
}

impl Default for FontHinting {
    fn default() -> Self {
        Self::Normal
    }
}
