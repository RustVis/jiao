// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that can be found
// in the LICENSE file.

//! Utilities for dealing with cubic formulas with one variable:
//! `f(t) = A*t^3 + B*t^2 + C*t + d`

#![allow(clippy::many_single_char_names)]

use crate::base::floating_point::{doubles_nearly_equal, f64_nearly_zero};

fn nearly_equal(x: f64, y: f64) -> bool {
    if f64_nearly_zero(x) {
        f64_nearly_zero(y)
    } else {
        doubles_nearly_equal(x, y)
    }
}

// When the A coefficient of a cubic is close to 0, there can be floating point error
// that arises from computing a very large root.
//
// In those cases, we would rather be precise about the smaller 2 roots,
// so we have this arbitrary cutoff for when A is really small or small compared to B.
fn close_to_a_quadratic(a: f64, b: f64) -> bool {
    if f64_nearly_zero(b) {
        f64_nearly_zero(a)
    } else {
        (a / b).abs() < 1.0e-7
    }
}

/// Puts up to 3 real solutions to the equation
/// `A*t^3 + B*t^2 + C*t + d = 0`
/// in the provided array and returns how many roots that was.
#[must_use]
#[inline]
pub fn roots_real(_a: f64, _b: f64, _c: f64, _d: f64, _solution: &mut [f64; 3]) -> i32 {
    unimplemented!()
}

/// Puts up to 3 real solutions to the equation
/// `A*t^3 + B*t^2 + C*t + D = 0`
/// in the provided array, with the constraint that t is in the range [0.0, 1.0],
/// and returns how many roots that was.
#[must_use]
#[inline]
pub fn roots_valid_t(_a: f64, _b: f64, _c: f64, _d: f64, _solution: &mut [f64; 3]) -> i32 {
    unimplemented!()
}

/// Puts up to 3 real solutions to the equation
/// `A*t^3 + B*t^2 + C*t + D = 0`
/// in the provided array, with the constraint that t is in the range [0.0, 1.0],
/// and returns how many roots that was.
///
/// This is a slower method than `roots_valid_t`, but more accurate in circumstances
/// where floating point error gets too big.
#[must_use]
#[inline]
pub fn binary_searchroots_valid_t(
    _a: f64,
    _b: f64,
    _c: f64,
    _d: f64,
    _solution: &mut [f64; 3],
) -> i32 {
    unimplemented!()
}

/// Evaluates the cubic function with the 4 provided coefficients and the
/// provided variable.
#[must_use]
#[inline]
pub fn eval_at(a: f64, b: f64, c: f64, d: f64, t: f64) -> f64 {
    t.mul_add(t.mul_add(t.mul_add(a, b), c), d)
}

#[must_use]
#[inline]
pub fn eval_at_slice(coefficients: &[f64; 4], t: f64) -> f64 {
    eval_at(
        coefficients[0],
        coefficients[1],
        coefficients[2],
        coefficients[3],
        t,
    )
}
