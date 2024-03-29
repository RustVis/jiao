// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that can be found
// in the LICENSE file.

use crate::painting::Color;

pub mod categorical;
pub mod diverging;

pub type ColorPalette<'a> = &'a [Color];
