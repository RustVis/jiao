// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that can be found
// in the LICENSE file.

//! Types, consts, functions, and macros for colors.

use bitflags::bitflags;
use std::marker::PhantomData;
use std::ops::{Index, IndexMut, Mul};

use crate::core::alpha_type::AlphaType;
use crate::core::color_priv::{get_packed_a32, get_packed_b32, get_packed_g32, get_packed_r32};
use crate::core::scalar::{Scalar, ScalarExt, SCALAR_1};
use crate::core::types::{A32_SHIFT, B32_SHIFT, G32_SHIFT, R32_SHIFT};

/// 8-bit type for an alpha value. 255 is 100% opaque, zero is 100% transparent.
pub type Alpha = u8;

/// Represents fully transparent Alpha value. Alpha ranges from zero,
/// fully transparent; to 255, fully opaque.
pub const ALPHA_TRANSPARENT: Alpha = 0x00;

/// Represents fully opaque Alpha value. Alpha ranges from zero,
/// fully transparent; to 255, fully opaque.
pub const ALPHA_OPAQUE: Alpha = 0xFF;

/// 32-bit ARGB color value, unpremultiplied.
///
/// Color components are always in a known order. This is different from `PMColor`,
/// which has its bytes in a configuration dependent order, to match
/// the format of Bgra8888 bitmaps. Color is the type used to specify colors
/// in Paint and in gradients.
///
/// Color that is premultiplied has the same component values as color
/// that is unpremultiplied if alpha is 255, fully opaque, although may have the
/// component values in a different order.
#[derive(Debug, Default, Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Color {
    alpha: u8,
    red: u8,
    green: u8,
    blue: u8,
}

impl From<u32> for Color {
    #[allow(clippy::cast_possible_truncation)]
    fn from(val: u32) -> Self {
        Self {
            alpha: (val >> 24) as u8,
            red: (val >> 16) as u8,
            green: (val >> 8) as u8,
            blue: val as u8,
        }
    }
}

impl From<Color> for u32 {
    fn from(color: Color) -> Self {
        (Self::from(color.alpha) << 24)
            | (Self::from(color.red) << 16)
            | (Self::from(color.green) << 8)
            | Self::from(color.blue)
    }
}

impl Color {
    /// Returns color value from 8-bit component values.
    ///
    /// Since color is unpremultiplied, `alpha` may be smaller than the largest of `red`, `green`, and `blue`.
    ///
    /// # Arguments
    /// - `alpha` - amount of alpha, from fully transparent (0) to fully opaque (255)
    /// - `red` - amount of red, from no red (0) to full red (255)
    /// - `green` - amount of green, from no green (0) to full green (255)
    /// - `blue` - amount of blue, from no blue (0) to full blue (255)
    #[must_use]
    #[inline]
    pub const fn from_argb(alpha: u8, red: u8, green: u8, blue: u8) -> Self {
        Self {
            alpha,
            red,
            green,
            blue,
        }
    }

    /// Returns color value from 8-bit component values, with alpha set fully opaque to 255.
    #[must_use]
    #[inline]
    pub const fn from_rgb(red: u8, green: u8, blue: u8) -> Self {
        Self::from_argb(0xFF, red, green, blue)
    }

    /// Returns alpha byte from color value.
    #[must_use]
    #[inline]
    pub const fn alpha(self) -> u8 {
        self.alpha
    }

    /// Returns red component of color, from zero to 255.
    #[must_use]
    #[inline]
    pub const fn red(self) -> u8 {
        self.red
    }

    /// Returns green component of color, from zero to 255.
    #[must_use]
    #[inline]
    pub const fn green(self) -> u8 {
        self.green
    }

    /// Returns blue component of color, from zero to 255.
    #[must_use]
    #[inline]
    pub const fn blue(self) -> u8 {
        self.blue
    }

    /// Returns unpremultiplied color with red, blue, and green set from self; and alpha set
    /// from `alpha`.
    ///
    /// Alpha component of self is ignored and is replaced by `alpha`.

    pub fn set_alpha(&mut self, alpha: u8) {
        self.alpha = alpha;
    }
}

/// Represents fully transparent Color.
///
/// May be used to initialize a destination containing a mask or a non-rectangular image.
pub const COLOR_TRANSPARENT: Color = Color::from_argb(0x00, 0x00, 0x00, 0x00);

/// Represents fully opaque black.
pub const COLOR_BLACK: Color = Color::from_argb(0xFF, 0x00, 0x00, 0x00);

/// Represents fully opaque dark gray.
///
/// Note that SVG dark gray is equivalent to 0xFFA9A9A9.
pub const COLOR_DKGRAY: Color = Color::from_argb(0xFF, 0x44, 0x44, 0x44);

/// Represents fully opaque gray.
///
/// Note that HTML gray is equivalent to 0xFF808080.
pub const COLOR_GRAY: Color = Color::from_argb(0xFF, 0x88, 0x88, 0x88);

/// Represents fully opaque light gray. HTML silver is equivalent to 0xFFC0C0C0.
///
/// Note that SVG light gray is equivalent to 0xFFD3D3D3.
pub const COLOR_LTGRAY: Color = Color::from_argb(0xFF, 0xCC, 0xCC, 0xCC);

/// Represents fully opaque white.
pub const COLOR_WHITE: Color = Color::from_argb(0xFF, 0xFF, 0xFF, 0xFF);

/// Represents fully opaque red.
pub const COLOR_RED: Color = Color::from_argb(0xFF, 0xFF, 0x00, 0x00);

/// Represents fully opaque green. HTML lime is equivalent.
///
/// Note that HTML green is equivalent to 0xFF008000.
pub const COLOR_GREEN: Color = Color::from_argb(0xFF, 0x00, 0xFF, 0x00);

/// Represents fully opaque blue.
pub const COLOR_BLUE: Color = Color::from_argb(0xFF, 0x00, 0x00, 0xFF);

/// Represents fully opaque yellow.
pub const COLOR_YELLOW: Color = Color::from_argb(0xFF, 0xFF, 0xFF, 0x00);

/// Represents fully opaque cyan. HTML aqua is equivalent.
pub const COLOR_CYAN: Color = Color::from_argb(0xFF, 0x00, 0xFF, 0xFF);

/// Represents fully opaque magenta. HTML fuchsia is equivalent.
pub const COLOR_MAGENTA: Color = Color::from_argb(0xFF, 0xFF, 0x00, 0xFF);

#[derive(Debug, Default, Clone, PartialEq)]
pub struct Hsv {
    /// From zero to less than 360
    pub hue: Scalar,
    /// From zero to one.
    pub saturation: Scalar,
    /// From zero to one.
    pub value: Scalar,
}

#[must_use]
#[inline]
fn byte_to_scalar(x: u8) -> Scalar {
    f32::from(x) / 255.0
}

#[must_use]
#[inline]
fn byte_div_to_scalar(numer: u8, denom: u8) -> Scalar {
    // cast to keep the answer signed
    f32::from(numer) / f32::from(denom)
}

/// Converts RGB to its HSV components.
///
/// Alpha value is dropped.
impl From<Color> for Hsv {
    #[allow(clippy::many_single_char_names)]
    fn from(color: Color) -> Self {
        let r: u8 = color.red;
        let g: u8 = color.green;
        let b: u8 = color.blue;
        let min: u8 = r.min(g.min(b));
        let max: u8 = r.max(g.max(b));
        let delta: u8 = max - min;
        let v: Scalar = byte_to_scalar(max);
        debug_assert!((0.0..=1.0).contains(&v));

        if delta == 0 {
            // we're a shade of gray
            return Self {
                hue: 0.0,
                saturation: 0.0,
                value: v,
            };
        }

        let s: Scalar = byte_div_to_scalar(delta, max);
        debug_assert!((0.0..=1.0).contains(&s));

        let mut h: Scalar = if r == max {
            byte_div_to_scalar(g - b, delta)
        } else if g == max {
            2.0 + byte_div_to_scalar(b - r, delta)
        } else {
            // b == max
            4.0 + byte_div_to_scalar(r - g, delta)
        };

        h *= 60.0;
        if h < 0.0 {
            h += 360.0;
        }
        debug_assert!((0.0..360.0).contains(&h));

        Self {
            hue: h,
            saturation: s,
            value: v,
        }
    }
}

/// Converts HSV components to an ARGB color.
///
/// Alpha is set to 255.
impl From<&Hsv> for Color {
    #[must_use]
    #[inline]
    fn from(hsv: &Hsv) -> Self {
        hsv.to_color(0xFF)
    }
}

impl Hsv {
    /// Converts HSV components to an ARGB color.
    ///
    /// Alpha is passed through unchanged.
    #[must_use]
    #[allow(clippy::cast_possible_truncation)]
    #[allow(clippy::cast_sign_loss)]
    #[allow(clippy::many_single_char_names)]
    pub fn to_color(&self, alpha: Alpha) -> Color {
        let s: Scalar = self.saturation.tpin(0.0, 1.0);
        let v: Scalar = self.value.tpin(0.0, 1.0);

        let v_byte: u8 = (v * 255.0).round_to_int() as u8;

        if s.nearly_zero() {
            // shade of gray
            return Color::from_argb(alpha, v_byte, v_byte, v_byte);
        }

        let hx: Scalar = if self.hue < 0.0 || self.hue >= 360.0 {
            0.0
        } else {
            self.hue / 60.0
        };
        let w: Scalar = hx.floor();
        let f: Scalar = hx - w;

        let p: u8 = ((SCALAR_1 - s) * v * 255.0).round_to_int() as u8;
        let q: u8 = (s.mul_add(-f, SCALAR_1) * v * 255.0).round_to_int() as u8;
        let t: u8 = (s.mul_add(-(SCALAR_1 - f), SCALAR_1) * v * 255.0).round_to_int() as u8;

        debug_assert!(w < 6.0);
        let (r, g, b) = match w as u32 {
            0 => (v_byte, t, p),
            1 => (q, v_byte, p),
            2 => (p, v_byte, t),
            3 => (p, q, v_byte),
            4 => (t, p, v_byte),
            _ => (v_byte, p, q),
        };
        Color::from_argb(alpha, r, g, b)
    }
}

/// 32-bit ARGB color value, premultiplied.
///
/// The byte order for this value is configuration dependent, matching
/// the format of BGRA8888 bitmaps. This is different from Color, which is unpremultiplied,
/// and is always in the same byte order.
#[derive(Debug, Default, Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct PMColor {
    /// Amount of alpha, from fully transparent (0) to fully opaque (255)
    alpha: u8,
    /// Amount of red, from no red (0) to full red (255)
    red: u8,
    /// Amount of green, from no green (0) to full green (255)
    green: u8,
    /// amount of blue, from no blue (0) to full blue (255)
    blue: u8,
}

/// Returns pmcolor closest to Color `color`.
///
/// Multiplies `color` RGB components by the `color` alpha,
/// and arranges the bytes to match the format of `ColorType` `N32`.
impl From<Color> for PMColor {
    fn from(color: Color) -> Self {
        Self::from_argb(color.alpha, color.red, color.green, color.blue)
    }
}

impl From<PMColor> for u32 {
    #[allow(clippy::use_self)]
    fn from(color: PMColor) -> u32 {
        (u32::from(color.alpha) << A32_SHIFT)
            | (u32::from(color.red) << R32_SHIFT)
            | (u32::from(color.green) << G32_SHIFT)
            | (u32::from(color.blue) << B32_SHIFT)
    }
}

impl From<u32> for PMColor {
    fn from(packed: u32) -> Self {
        let alpha = get_packed_a32(packed);
        let red = get_packed_r32(packed);
        let green = get_packed_g32(packed);
        let blue = get_packed_b32(packed);
        Self::from_argb(alpha, red, green, blue)
    }
}

impl PMColor {
    /// Returns a `PMColor` value from unpremultiplied 8-bit component values.
    #[must_use]
    #[inline]
    pub const fn from_argb(alpha: u8, red: u8, green: u8, blue: u8) -> Self {
        Self {
            alpha,
            red,
            green,
            blue,
        }
    }

    #[must_use]
    #[inline]
    pub const fn alpha(&self) -> u8 {
        self.alpha
    }

    #[must_use]
    #[inline]
    pub const fn red(&self) -> u8 {
        self.red
    }

    #[must_use]
    #[inline]
    pub const fn green(&self) -> u8 {
        self.green
    }

    #[must_use]
    #[inline]
    pub const fn blue(&self) -> u8 {
        self.blue
    }
}

/// `ColorChannel` describes different color channels one can manipulate
#[repr(u8)]
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum ColorChannel {
    /// the red channel
    Red,

    /// the green channel
    Green,

    /// the blue channel
    Blue,

    /// the alpha channel
    Alpha,
}

bitflags! {
    /// Used to represent the channels available in a color type or texture format as a mask.
    #[derive(Debug, Default, Clone, Copy, Eq, PartialEq, Hash)]
    pub struct ColorChannelFlag: u32 {
        const Red = 1 << ColorChannel::Red as u8;
        const Green = 1 << ColorChannel::Green as u8;
        const Blue = 1 << ColorChannel::Blue as u8;
        const Alpha = 1 << ColorChannel::Alpha as u8;
        const Gray = 0x10;

        // Convenience values
        const GrayAlpha = Self::Gray.bits() | Self::Alpha.bits();
        const RG      = Self::Red.bits() | Self::Green.bits();
        const RGB = Self::RG.bits() | Self::Blue.bits();
        const RGBA      = Self::RGB.bits() | Self::Alpha.bits();
    }
}

/// `Rgba4F` represents RGBA color value, holding four floating point components.
///
/// Color components are always in a known order.
/// `AlphaType` determines if the `Rgba4F`'s R, G, and B components are premultiplied by alpha or not.
///
/// Crate public API always uses unpremultiplied colors, which can be stored as
/// `Rgba4F<Unpremul>`. For convenience, this type can also be referred to as `Color4f`.
#[derive(Debug, Default, Clone, PartialEq)]
pub struct Rgba4F<AlphaType> {
    red: f32,
    green: f32,
    blue: f32,
    alpha: f32,
    phantom: PhantomData<AlphaType>,
}

/// Returns `Rgba4F` multiplied by scale.
impl Mul<f32> for Rgba4F<AlphaType> {
    type Output = Self;

    fn mul(self, scale: f32) -> Self {
        Self {
            red: self.red * scale,
            green: self.green * scale,
            blue: self.blue * scale,
            alpha: self.alpha * scale,
            phantom: self.phantom,
        }
    }
}

/// Returns `Rgba4F` multiplied component-wise by scale.
impl Mul<&Self> for Rgba4F<AlphaType> {
    type Output = Self;

    fn mul(self, other: &Self) -> Self {
        Self {
            red: self.red * other.red,
            green: self.green * other.green,
            blue: self.blue * other.blue,
            alpha: self.alpha * other.alpha,
            phantom: self.phantom,
        }
    }
}

/// Returns one component.
///
/// Index should not be larger than 3.
impl Index<usize> for Rgba4F<AlphaType> {
    type Output = f32;

    fn index(&self, index: usize) -> &Self::Output {
        debug_assert!(index < 4);
        match index {
            0 => &self.red,
            1 => &self.green,
            2 => &self.blue,
            3 => &self.alpha,
            _ => panic!("Index out of range"),
        }
    }
}

impl IndexMut<usize> for Rgba4F<AlphaType> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        debug_assert!(index < 4);
        match index {
            0 => &mut self.red,
            1 => &mut self.green,
            2 => &mut self.blue,
            3 => &mut self.alpha,
            _ => panic!("Index out of range"),
        }
    }
}

impl Rgba4F<AlphaType> {
    #[must_use]
    pub const fn from_rgba(red: f32, green: f32, blue: f32, alpha: f32) -> Self {
        Self {
            red,
            green,
            blue,
            alpha,
            phantom: PhantomData::<AlphaType>,
        }
    }

    /// Returns a pointer to components of `Rgba4F`, for array access.
    ///
    /// Orders in array is [red, green, blue, alpha]
    #[must_use]
    pub const fn to_vec(&self) -> [f32; 4] {
        [self.red, self.green, self.blue, self.alpha]
    }

    /// Returns true if `Rgba4F` is an opaque color.
    #[must_use]
    pub fn is_opaque(&self) -> bool {
        debug_assert!((0.0..=1.0).contains(&self.alpha));
        self.alpha.fuzzy_equal(1.0)
    }

    /// Returns true if all channels are in [0, 1].
    #[must_use]
    pub fn fits_in_bytes(&self) -> bool {
        let range = 0.0..=1.0;
        debug_assert!(range.contains(&self.alpha));
        range.contains(&self.red) && range.contains(&self.green) && range.contains(&self.blue)
    }

    /// Returns a copy of the `Rgba4F` but with alpha component set to 1.0f.
    #[must_use]
    pub const fn new_opaque(&self) -> Self {
        Self {
            red: self.red,
            green: self.green,
            blue: self.blue,
            alpha: 1.0,
            phantom: self.phantom,
        }
    }
}

// TODO(Shaohua): Replace with partial specialization of Rgba4F<AlphaType>
/// `Color4f` represents RGBA color value, holding four floating point components.
///
/// Color components are always in a known order, and are unpremultiplied.
#[derive(Debug, Default, Clone, PartialEq)]
pub struct Color4f {
    red: f32,
    green: f32,
    blue: f32,
    alpha: f32,
}

/// Returns `Color4f` multiplied by scale.
impl Mul<f32> for Color4f {
    type Output = Self;

    fn mul(self, scale: f32) -> Self {
        Self {
            red: self.red * scale,
            green: self.green * scale,
            blue: self.blue * scale,
            alpha: self.alpha * scale,
        }
    }
}

/// Returns `Color4f` multiplied component-wise by scale.
impl Mul<&Self> for Color4f {
    type Output = Self;

    fn mul(self, other: &Self) -> Self {
        Self {
            red: self.red * other.red,
            green: self.green * other.green,
            blue: self.blue * other.blue,
            alpha: self.alpha * other.alpha,
        }
    }
}

/// Returns one component.
///
/// Index should not be larger than 3.
impl Index<usize> for Color4f {
    type Output = f32;

    fn index(&self, index: usize) -> &Self::Output {
        debug_assert!(index < 4);
        match index {
            0 => &self.red,
            1 => &self.green,
            2 => &self.blue,
            3 => &self.alpha,
            _ => panic!("Index out of range"),
        }
    }
}

impl IndexMut<usize> for Color4f {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        debug_assert!(index < 4);
        match index {
            0 => &mut self.red,
            1 => &mut self.green,
            2 => &mut self.blue,
            3 => &mut self.alpha,
            _ => panic!("Index out of range"),
        }
    }
}

impl From<Color> for Color4f {
    fn from(_color: Color) -> Self {
        unimplemented!()
    }
}

impl From<&Color4f> for Color {
    fn from(_color: &Color4f) -> Self {
        unimplemented!()
    }
}

impl From<Color4f> for Color {
    fn from(_color: Color4f) -> Self {
        unimplemented!()
    }
}

impl Color4f {
    #[must_use]
    pub const fn from_rgba(red: f32, green: f32, blue: f32, alpha: f32) -> Self {
        Self {
            red,
            green,
            blue,
            alpha,
        }
    }

    #[must_use]
    pub const fn red(&self) -> f32 {
        self.red
    }

    #[must_use]
    pub const fn green(&self) -> f32 {
        self.green
    }

    #[must_use]
    pub const fn blue(&self) -> f32 {
        self.blue
    }

    #[must_use]
    pub const fn alpha(&self) -> f32 {
        self.alpha
    }

    pub fn set_alpha(&mut self, alpha: f32) {
        debug_assert!((0.0..=1.0).contains(&alpha));
        self.alpha = alpha;
    }

    /// Returns a pointer to components of `Color4f`, for array access.
    ///
    /// Orders in array is [red, green, blue, alpha]
    #[must_use]
    pub const fn to_vec(&self) -> [f32; 4] {
        [self.red, self.green, self.blue, self.alpha]
    }

    /// Returns true if `Color4f` is an opaque color.
    #[must_use]
    pub fn is_opaque(&self) -> bool {
        debug_assert!((0.0..=1.0).contains(&self.alpha));
        self.alpha.fuzzy_equal(1.0)
    }

    /// Returns true if all channels are in [0, 1].
    #[must_use]
    pub fn fits_in_bytes(&self) -> bool {
        let range = 0.0..=1.0;
        debug_assert!(range.contains(&self.alpha));
        range.contains(&self.red) && range.contains(&self.green) && range.contains(&self.blue)
    }

    /// Returns a copy of the `Color4f` but with alpha component set to 1.0f.
    #[must_use]
    pub const fn new_opaque(&self) -> Self {
        Self {
            red: self.red,
            green: self.green,
            blue: self.blue,
            alpha: 1.0,
        }
    }

    #[must_use]
    pub const fn to_bytes_rgba(&self) -> u32 {
        unimplemented!()
    }

    #[must_use]
    pub fn from_bytes_rgba(_color: u32) -> Self {
        unimplemented!()
    }
}

pub mod colors {
    use super::Color4f;

    pub const TRANSPARENT: Color4f = Color4f::from_rgba(0.0, 0.0, 0.0, 0.0);
    pub const BLACK: Color4f = Color4f::from_rgba(0.0, 0.0, 0.0, 1.0);
    pub const DARK_GRAY: Color4f = Color4f::from_rgba(0.25, 0.25, 0.25, 1.0);
    pub const GRAY: Color4f = Color4f::from_rgba(0.50, 0.50, 0.50, 1.0);
    pub const LIGHT_GRAY: Color4f = Color4f::from_rgba(0.75, 0.75, 0.75, 1.0);
    pub const WHITE: Color4f = Color4f::from_rgba(1.0, 1.0, 1.0, 1.0);
    pub const RED: Color4f = Color4f::from_rgba(1.0, 0.0, 0.0, 1.0);
    pub const GREEN: Color4f = Color4f::from_rgba(0.0, 1.0, 0.0, 1.0);
    pub const BLUE: Color4f = Color4f::from_rgba(0.0, 0.0, 1.0, 1.0);
    pub const YELLOW: Color4f = Color4f::from_rgba(1.0, 1.0, 0.0, 1.0);
    pub const CYAN: Color4f = Color4f::from_rgba(0.0, 1.0, 1.0, 1.0);
    pub const MAGENTA: Color4f = Color4f::from_rgba(1.0, 0.0, 1.0, 1.0);
}
