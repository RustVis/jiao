// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

#![allow(clippy::module_name_repetitions)]

use crate::core::color_filter::ColorFilter;
use crate::core::flattenable::{Flattenable, Type};
use crate::core::irect::IRect;
use crate::core::matrix::Matrix;
use crate::core::rect::Rect;

/// Base class for image filters.
///
/// If one is installed in the paint, then all drawing occurs as usual, but it is
/// as if the drawing happened into an offscreen (before the xfermode is applied).
/// This offscreen bitmap will then be handed to the imagefilter, who in turn
/// creates a new bitmap which is what will finally be drawn to the device
/// (using the original xfermode).
///
/// The local space of image filters matches the local space of the drawn geometry.
/// For instance if there is rotation on the canvas, the blur will be computed
/// along those rotated axes and not in the device space.
/// In order to achieve this result, the actual drawing of the geometry may happen
/// in an unrotated coordinate system so that the filtered image can be computed more easily,
/// and then it will be post transformed to match what would have been produced
/// if the geometry were drawn with the total canvas matrix to begin with.
pub struct ImageFilter {}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum MapDirection {
    Forward,
    Reverse,
}

impl Flattenable for ImageFilter {
    fn get_flattenable_type(&self) -> Type {
        Type::ImageFilter
    }
}

impl ImageFilter {
    /// Map a device-space rect recursively forward or backward through the filter DAG.
    ///
    /// `MapDirection::Forward` is used to determine which pixels of the destination
    /// canvas a source image rect would touch after filtering.
    /// `MapDirection::Reverse` is used to determine which rect of the source image
    /// would be required to fill the given rect (typically, clip bounds).
    /// Used for clipping and temp-buffer allocations, so the result need not be exact,
    /// but should never be smaller than the real answer. The default implementation
    /// recursively unions all input bounds, or returns the source rect if no inputs.
    ///
    /// In Reverse mode, `input_rect` is the device-space bounds of the input pixels.
    /// In Forward mode it should always be null.
    /// If `input_rect` is null in Reverse mode the resulting answer may be incorrect.
    #[must_use]
    pub const fn filter_bounds(
        &self,
        _src: &IRect,
        _ctm: &Matrix,
        _dir: MapDirection,
        _input_rect: Option<&IRect>,
    ) -> IRect {
        unimplemented!()
    }

    /// Returns whether this image filter is a color filter and puts the color filter into the
    /// "filter" parameter if it can.
    ///
    /// Does nothing otherwise.
    /// If this returns false, then the filter is unchanged.
    /// If this returns true, then if filter is not null, it must be set to a ref'd colorfitler
    /// (i.e. it may not be set to NULL).
    #[must_use]
    pub fn is_color_filter_node(&self, _filter: &mut ColorFilter) -> bool {
        unimplemented!()
    }

    /// Returns true (and optionally returns a ref'd filter) if this imagefilter can be completely
    /// replaced by the returned colorfilter. i.e. the two effects will affect drawing in the same
    /// way.
    #[must_use]
    pub fn as_a_color_filter(&self, _filter: &mut ColorFilter) -> bool {
        unimplemented!()
    }

    /// Returns the number of inputs this filter will accept (some inputs can be NULL).
    #[must_use]
    pub fn count_inputs(&self) -> i32 {
        unimplemented!()
    }

    /// Returns the input filter at a given index, or NULL if no input is connected.
    /// The indices used are filter-specific.
    #[must_use]
    pub fn get_input(&self, _index: i32) -> &Self {
        unimplemented!()
    }

    /// Can this filter DAG compute the resulting bounds of an object-space rectangle?
    #[must_use]
    pub fn can_compute_fast_bounds(&self) -> bool {
        unimplemented!()
    }

    /// If this filter can be represented by another filter + a `local_matrix`,
    /// return that filter, else return null.
    #[must_use]
    pub fn with_local_matrix(_matrix: &Matrix) -> Option<Self> {
        unimplemented!()
    }
}

pub trait ImageFilterTrait {
    /// Default impl returns union of all input bounds.
    fn compute_fast_bounds(&self, _bounds: &Rect) -> Rect {
        unimplemented!()
    }
}
