// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

use crate::core::blur_types::BlurStyle;
use crate::core::rect::Rect;
use crate::core::scalar::Scalar;

/// `MaskFilter` is the base class for object that perform transformations on
/// the mask before drawing it.
///
/// An example subclass is Blur.
#[derive(Debug, Clone)]
pub struct MaskFilter {
    style: BlurStyle,
    sigma: Scalar,
    respect_ctm: bool,
}

impl MaskFilter {
    /// Create a blur maskfilter.
    ///
    /// # Parameters
    /// - `style` - The `BlurStyle` to use
    /// - `sigma` - Standard deviation of the Gaussian blur to apply. Must be > 0.
    /// - `respect_ctm` - if true the blur's sigma is modified by the CTM.
    ///
    /// Returns the new blur mask filter.
    #[must_use]
    pub const fn new(style: BlurStyle, sigma: Scalar, respect_ctm: bool) -> Self {
        Self {
            style,
            sigma,
            respect_ctm,
        }
    }

    /// Returns the approximate bounds that would result from filtering the src rect.
    ///
    /// The actual result may be different, but it should be contained within the returned bounds.
    #[must_use]
    pub const fn approximate_filtered_bounds(_src: &Rect) -> Rect {
        unimplemented!()
    }
}
