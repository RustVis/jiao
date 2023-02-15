// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

#![allow(clippy::cast_possible_truncation)]

use jiao::base::{PointF, RectF};

#[must_use]
pub fn to_sk_point(point: PointF) -> skia_safe::Point {
    skia_safe::Point::from((point.x() as f32, point.y() as f32))
}

#[must_use]
pub fn to_sk_rect(rect: &RectF) -> skia_safe::Rect {
    skia_safe::Rect::new(
        rect.left() as f32,
        rect.top() as f32,
        rect.right() as f32,
        rect.bottom() as f32,
    )
}
