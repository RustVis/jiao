// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that can be found
// in the LICENSE file.

use crate::core::matrix::Matrix;
use crate::core::path::Path;
use crate::core::path_effect::PathEffect;
use crate::core::rect::Rect;
use crate::core::scalar::Scalar;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
pub enum Style {
    /// Translate the shape to each position
    Translate,

    /// Rotate the shape about its center
    Rotate,

    /// Transform each point, and turn lines into curves
    Morph,
}

/// Dash by replicating the specified path.
///
/// # parameters
/// - `path` - The path to replicate (dash)
/// - `advance` - The space between instances of path
/// - `phase` - distance (mod advance) along path for its initial position
/// - `style` - how to transform path at each point (based on the current position and tangent)
#[must_use]
pub fn make(_path: &Path, _advance: Scalar, _phase: Scalar, _style: Style) -> Box<D1PathEffect> {
    unimplemented!()
}

pub struct D1PathEffect {}

impl PathEffect for D1PathEffect {
    fn filter_path(&self, _dst: &mut Path, _src: &Path, _stroke_rec: &mut Rect, _cull_r: &Rect) {
        unimplemented!()
    }

    /// Version of `filterPath` that can be called when the CTM is known.
    fn filter_path_with_matrix(
        &self,
        _dst: &mut Path,
        _src: &Path,
        _stroke_rec: &mut Rect,
        _cull_r: &Rect,
        _ctm: &Matrix,
    ) {
        unimplemented!()
    }

    /// True if this path effect requires a valid CTM.
    fn needs_ctm(&self) -> bool {
        unimplemented!()
    }
}
