// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

#![allow(clippy::cast_possible_truncation)]

use skia_safe::Rect as SkRect;

use crate::base::RectF;

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
