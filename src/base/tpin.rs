// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that can be found
// in the LICENSE file.

/// Returns x pinned (clamped) between lo and hi, inclusively.
///
/// Unlike `clamp()`, `tpin()` always returns a value between low and hi.
/// If x is NaN, `tpin()` returns lo but `clamp()` returns NaN.
#[must_use]
#[inline]
pub fn tpin(x: f32, lo: f32, hi: f32) -> f32 {
    // FIXME(Shaohua): `2.0.min(f32::NAN)` returns 2.0 in rust, while returns `NAN` in C++17.
    if x.is_nan() {
        lo
    } else {
        lo.max(hi.min(x))
    }
}
