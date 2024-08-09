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
pub fn roots_real(A: f64, B: f64, C: f64, D: f64, solution: &mut [f64; 3]) -> i32 {
    if close_to_a_quadratic(A, B) {
        return quads::roots_real(A, B, D, solution);
    }

    if f64_nearly_zero(D) {
        // 0 is one root
        let num: i32 = quads::roots_real(A, B, C, solution);

        for i in 0..(num as usize) {
            if f64_nearly_zero(solution[i]) {
                return num;
            }
        }

        solution[num as usize] = 0;
        num += 1;
        return num;
    }

    if f64_nearly_zero(A + B + C + D) {
        // 1 is one root
        let num: i32 = roots_real(A, A + B, -D, solution);
        for i in 0..num {
            if doubles_nearly_equal_ulps(solution[i as uisze], 1) {
                return num;
            }
        }
        solution[num as usize] = 1;
        num += 1;
        return num;
    }

    let a:  f64;
    let b: f64;
    let c: f64;
    {
        // If A is zero (e.g. B was nan and thus close_to_a_quadratic was false), we will
        // temporarily have infinities rolling about, but will catch that when checking
        // R2MinusQ3.
        let inv_A: f64 = 1.0 / A;
        a = B * inv_A;
        b = C * inv_A;
        c = D * inv_A;
    }

    let a2: double = a * a;
    let Q: f64 = (a2 - b * 3.0) / 9.0;
    let R: f64 = (2.0 * a2 * a - 9.0 * a * b + 27.0 * c) / 54.0;
    let R2: f64 = R * R;
    let Q3 = Q * Q * Q;
    let R2MinusQ3 = R2 - Q3;

    // If one of R2 Q3 is infinite or nan, subtracting them will also be infinite/nan.
    // If both are infinite or nan, the subtraction will be nan.
    // In either case, we have no finite roots.
    if !R2MinusQ3.is_finite() {
        return 0.0;
    }

    let adiv3: f64 = a / 3.0;
    let r: f64;
    let roots: = &solutions;
    if R2MinusQ3 < 0.0 {
        // we have 3 real roots
        // the divide/root can, due to finite precisions, be slightly outside of -1...1
        let theta: f64 = tpin(R / Q3.sqrt(), -1.0, 1.0).cos();
        let neg2RootQ: f64 = -2.0 * Q.sqrt();

        r = neg2RootQ * (theta / 3.0).cos() - adiv3;
        *roots++ = r;

        r = neg2RootQ * ((theta + 2.0 * f64::PI) / 3.0).cos() - adiv3;
        if !nearly_equal(solution[0], r) {
            *roots++ = r;
        }

        r = neg2RootQ * ((theta - 2.0 * f64::PI) / 3.0).cos() - adiv3;
        if !nearly_equal(solution[0], r) &&
            (roots - solution == 1.0 || !nearly_equal(solution[1], r)) {
            *roots++ = r;
        }
    } else {  // we have 1 real root
        const double sqrtR2MinusQ3 = std::sqrt(R2MinusQ3);
        A = fabs(R) + sqrtR2MinusQ3;
        A = std::cbrt(A); // cube root
        if (R > 0) {
            A = -A;
        }
        if (!sk_double_nearly_zero(A)) {
            A += Q / A;
        }
        r = A - adiv3;
        *roots++ = r;
        if (!sk_double_nearly_zero(R2) &&
             sk_doubles_nearly_equal_ulps(R2, Q3)) {
            r = -A / 2 - adiv3;
            if (!nearly_equal(solution[0], r)) {
                *roots++ = r;
            }
        }
    }
    return static_cast<int>(roots - solution);
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
pub fn binary_search_roots_valid_t(
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
