// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by BSD-style License that can be found
// in the LICENSE file.

#![allow(clippy::module_name_repetitions)]

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Box16 {
    pub x1: i16,
    pub y1: i16,
    pub x2: i16,
    pub y2: i16,
}
