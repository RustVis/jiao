// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

use float_cmp::ApproxEq;

/// Compares two floating point values and returns true if they are considered equal,
/// otherwise false.
#[inline]
#[must_use]
pub fn fuzzy_compare(p1: f64, p2: f64) -> bool {
    p1.approx_eq(p2, (0.0, 1))
}

/// Check floating point equals 0.0.
#[inline]
#[must_use]
pub fn fuzzy_is_zero(p: f64) -> bool {
    fuzzy_compare(p, 0.0)
}

/// Compares two floating point values and returns true if they are considered equal,
/// otherwise false.
#[inline]
#[must_use]
pub fn fuzzy_compare_f32(p1: f32, p2: f32) -> bool {
    p1.approx_eq(p2, (0.0, 1))
}

/// Check floating point equals 0.0.
#[inline]
#[must_use]
pub fn fuzzy_is_zero_f32(p: f32) -> bool {
    fuzzy_compare_f32(p, 0.0)
}

#[must_use]
pub fn div_257_floor(x: u32) -> u8 {
    ((x - (x >> 8)) >> 8) as u8
}

#[must_use]
pub fn div_257(x: u16) -> u8 {
    div_257_floor(u32::from(x) + 128)
}
