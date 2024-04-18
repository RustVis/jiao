// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that can be found
// in the LICENSE file.

use crate::core::point::Point;
use crate::core::scalar::ScalarExt;

/// Fast evaluation of a cubic ease-in / ease-out curve.
///
/// This is defined as a parametric cubic curve inside the unit square.
///
/// pt[0] is implicitly { 0, 0 }
/// pt[3] is implicitly { 1, 1 }
/// pts[1, 2].x are inside the unit [0..1]
pub struct CubicMap {
    coeff: [Point; 3],
    type_: Type,
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum Type {
    /// x == y
    Line,

    /// At^3 == x
    CubeRoot,

    /// general monotonic cubic solver
    Solver,
}

impl CubicMap {
    #[must_use]
    pub fn new(_p1: Point, _p2: Point) -> Self {
        unimplemented!()
    }

    #[must_use]
    pub fn is_linear(p1: Point, p2: Point) -> bool {
        p1.x().nearly_equal(p1.y()) && p2.x().nearly_equal(p2.y())
    }

    #[must_use]
    pub fn compute_y_from_x(&self, _x: f32) -> f32 {
        unimplemented!()
    }

    #[must_use]
    pub fn compute_from_t(&self, _t: f32) -> Point {
        unimplemented!()
    }
}
