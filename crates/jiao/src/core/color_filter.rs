// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

use std::rc::Rc;

use crate::core::blend_mode::BlendMode;
use crate::core::color::Color;
use crate::core::color::Color4F;
use crate::core::color_space::ColorSpace;
use crate::core::color_table::ColorTable;

/// `ColorFilters` are optional objects in the drawing pipeline.
///
/// When present in a paint, they are called with the "src" colors, and return new colors,
/// which are then passed onto the next stage (either ImageFilter or Xfermode).
///
/// All subclasses are required to be reentrant-safe : it must be legal to share
/// the same instance between several threads.
pub struct ColorFilter {}

impl ColorFilter {
    /// If the filter can be represented by a source color plus Mode, this
    /// returns true, and sets (if not NULL) the color and mode appropriately.
    ///
    /// If not, this returns false and ignores the parameters.
    #[must_use]
    pub fn as_a_color_mode(&self, color: &mut Color, mode: &mut BlendMode) -> bool {
        unimplemented!()
    }

    /// If the filter can be represented by a 5x4 matrix, this returns true,
    /// and sets the matrix appropriately.
    ///
    /// If not, this returns false and ignores the parameter.
    #[must_use]
    pub fn as_a_color_matrix(&self, matrix: &mut [f32; 20]) -> bool {
        unimplemented!()
    }

    /// Returns true if the filter is guaranteed to never change the alpha of a color it filters.
    #[must_use]
    pub fn is_alpha_unchanged(&self) -> bool {
        unimplemented!()
    }

    #[must_use]
    pub fn filter_color(&self, color: Color) -> Color {
        unimplemented!()
    }

    /// Converts the src color (in src colorspace), into the dst colorspace,
    /// then applies this filter to it, returning the filtered color in the dst colorspace.
    #[must_use]
    pub fn filterColor4f(
        &self,
        src_color: &Color4F,
        src_cs: &ColorSpace,
        dst_cs: &mut ColorSpace,
    ) -> Color4F {
        unimplemented!()
    }

    /// Construct a colorfilter whose effect is to first apply the inner filter and
    /// then apply this filter, applied to the output of the inner filter.
    /// `result = this(inner(...))`
    #[must_use]
    pub fn from_composed(&self, inner: &Rc<Self>) -> Rc<Self> {
        unimplemented!()
    }
}

impl ColorFilter {
    #[must_use]
    pub fn compose(outer: &Rc<ColorFilter>, inner: &Rc<ColorFilter>) -> Rc<ColorFilter> {
        outer.from_composed(inner)
    }

    /// Blends between the constant color (src) and input color (dst) based on the BlendMode.
    ///
    /// If the color space is null, the constant color is assumed to be defined in sRGB.
    #[must_use]
    pub fn blend_with_cs(color: &Color4F, cs: &ColorSpace, mode: BlendMode) -> Rc<ColorFilter> {
        unimplemented!()
    }

    #[must_use]
    pub fn blend(color: Color, mode: BlendMode) -> Rc<ColorFilter> {
        unimplemented!()
    }

    #[must_use]
    pub fn matrix(matrix: &ColorMatrix) -> Rc<ColorFilter> {
        unimplemented!()
    }

    #[must_use]
    pub fn matrix_row_major(row_major: &[f32; 20]) -> Rc<ColorFilter> {
        unimplemented!()
    }

    /// A version of Matrix which operates in HSLA space instead of RGBA.
    /// I.e. HSLA-to-RGBA(Matrix(RGBA-to-HSLA(input))).
    pub fn hsla_matrix(matrix: &ColorMatrix) -> Rc<Self> {
        unimplemented!()
    }

    #[must_use]
    pub fn hsla_matrix_row_major(row_major: &[f32; 20]) -> Rc<Self> {
        unimplemented!()
    }

    #[must_use]
    pub fn linear_to_srgb_gamma() -> Rc<Self> {
        unimplemented!()
    }

    #[must_use]
    pub fn SRGBToLinearGamma() -> Rc<Self> {
        unimplemented!()
    }

    #[must_use]
    pub fn lerp(t: f32, dst: Rc<Self>, src: Rc<Self>) -> Rc<Self> {
        unimplemented!()
    }

    /// Create a table colorfilter, copying the table into the filter, and
    /// applying it to all 4 components.
    /// - a = table[a];
    /// - r = table[r];
    /// - g = table[g];
    /// - b = table[b];
    /// Components are operated on in unpremultiplied space.
    /// If the incomming colors are premultiplied, they are temporarily unpremultiplied,
    /// then the table is applied, and then the result is remultiplied.
    #[must_use]
    pub fn table_slice(table: &[u8; 256]) -> Rc<Self> {
        unimplemented!()
    }

    /// Create a table colorfilter, with a different table for each
    /// component [A, R, G, B].
    ///
    /// If a given table is NULL, then it is treated as identity, with the component left unchanged.
    /// If a table is not null, then its contents are copied into the filter.
    #[must_use]
    pub fn table_argb(
        table_alpha: &[u8; 256],
        table_red: &[u8; 256],
        table_green: &[u8; 256],
        table_blue: &[u8; 256],
    ) -> Rc<Self> {
        unimplemented!()
    }

    /// Create a table colorfilter that holds a ref to the shared color table.
    pub fn table(table: &ColorTable) -> Rc<ColorFilter> {
        unimplemented!()
    }

    /// Create a colorfilter that multiplies the RGB channels by one color, and
    /// then adds a second color, pinning the result for each component to
    /// [0..255].
    ///
    /// The alpha components of the mul and add arguments are ignored.
    pub fn lighting(mul: Color, add: Color) -> Rc<ColorFilter> {
        unimplemented!()
    }
}
