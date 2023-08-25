// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

use crate::core::color::Color;
use crate::core::types::FourByteTag;

#[derive(Debug, Default, Clone, PartialEq)]
pub struct Coordinate {
    pub axis: FourByteTag,
    pub value: f32,
}

#[derive(Debug, Default, Clone, PartialEq)]
pub struct VariationPosition {
    pub coordinates: Vec<Coordinate>,
}

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct Override {
    pub index: i32,
    pub color: Color,
}

/// Specify a palette to use and overrides for palette entries.
///
/// The overriden palette entries will use the associated color.
/// Override pairs with palette entry indices out of range will not be applied.
/// Later override entries override earlier ones.
#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct Palette {
    pub index: i32,

    /// overrides is a list of pairs of palette entry index and color.
    pub overrides: Vec<Override>,
}

/// `FontArguments` represents a set of actual arguments for a font.
#[derive(Debug, Default, Clone, PartialEq)]
pub struct FontArguments {
    collection_index: i32,
    variation_design_position: VariationPosition,
    palette: Palette,
}

impl FontArguments {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Specify the index of the desired font.
    ///
    /// Font formats like ttc, dfont, cff, cid, pfr, t42, t1, and fon may actually be indexed
    /// collections of fonts.
    pub fn set_collection_index(&mut self, collection_index: i32) -> &mut Self {
        self.collection_index = collection_index;
        self
    }

    /// Specify a position in the variation design space.
    ///
    /// Any axis not specified will use the default value.
    /// Any specified axis not actually present in the font will be ignored.
    ///
    /// # Parameters
    /// - `position` not copied. The value must remain valid for life of `FontArguments`.
    pub fn set_variation_design_position(&mut self, position: VariationPosition) -> &mut Self {
        self.variation_design_position.coordinates = position.coordinates;
        self
    }

    #[must_use]
    pub const fn get_collection_index(&self) -> i32 {
        self.collection_index
    }

    #[must_use]
    pub const fn get_variation_design_position(&self) -> &VariationPosition {
        &self.variation_design_position
    }

    pub fn set_palette(&mut self, palette: Palette) -> &mut Self {
        self.palette.index = palette.index;
        self.palette.overrides = palette.overrides;
        self
    }

    #[must_use]
    pub const fn get_palette(&self) -> &Palette {
        &self.palette
    }
}
