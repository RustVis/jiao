// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

use super::{constant, ReturnFunc};
use crate::util::{fuzzy_compare, fuzzy_is_zero};

#[must_use]
pub fn linear(a: f64, b: f64) -> ReturnFunc {
    Box::new(move |t: f64| -> f64 { t.mul_add(b, a) })
}

/// # Panics
///
/// Raise panic if y is zero.
#[must_use]
pub fn exponential(a: f64, b: f64, y: f64) -> ReturnFunc {
    assert!(!fuzzy_is_zero(y));
    let a = a.powf(y);
    let b = b.powf(y) - a;
    let y = 1.0 / y;
    Box::new(move |t: f64| -> f64 { t.mul_add(b, a).powf(y) })
}

#[must_use]
pub fn hue(a: f64, b: f64) -> ReturnFunc {
    let d = b - a;
    if d != 0.0 {
        let c = if d < -180.0 || d > 180.0 {
            d - 360.0 * (d / 360.0).round()
        } else {
            d
        };
        linear(a, c)
    } else if a.is_nan() {
        constant(b)
    } else {
        constant(a)
    }
}

pub fn gamma(y: f64) -> impl Fn(f64, f64) -> ReturnFunc {
    move |a: f64, b: f64| {
        if fuzzy_compare(y, 1.0) {
            nogamma(a, b)
        } else if !fuzzy_compare(a, b) {
            exponential(a, b, y)
        } else if a.is_nan() {
            constant(b)
        } else {
            constant(a)
        }
    }
}

#[must_use]
pub fn nogamma(a: f64, b: f64) -> ReturnFunc {
    let d = b - a;
    if d != 0.0 {
        linear(a, d)
    } else if a.is_nan() {
        constant(b)
    } else {
        constant(a)
    }
}
