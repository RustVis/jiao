// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

use crate::core::irect::IRect;
use crate::core::point::Point;
use crate::core::rect::Rect;
use crate::core::scalar::Scalar;

/// This struct is initialized with a clip rectangle, and then can be fed cubics,
/// which must already be monotonic in Y.
///
/// In the future, it might return a series of segments, allowing it to clip
/// also in X, to ensure that all segments fit in a finite coordinate system.
#[derive(Debug, Clone, PartialEq)]
pub struct CubicClipper {
    clip: Rect,
}

impl Default for CubicClipper {
    fn default() -> Self {
        Self::new()
    }
}

impl CubicClipper {
    #[must_use]
    pub const fn new() -> Self {
        Self { clip: Rect::new() }
    }

    pub fn set_clip(&mut self, clip: &IRect) {
        self.clip.set_irect(clip);
    }

    #[must_use]
    pub fn clip_cubic(_src: &[Point; 4], _dst: &mut [Point; 4]) -> bool {
        unimplemented!()
    }

    #[must_use]
    pub fn chop_mono_at_y(_pts: &[Point; 4], _y: Scalar, _t: &mut Scalar) -> bool {
        unimplemented!()
    }
}
