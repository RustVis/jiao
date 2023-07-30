// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by BSD-style License that can be found
// in the LICENSE file.

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Color {
    pub red: u16,
    pub green: u16,
    pub blue: u16,
    pub alpha: u16,
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct Argb {
    pub alpha: f32,
    pub red: f32,
    pub green: f32,
    pub blue: f32,
}
