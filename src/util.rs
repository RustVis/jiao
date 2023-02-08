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
#[allow(clippy::cast_possible_truncation)]
pub const fn div_257_floor(x: u32) -> u8 {
    ((x - (x >> 8)) >> 8) as u8
}

#[must_use]
pub fn div_257(x: u16) -> u8 {
    div_257_floor(u32::from(x) + 128)
}

#[inline]
#[must_use]
pub const fn degrees_to_radians32(degrees: f32) -> f32 {
    degrees * std::f32::consts::PI / 180.0
}

#[inline]
#[must_use]
pub const fn radians_to_degrees32(radians: f32) -> f32 {
    radians * (180.0 / std::f32::consts::PI)
}

/// This function converts the degrees in float to radians.
#[inline]
#[must_use]
pub const fn degrees_to_radians(degrees: f64) -> f64 {
    degrees * std::f64::consts::PI / 180.0
}

/// This function converts the radians in float to degrees.
#[inline]
#[must_use]
pub const fn radians_to_degrees(radians: f64) -> f64 {
    radians * (180.0 / std::f64::consts::PI)
}
