// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that can be found
// in the LICENSE file.

use crate::core::flattenable::{Flattenable, Type};
use crate::core::matrix::Matrix;
use crate::core::path::Path;
use crate::core::rect::Rect;
use crate::core::scalar::Scalar;

#[repr(u8)]
#[derive(Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
pub enum DashType {
    /// ignores the info parameter
    None,
    /// fills in all of the info parameter
    Dash,
}

#[derive(Debug, Clone, PartialEq)]
pub struct DashInfo {
    /// Length of on/off intervals for dashed lines.
    ///
    /// Even values represent ons, and odds offs.
    ///
    /// Number of intervals in the dash should be even number.
    intervals: Vec<Scalar>,

    /// Offset into the dashed interval pattern mod the sum of all intervals.
    phase: Scalar,
}

impl Default for DashInfo {
    fn default() -> Self {
        Self::new()
    }
}

impl DashInfo {
    #[must_use]
    pub const fn new() -> Self {
        Self {
            intervals: Vec::new(),
            phase: 0.0,
        }
    }

    #[must_use]
    pub const fn from_intervals(intervals: Vec<Scalar>, phase: Scalar) -> Self {
        Self { intervals, phase }
    }

    #[must_use]
    pub const fn get_type(&self) -> DashType {
        unimplemented!()
    }
}

/// `PathEffect` is the trait for objects in the Paint that affect
/// the geometry of a drawing primitive before it is transformed by the
/// canvas' matrix and drawn.
pub trait PathEffect {
    /// Given a src path (input) and a stroke-rec (input and output), apply
    /// this effect to the src path, returning the new path in dst, and return
    /// true. If this effect cannot be applied, return false and ignore dst
    /// and stroke-rec.
    ///
    /// The stroke-rec specifies the initial request for stroking (if any).
    /// The effect can treat this as input only, or it can choose to change
    /// the rec as well. For example, the effect can decide to change the
    /// stroke's width or join, or the effect can change the rec from stroke
    /// to fill (or fill to stroke) in addition to returning a new (dst) path.
    ///
    /// If this method returns true, the caller will apply (as needed) the
    /// resulting stroke-rec to dst and then draw.
    // TODO(Shaohua): Rename to StrokeRec
    fn filter_path(&self, dst: &mut Path, src: &Path, stroke_rec: &mut Rect, cull_r: &Rect);

    /// Version of `filterPath` that can be called when the CTM is known.
    fn filter_path_with_matrix(
        &self,
        dst: &mut Path,
        src: &Path,
        stroke_rec: &mut Rect,
        cull_r: &Rect,
        ctm: &Matrix,
    );

    /// True if this path effect requires a valid CTM.
    fn needs_ctm(&self) -> bool;
}

impl<T: PathEffect> Flattenable for T {
    fn get_flattenable_type(&self) -> Type {
        Type::PathEffect
    }
}

/// Returns a patheffect that apples each effect (first and second) to the original path,
///
/// and returns a path with the sum of these.
///
/// result = first(path) + second(path)
fn make_sum(_first: &dyn PathEffect, _second: &dyn PathEffect) -> Box<dyn PathEffect> {
    unimplemented!()
}

/// Returns a patheffect that applies the inner effect to the path, and then applies the
/// outer effect to the result of the inner's.
///
/// result = outer(inner(path))
fn make_compose(_outer: &dyn PathEffect, _inner: &dyn PathEffect) -> Box<dyn PathEffect> {
    unimplemented!()
}

fn deserialize(_data: &[u8] /* _procs: Option<&DeserialProcs>*/) -> Box<dyn PathEffect> {
    unimplemented!()
}
