// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that can be found
// in the LICENSE file.

#![allow(clippy::module_name_repetitions)]

use bitflags::bitflags;

use crate::core::matrix::Matrix;
use crate::core::path::Path;
use crate::core::point::{Point, Vector};
use crate::core::scalar::Scalar;

#[derive(Debug, Clone)]
pub struct ContourMeasure {
    segments: Vec<Segment>,
    // Points used to define the segments
    points: Vec<Point>,

    length: Scalar,
    is_closed: bool,
}

bitflags! {
    #[derive(Debug, Clone, Copy, Eq, PartialEq)]
    pub struct MatrixFlags: u8 {
        const GetPosition = 0x01;
        const GetTangent = 0x02;
        //const GetPosAndTan = GetPosition | GetTangent;
        const GetPosAndTan = 0x01 | 0x02;
    }
}

#[derive(Debug, Clone)]
struct Segment {
    /// total distance up to this point
    distance: Scalar,

    /// index into the points array
    point_index: usize,

    t_value: [u8; 4],
    // actually the enum SegType
    // TODO(Shaohua): Add SegType.
    type_: u8,
}

impl Segment {
    #[must_use]
    fn get_scalar_t() -> Scalar {
        unimplemented!()
    }

    /*
    fn next(seg: &Self) -> &Self {
        unimplemented!()
        unsigned ptIndex = seg->fPtIndex;
        do {
            ++seg;
        } while (seg->fPtIndex == ptIndex);
        return seg;
    }
    */
}

impl ContourMeasure {
    /// Return the length of the contour.
    #[must_use]
    pub const fn length(&self) -> Scalar {
        self.length
    }

    /// Pins distance to `0 <= distance <= length()`, and then computes the corresponding
    /// position and tangent.
    #[must_use]
    pub fn get_pos_tan(
        &self,
        _distance: Scalar,
        _position: &mut Point,
        _tangent: &mut Vector,
    ) -> bool {
        unimplemented!()
    }

    /// Pins distance to `0 <= distance <= length()`, and then computes the corresponding
    /// matrix (by calling `get_pos_tan()`).
    ///
    /// Returns false if there is no path, or a zero-length path was specified, in which case
    /// matrix is unchanged.
    #[must_use]
    pub fn get_matrix(&self, _distance: Scalar, _matrix: &mut Matrix, _flags: MatrixFlags) -> bool {
        unimplemented!()
    }

    /// Given a start and stop distance, return in dst the intervening segment(s).
    ///
    /// If the segment is zero-length, return false, else return true.
    /// `start_d` and `stop_d` are pinned to legal values (0..getLength()).
    /// If `start_d > stop_d` then return false (and leave dst untouched).
    /// Begin the segment with a moveTo if `start_with_move_to` is true.
    #[must_use]
    pub fn get_segment(
        &self,
        _start_d: Scalar,
        _stop_d: Scalar,
        _dst: &mut Path,
        _start_with_move_to: bool,
    ) -> bool {
        unimplemented!()
    }

    /// Return true if the contour is closed()
    #[must_use]
    pub const fn is_closed(&self) -> bool {
        self.is_closed
    }

    fn distance_to_segment(_distance: Scalar, _target: &mut Scalar) -> &Segment {
        unimplemented!()
    }
}

#[derive(Debug)]
pub struct ContourMeasureIter {
    path: Path,
    force_closed: bool,
    res_scale: Scalar,
}

impl ContourMeasureIter {
    /// Initialize the Iter with a path.
    ///
    /// The parts of the path that are needed are copied, so the client is free to modify/delete
    /// the path after this call.
    ///
    /// `res_scale` controls the precision of the measure. values > 1 increase the
    /// precision (and possibly slow down the computation).
    #[must_use]
    pub const fn new(path: Path, force_closed: bool, res_scale: Scalar) -> Self {
        Self {
            path,
            force_closed,
            res_scale,
        }
    }

    /// Reset the Iter with a path.
    /// The parts of the path that are needed are copied, so the client is free to modify/delete
    /// the path after this call.
    pub fn reset(&mut self, path: Path, force_closed: bool, res_scale: Scalar) {
        self.path = path;
        self.force_closed = force_closed;
        self.res_scale = res_scale;
    }
}

impl Iterator for ContourMeasureIter {
    type Item = ContourMeasure;

    /// Iterates through contours in path, returning a contour-measure object for each contour
    /// in the path. Returns null when it is done.
    ///
    /// This only returns non-zero length contours, where a contour is the segments between
    /// a `Verb::Move` and either:
    /// - the next `Verb::Move`
    /// - `Verb::Close` (1 or more)
    /// - `Verb::Done`
    ///
    /// If it encounters a zero-length contour, it is skipped.
    fn next(&mut self) -> Option<Self::Item> {
        unimplemented!()
    }
}
