// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that can be found
// in the LICENSE file.

use crate::core::point::Point;
use crate::core::rect::Rect;

pub const MAX_POINTS: usize = 4;
pub const MAX_LIPPED_LINE_SEGMENTS: usize = MAX_POINTS - 1;

/// Clip the line pts[0]...pts[1] against clip, ignoring segments that
/// lie completely above or below the clip.
///
/// For portions to the left or right, turn those into vertical line segments
/// that are aligned to the edge of the clip.
///
/// Return the number of line segments that result, and store the end-points
/// of those segments sequentially in lines as follows:
/// - 1st segment: lines[0]..lines[1]
/// - 2nd segment: lines[1]..lines[2]
/// - 3rd segment: lines[2]..lines[3]
#[must_use]
pub fn clip_line(
    _pts: &[Point; 2],
    _clip: &Rect,
    _lines: &mut [Point; MAX_POINTS],
    _can_cull_to_the_right: bool,
) -> i32 {
    unimplemented!()
}

/// Intersect the line segment against the rect.
///
/// If there is a non-empty resulting segment, return true and set dst[] to that segment.
/// If not, return false and ignore dst[].
///
/// `clip_line()` is specialized for scan-conversion, as it adds vertical
/// segments on the sides to show where the line extended beyond the
/// left or right sides. `intersect_line()` does not.
#[must_use]
pub fn intersect_line(_src: &[Point; 2], _clip: &Rect, _dst: &mut Point) -> bool {
    unimplemented!()
}
