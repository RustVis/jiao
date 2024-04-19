// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesesr General Public License that can be found
// in the LICENSE file.

use std::mem::size_of;

use crate::core::point::Point;
use crate::core::rect::Rect;
use crate::core::scalar::{Scalar, ScalarExt};

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum Side {
    Left = -1,
    On = 0,
    Right = 1,
}

impl From<i32> for Side {
    #[allow(clippy::comparison_chain)]
    fn from(value: i32) -> Self {
        if value < 0 {
            Self::Left
        } else if value == 0 {
            Self::On
        } else {
            Self::Right
        }
    }
}

#[must_use]
#[inline]
pub fn are_finite(array: &[Point]) -> bool {
    array
        .iter()
        .all(|point| point.x().is_finite() && point.y().is_finite())
}

#[must_use]
pub fn distance_to_line_between_sqd(pt: Point, a: Point, b: Point, side: &mut Side) -> Scalar {
    let u = b - a;
    let v = pt - a;

    let u_length_sqd = u.length_sqd();
    let det = u.cross(&v);
    *side = det.sign_as_int().into();
    let temp = (det / u_length_sqd) * det;
    // It's possible we have a degenerate line vector, or we're so far away it looks degenerate
    // In this case, return squared distance to point A.
    if temp.is_finite() {
        temp
    } else {
        v.length_sqd()
    }
}

#[must_use]
#[inline]
pub fn distance_to_line_between(pt: Point, a: Point, b: Point, side: &mut Side) -> Scalar {
    distance_to_line_between_sqd(pt, a, b, side).sqrt()
}

#[must_use]
pub fn distance_to_line_segment_between_sqd(pt: Point, a: Point, b: Point) -> Scalar {
    // See comments to distanceToLineBetweenSqd. If the projection of c onto
    // u is between a and b then this returns the same result as that
    // function. Otherwise, it returns the distance to the closest of a and
    // b. Let the projection of v onto u be v'.  There are three cases:
    //    1. v' points opposite to u. c is not between a and b and is closer
    //       to a than b.
    //    2. v' points along u and has magnitude less than y. c is between
    //       a and b and the distance to the segment is the same as distance
    //       to the line ab.
    //    3. v' points along u and has greater magnitude than u. c is not
    //       between a and b and is closer to b than a.
    // v' = (u dot v) * u / |u|. So if (u dot v)/|u| is less than zero we're
    // in case 1. If (u dot v)/|u| is > |u| we are in case 3. Otherwise,
    // we're in case 2. We actually compare (u dot v) to 0 and |u|^2 to
    // avoid a sqrt to compute |u|.

    let u = b - a;
    let v = pt - a;

    let u_length_sqd = u.length_sqd();
    let u_dot_v = u.dot(&v);

    // closest point is point A
    if u_dot_v <= 0.0 {
        v.length_sqd()
    } else if u_dot_v > u_length_sqd {
        // closest point is point B
        b.distance_to_sqd(&pt)
    } else {
        // closest point is inside segment
        let det = u.cross(&v);
        let temp = (det / u_length_sqd) * det;
        // It's possible we have a degenerate segment, or we're so far away it looks degenerate
        // In this case, return squared distance to point A.
        if temp.is_finite() {
            temp
        } else {
            v.length_sqd()
        }
    }
}

#[must_use]
#[inline]
pub fn distance_to_line_segment_between(pt: Point, a: Point, b: Point) -> Scalar {
    distance_to_line_segment_between_sqd(pt, a, b).sqrt()
}

#[must_use]
#[inline]
pub fn equals_within_tolerance(pt: Point, p: Point, tol: Scalar) -> bool {
    (pt.x() - p.x()).nearly_zero_tolerance(tol) && (pt.y() - p.y()).nearly_zero_tolerance(tol)
}

impl Point {
    #[must_use]
    #[inline]
    pub fn rotate_ccw(&self) -> Self {
        Self::from_xy(self.y(), -self.x())
    }

    #[must_use]
    #[inline]
    pub fn rotate_cw(&self) -> Self {
        Self::from_xy(-self.y(), self.x())
    }

    pub fn set_length_fast(&mut self, length: f32) -> bool {
        let mut orig_length = 0.0;
        self.set_point_length(self.x(), self.y(), length, &mut orig_length, true)
    }

    #[must_use]
    pub fn make_orthog(&self, side: Side) -> Self {
        debug_assert!(side == Side::Right || side == Side::Left);
        if side == Side::Right {
            Self::from_xy(-self.y(), self.x())
        } else {
            Self::from_xy(self.y(), -self.x())
        }
    }

    #[must_use]
    pub fn distance_to_sqd(&self, a: &Self) -> Scalar {
        let dx = self.x() - a.x();
        let dy = self.y() - a.y();
        dx.mul_add(dx, dy * dy)
    }
}

/// counter-clockwise fan
#[allow(clippy::erasing_op)]
#[allow(clippy::identity_op)]
pub fn set_rect_fan(
    v: &mut [Point],
    left: Scalar,
    top: Scalar,
    right: Scalar,
    bottom: Scalar,
    stride: usize,
) {
    debug_assert!(stride >= size_of::<Point>());

    v[0 * stride].set(left, top);
    v[1 * stride].set(left, bottom);
    v[2 * stride].set(right, bottom);
    v[3 * stride].set(right, top);
}

/// tri strip with two counter-clockwise triangles
#[allow(clippy::erasing_op)]
#[allow(clippy::identity_op)]
pub fn set_rect_tri_strip(
    v: &mut [Point],
    left: Scalar,
    top: Scalar,
    right: Scalar,
    bottom: Scalar,
    stride: usize,
) {
    debug_assert!(stride >= size_of::<Point>());

    v[0 * stride].set(left, top);
    v[1 * stride].set(left, bottom);
    v[2 * stride].set(right, top);
    v[3 * stride].set(right, bottom);
}

pub fn set_rect_tri_strip_with_rect(v: &mut [Point], rect: &Rect, stride: usize) {
    set_rect_tri_strip(
        v,
        rect.left(),
        rect.top(),
        rect.right(),
        rect.bottom(),
        stride,
    );
}
