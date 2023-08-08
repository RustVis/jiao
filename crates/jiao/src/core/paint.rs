// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

//! Paint controls options applied when drawing.
//!
//! Paint collects all options outside of the Canvas clip and Canvas matrix.
//! Various options apply to strokes and fills, and images.
//!
//! Paint collects effects and filters that describe single-pass and multiple-pass
//! algorithms that alter the drawing geometry, color, and transparency.
//! For instance, Paint does not directly implement dashing or blur, but contains
//! the objects that do so.

#![allow(clippy::module_name_repetitions)]

use crate::core::color::{colors::BLACK, Color4F};
use crate::core::color_space::ColorSpace;
use crate::core::font_types::FontHinting;
use crate::core::paint_types::{PaintStyle, StrokeCap, StrokeJoin};
use crate::core::scalar::Scalar;

pub const DEFAULT_TEXT_SIZE: Scalar = 12.0;
pub const DEFAULT_FONT_HINTING: FontHinting = FontHinting::Normal;
pub const DEFAULT_MITER_LIMIT: Scalar = 4.0;

#[derive(Debug, Clone)]
pub struct Paint {
    color_space: Option<ColorSpace>,
    color: Color4F,
    stroke_width: Scalar,
    miter_limit: Scalar,

    anti_alias: bool,
    dither: bool,
    style: PaintStyle,
    cap: StrokeCap,
    join: StrokeJoin,
}

impl Paint {
    /// Constructs Paint with default values.
    #[must_use]
    pub fn new() -> Self {
        Self {
            color_space: None,
            color: BLACK,
            stroke_width: 0.0,
            miter_limit: DEFAULT_MITER_LIMIT,

            anti_alias: false,
            dither: false,
            style: PaintStyle::Fill,
            cap: StrokeCap::default(),
            join: StrokeJoin::default(),
        }
    }

    /// Constructs Paint with default values and the given color.
    #[must_use]
    pub fn from_color(color: &Color4F) -> Self {
        let mut obj = Self::new();
        obj.color = color.clone();
        obj
    }

    #[must_use]
    pub fn from_color_space(color: &Color4F, color_space: &ColorSpace) -> Self {
        let mut obj = Self::new();
        obj.color = color.clone();
        obj.color_space = Some(color_space.clone());
        obj
    }

    /// Sets all Paint contents to their initial values.
    ///
    /// This is equivalent to replacing Paint with the result of `Paint::default()`.
    pub fn reset(&mut self) {
        unimplemented!()
    }

    /// Returns the thickness of the pen used by Paint to outline the shape.
    ///
    /// Returns zero for hairline, greater than zero for pen thickness
    #[must_use]
    pub const fn get_stroke_width(&self) -> Scalar {
        self.stroke_width
    }

    /// Sets the thickness of the pen used by the paint to outline the shape.
    ///
    /// A stroke-width of zero is treated as "hairline" width. Hairlines are always exactly one
    /// pixel wide in device space (their thickness does not change as the canvas is scaled).
    /// Negative stroke-widths are invalid; setting a negative width will have no effect.
    ///
    /// # Parameters
    /// - `width` - zero thickness for hairline; greater than zero for pen thickness
    pub fn set_stroke_width(&mut self, width: Scalar) {
        self.stroke_width = width;
    }

    /// Returns the limit at which a sharp corner is drawn beveled.
    ///
    /// Returns zero and greater miter limit
    #[must_use]
    pub const fn get_stroke_miter(&self) -> Scalar {
        self.miter_limit
    }

    /// Sets the limit at which a sharp corner is drawn beveled.
    ///
    /// Valid values are zero and greater.
    /// Has no effect if miter is less than zero.
    ///
    /// # Parameters
    /// - `miter` - zero and greater miter limit
    pub fn set_stroke_miter(&mut self, miter: Scalar) {
        debug_assert!(miter >= 0.0);
        self.miter_limit = miter;
    }

    /// Returns true if pixels on the active edges of Path may be drawn with partial transparency.
    #[must_use]
    pub const fn is_anti_alias(&self) -> bool {
        self.anti_alias
    }

    /// Requests, but does not require, that edge pixels draw opaque or with partial transparency.
    pub fn set_anti_alias(&mut self, aa: bool) {
        self.anti_alias = aa;
    }

    /// Returns true if color error may be distributed to smooth color transition.
    #[must_use]
    pub const fn is_dither(&self) -> bool {
        self.dither
    }

    /// Requests, but does not require, to distribute color error.
    pub fn set_dither(&mut self, dither: bool) {
        self.dither = dither;
    }

    /// Returns whether the geometry is filled, stroked, or filled and stroked.
    #[must_use]
    pub const fn get_style(&self) -> PaintStyle {
        self.style
    }

    /// Sets whether the geometry is filled, stroked, or filled and stroked.
    ///
    /// Has no effect if style is not a legal `PaintStyle` value.
    pub fn set_style(&mut self, style: PaintStyle) {
        self.style = style;
    }

    /// Returns the geometry drawn at the beginning and end of strokes.
    #[must_use]
    pub const fn get_stroke_cap(&self) -> StrokeCap {
        self.cap
    }

    /// Sets the geometry drawn at the beginning and end of strokes.
    pub fn set_stroke_cap(&mut self, cap: StrokeCap) {
        self.cap = cap;
    }

    /// Returns the geometry drawn at the corners of strokes.
    #[must_use]
    pub const fn get_stroke_join(&self) -> StrokeJoin {
        self.join
    }

    /// Sets the geometry drawn at the corners of strokes.
    pub fn set_stroke_join(&mut self, join: StrokeJoin) {
        self.join = join;
    }
}

impl Default for Paint {
    fn default() -> Self {
        Self::new()
    }
}

// TODO(Shaohua): Impl PartialEq
