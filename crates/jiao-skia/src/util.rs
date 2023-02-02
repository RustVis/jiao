// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

#![allow(clippy::cast_possible_truncation)]

use jiao::base::{PointF, RectF};
use skia_safe::Point as SkPoint;
use skia_safe::Rect as SkRect;

/// Convert jiao point into skia point manually.
#[inline]
#[must_use]
pub const fn to_sk_point(p: PointF) -> SkPoint {
    SkPoint::new(p.x() as f32, p.y() as f32)
}

/// Convert jiao rect into skia rect manually.
#[inline]
#[must_use]
pub fn to_sk_rect(r: &RectF) -> SkRect {
    SkRect::new(
        r.left() as f32,
        r.top() as f32,
        r.right() as f32,
        r.bottom() as f32,
    )
}
