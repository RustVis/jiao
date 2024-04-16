// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that can be found
// in the LICENSE file.

/// Specifies the current text alignment used when drawing text.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TextAlign {
    /// The text is left-aligned.
    Left,

    /// The text is right-aligned.
    Right,

    /// The text is centered.
    Center,
}
