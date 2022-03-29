// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

/// Compares two floating point values and returns true if they are considered equal,
/// otherwise false.
#[inline]
pub fn fuzzy_compare(p1: f64, p2: f64) -> bool {
    p1.to_bits() == p2.to_bits()
}

/// Check floating point equals zero.
#[inline]
pub fn fuzzy_is_zero(p: f64) -> bool {
    fuzzy_compare(p, 0.0)
}

pub fn div_257_floor(x: u32) -> u8 {
    ((x - (x >> 8)) >> 8) as u8
}

pub fn div_257(x: u16) -> u8 {
    div_257_floor(x as u32 + 128)
}
