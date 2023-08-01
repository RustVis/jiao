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

use crate::core::color::Color4F;
use crate::core::color_space::ColorSpace;

/// `PaintStyle` set Style to fill, stroke, or both fill and stroke geometry.
///
/// The stroke and fill share all paint attributes; for instance,
/// they are drawn with the same color.
///
/// Use `StrokeAndFill` to avoid hitting the same pixels twice with a stroke draw
/// and a fill draw.
#[repr(u8)]
#[derive(Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum PaintStyle {
    /// Set to fill geometry
    Fill,
    /// Set to stroke geometry
    Stroke,
    /// Sets to stroke and fill geometry
    StrokeAndFill,
}

#[derive(Debug, Clone)]
pub struct Paint {
    color: Color4F,
    color_space: Option<ColorSpace>,
    anti_alias: bool,
    dither: bool,
}

impl Paint {
    #[must_use]
    pub fn new() -> Self {
        Self {
            color: Color4F::default(),
            color_space: None,
            anti_alias: false,
            dither: false,
        }
    }

    /// Constructs Paint with default values and the given color.
    #[must_use]
    pub fn new_with_color(color: &Color4F) -> Self {
        let mut obj = Self::new();
        obj.color = color.clone();
        obj
    }

    #[must_use]
    pub fn new_with_color_space(color: &Color4F, color_space: &ColorSpace) -> Self {
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
}

impl Default for Paint {
    fn default() -> Self {
        Self::new()
    }
}

// TODO(Shaohua): Impl PartialEq
