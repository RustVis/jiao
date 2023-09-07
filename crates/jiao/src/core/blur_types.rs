// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that can be found
// in the LICENSE file.

#[repr(u8)]
#[derive(Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum BlurStyle {
    /// fuzzy inside and outside
    Normal,

    /// solid inside, fuzzy outside
    Solid,

    /// nothing inside, fuzzy outside
    Outer,

    /// fuzzy inside, nothing outside
    Inner,
}
