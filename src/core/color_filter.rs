// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that can be found
// in the LICENSE file.

use std::rc::Rc;

use crate::core::blend_mode::BlendMode;
use crate::core::color::Color;
use crate::core::color::Color4f;
use crate::core::color_space::ColorSpace;
use crate::core::color_table::ColorTable;
use crate::effects::color_matrix::ColorMatrix;

/// `ColorFilters` are optional objects in the drawing pipeline.
///
/// When present in a paint, they are called with the "src" colors, and return new colors,
/// which are then passed onto the next stage (either `ImageFilter` or `Xfermode`).
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
    pub fn as_a_color_mode(&self, _color: &mut Color, _mode: &mut BlendMode) -> bool {
        unimplemented!()
    }

    /// If the filter can be represented by a 5x4 matrix, this returns true,
    /// and sets the matrix appropriately.
    ///
    /// If not, this returns false and ignores the parameter.
    #[must_use]
    pub fn as_a_color_matrix(&self, _matrix: &mut [f32; 20]) -> bool {
        unimplemented!()
    }

    /// Returns true if the filter is guaranteed to never change the alpha of a color it filters.
    #[must_use]
    pub fn is_alpha_unchanged(&self) -> bool {
        unimplemented!()
    }

    #[must_use]
    pub fn filter_color(&self, _color: Color) -> Color {
        unimplemented!()
    }

    /// Converts the src color (in src colorspace), into the dst colorspace,
    /// then applies this filter to it, returning the filtered color in the dst colorspace.
    #[must_use]
    pub fn filter_color4f(
        &self,
        _src_color: &Color4f,
        _src_cs: &ColorSpace,
        _dst_cs: &mut ColorSpace,
    ) -> Color4f {
        unimplemented!()
    }

    /// Construct a colorfilter whose effect is to first apply the inner filter and
    /// then apply this filter, applied to the output of the inner filter.
    /// `result = this(inner(...))`
    #[must_use]
    pub fn from_composed(&self, _inner: &Rc<Self>) -> Rc<Self> {
        unimplemented!()
    }
}

impl ColorFilter {
    #[must_use]
    pub fn compose(outer: &Rc<Self>, inner: &Rc<Self>) -> Rc<Self> {
        outer.from_composed(inner)
    }

    /// Blends between the constant color (src) and input color (dst) based on the `BlendMode`.
    ///
    /// If the color space is null, the constant color is assumed to be defined in `sRGB`.
    #[must_use]
    pub fn blend_with_cs(_color: &Color4f, _cs: &ColorSpace, _mode: BlendMode) -> Rc<Self> {
        unimplemented!()
    }

    #[must_use]
    pub fn blend(_color: Color, _mode: BlendMode) -> Rc<Self> {
        unimplemented!()
    }

    #[must_use]
    pub fn matrix(_matrix: &ColorMatrix) -> Rc<Self> {
        unimplemented!()
    }

    #[must_use]
    pub fn matrix_row_major(_row_major: &[f32; 20]) -> Rc<Self> {
        unimplemented!()
    }

    /// A version of Matrix which operates in HSLA space instead of RGBA.
    /// I.e. HSLA-to-RGBA(Matrix(RGBA-to-HSLA(input))).
    #[must_use]
    pub fn hsla_matrix(_matrix: &ColorMatrix) -> Rc<Self> {
        unimplemented!()
    }

    #[must_use]
    pub fn hsla_matrix_row_major(_row_major: &[f32; 20]) -> Rc<Self> {
        unimplemented!()
    }

    #[must_use]
    pub fn linear_to_srgb_gamma() -> Rc<Self> {
        unimplemented!()
    }

    #[must_use]
    pub fn srgb_to_linear_gamma() -> Rc<Self> {
        unimplemented!()
    }

    #[must_use]
    pub fn lerp(_t: f32, _dst: &Rc<Self>, _src: &Rc<Self>) -> Rc<Self> {
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
    pub fn table_slice(_table: &[u8; 256]) -> Rc<Self> {
        unimplemented!()
    }

    /// Create a table colorfilter, with a different table for each
    /// component [A, R, G, B].
    ///
    /// If a given table is NULL, then it is treated as identity, with the component left unchanged.
    /// If a table is not null, then its contents are copied into the filter.
    #[must_use]
    pub fn table_argb(
        _table_alpha: &[u8; 256],
        _table_red: &[u8; 256],
        _table_green: &[u8; 256],
        _table_blue: &[u8; 256],
    ) -> Rc<Self> {
        unimplemented!()
    }

    /// Create a table colorfilter that holds a ref to the shared color table.
    #[must_use]
    pub fn table(_table: &ColorTable) -> Rc<Self> {
        unimplemented!()
    }

    /// Create a colorfilter that multiplies the RGB channels by one color, and
    /// then adds a second color, pinning the result for each component to
    /// [0..255].
    ///
    /// The alpha components of the mul and add arguments are ignored.
    #[must_use]
    pub fn lighting(_mul: Color, _add: Color) -> Rc<Self> {
        unimplemented!()
    }
}
