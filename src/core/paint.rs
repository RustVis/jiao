// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that can be found
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

use crate::core::color::{colors::BLACK, Color, Color4F};
use crate::core::color_space::ColorSpace;
use crate::core::font_types::FontHinting;
use crate::core::paint_types::{PaintStyle, StrokeCap, StrokeJoin};
use crate::core::scalar::Scalar;

pub const DEFAULT_TEXT_SIZE: Scalar = 12.0;
pub const DEFAULT_FONT_HINTING: FontHinting = FontHinting::Normal;
pub const DEFAULT_MITER_LIMIT: Scalar = 4.0;

#[derive(Debug, Clone, PartialEq)]
pub struct Paint {
    color_space: Option<ColorSpace>,
    color4f: Color4F,
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
            color4f: BLACK,
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
        let mut p = Self::new();
        p.color4f = color.clone();
        p
    }

    /// Constructs Paint with default values and the given color.
    ///
    /// Sets alpha and RGB used when stroking and filling.
    /// The color is four floating point values, unpremultiplied.
    /// The color values are interpreted as being in the `color_space`.
    /// If `color_space` is None, then color is assumed to be in the
    /// `sRGB` color space.
    ///
    /// # Parameters
    /// - `color` - unpremultiplied RGBA
    /// - `color_space` - `ColorSpace` describing the encoding of color
    #[must_use]
    pub fn from_color_space(color: &Color4F, color_space: &Option<ColorSpace>) -> Self {
        let mut p = Self::new();
        p.color4f = color.clone();
        p.color_space = color_space.clone();
        p
    }

    /// Sets all Paint contents to their initial values.
    ///
    /// This is equivalent to replacing Paint with the result of `Paint::default()`.
    pub fn reset(&mut self) {
        *self = Self::new();
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
        debug_assert!(width >= 0.0);
        if width >= 0.0 {
            self.stroke_width = width;
        }
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
        if miter >= 0.0 {
            self.miter_limit = miter;
        }
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

    /// Set paint's style to `PaintStyle::Stroke` if true, or `PaintStyle::Fill` if false.
    pub fn set_stroke(&mut self, is_stroke: bool) {
        self.style = if is_stroke {
            PaintStyle::Stroke
        } else {
            PaintStyle::Fill
        };
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

    /// Retrieves alpha and RGB, unpremultiplied, packed into 32 bits.
    ///
    /// Use helpers `get_alpha()`, `get_red()`, `get_green()`, and `get_blue()`
    /// to extract a color component.
    #[must_use]
    pub fn get_color(&self) -> Color {
        (&self.color4f).into()
    }

    /// Retrieves alpha and RGB, unpremultiplied, as four floating point values.
    ///
    /// RGB are extended `sRGB` values (`sRGB` gamut, and encoded with the `sRGB` transfer function).
    #[must_use]
    pub const fn get_color4f(&self) -> &Color4F {
        &self.color4f
    }

    /// Sets alpha and RGB used when stroking and filling.
    ///
    /// The color is a 32-bit value, unpremultiplied, packing 8-bit components
    /// for alpha, red, blue, and green.
    ///
    /// # Parameters
    /// - `color` - unpremultiplied ARGB
    pub fn set_color(&mut self, color: Color) {
        self.color4f = color.into();
    }

    /// Sets alpha and RGB used when stroking and filling.
    ///
    /// The color is four floating point values, unpremultiplied.
    /// The color values are interpreted as being in the `color_space`.
    /// If `color_space` is None , then color is assumed to be in the `sRGB` color space.
    ///
    /// # Parameters
    /// - `color` - unpremultiplied RGBA
    /// - `color_space` - `ColorSpace` describing the encoding of color
    pub fn set_color_space(&mut self, color: &Color4F, color_space: &Option<ColorSpace>) {
        self.color4f = color.clone();
        self.color_space = color_space.clone();
    }

    /// Retrieves alpha from the color used when stroking and filling.
    ///
    /// Returns alpha ranging from zero, fully transparent, to one, fully opaque
    #[must_use]
    pub const fn get_alphaf(&self) -> Scalar {
        self.color4f.alpha()
    }

    /// Helper that scales the alpha by 255.
    #[must_use]
    #[allow(clippy::cast_sign_loss)]
    #[allow(clippy::cast_possible_truncation)]
    pub fn get_alpha(&self) -> u8 {
        (self.color4f.alpha() * 255.0) as u8
    }

    /// Replaces alpha, leaving RGB unchanged.
    ///
    /// alpha is a value from 0.0 to 1.0.
    /// alpha set to zero makes color fully transparent; a set to 1.0 makes color
    /// fully opaque.
    pub fn set_alphaf(&mut self, alpha: f32) {
        self.color4f.set_alpha(alpha);
    }

    /// Helper that accepts an int between 0 and 255, and divides it by 255.0
    #[allow(clippy::cast_lossless)]
    pub fn set_alpha(&mut self, alpha: u8) {
        self.color4f.set_alpha(alpha as f32 / 255.0);
    }

    /// Sets color used when drawing solid fills.
    ///
    /// The color components range from 0 to 255. The color is unpremultiplied;
    /// alpha sets the transparency independent of RGB.
    ///
    /// # Parameters
    /// - `alpha` - amount of alpha, from fully transparent (0) to fully opaque (255)
    /// - `red` - amount of red, from no red (0) to full red (255)
    /// - `green` - amount of green, from no green (0) to full green (255)
    /// - `blue` - amount of blue, from no blue (0) to full blue (255)
    pub fn set_argb(&mut self, alpha: u8, red: u8, green: u8, blue: u8) {
        self.set_color(Color::from_argb(alpha, red, green, blue));
    }
}

impl Default for Paint {
    fn default() -> Self {
        Self::new()
    }
}

// TODO(Shaohua): Impl PartialEq
