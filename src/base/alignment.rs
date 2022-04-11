// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

/// Specify the text alignment when drawing text.
#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(u8)]
pub enum TextAlignment {
    /// The text is left-aligned.
    Left,

    /// The text is centered.
    Center,

    /// The text is right-aligned.
    Right,

    /// Justifies the text in the available space.
    Justify,

    /// The text is aligned at the normal start of the line (left-aligned for left-to-right locales,
    /// right-aligned for right-to-left locales).
    Start,

    /// The text is aligned at the normal end of the line (right-aligned for left-to-right locales,
    /// left-aligned for right-to-left locales).
    End,
}
