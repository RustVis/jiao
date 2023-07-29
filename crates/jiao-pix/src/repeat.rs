// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by BSD-style License that can be found
// in the LICENSE file.

#[repr(u8)]
#[derive(Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
pub enum Repeat {
    None,
    Normal,
    Pad,
    Reflect,
}
