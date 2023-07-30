// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by BSD-style License that can be found
// in the LICENSE file.

#![allow(clippy::module_name_repetitions)]

#[repr(u8)]
#[derive(Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
pub enum FormatType {
    Other = 0,
    A = 1,
    Argb = 2,
    Abgr = 3,
    Color = 4,
    Gray = 5,
    Yuy2 = 6,
    Yv12 = 7,
    Bgra = 8,
    Rgba = 9,
    ArgbSrgb = 10,
    RgbaFloat = 11,
}
