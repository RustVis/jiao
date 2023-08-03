// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

#[repr(u8)]
#[derive(Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
pub enum TileMode {
    /// Replicate the edge color if the shader draws outside of its original bounds.
    Clamp,

    /// Repeat the shader's image horizontally and vertically.
    Repeat,

    /// Repeat the shader's image horizontally and vertically, alternating
    /// mirror images so that adjacent images always seam.
    Mirror,

    /// Only draw within the original domain, return transparent-black everywhere else.
    Decal,
}

pub const LAST_TILE_MODE: TileMode = TileMode::Decal;

pub const TILE_MODE_COUNT: usize = LAST_TILE_MODE as usize + 1;
