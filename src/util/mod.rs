// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
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
