// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that can be found
// in the LICENSE file.

use crate::core::canvas::Canvas;
use crate::core::color::Color;
use crate::core::types::GlyphId;

pub trait OpenTypeSvgDecoder {
    /// Each instance probably owns an SVG DOM.
    ///
    /// The instance may be cached so needs to report how much memory it retains.
    fn approximate_size(&self) -> usize;

    fn render(
        canvas: &Canvas,
        upem: i32,
        glhph_id: GlyphId,
        foreground_color: Color,
        palette: &[Color],
    ) -> bool;
}
