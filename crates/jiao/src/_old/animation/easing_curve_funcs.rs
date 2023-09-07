// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that can be found
// in the LICENSE file.

//! These easing functions are based on Robert Penner's Easing Equations.

#![allow(clippy::suboptimal_flops)]

use core::f64::consts::{FRAC_PI_2, PI};

use crate::util::{fuzzy_compare, fuzzy_is_zero};

/// Easing equation function for a simple linear tweening, with no easing.
///
/// Returns current value.
///
/// # Arguments
/// * `progress` - Current time (in frames or seconds)
#[must_use]
pub const fn ease_none(progress: f64) -> f64 {
    progress
}

/// Easing equation function for a quadratic (t^2) easing in: accelerating from zero velocity.
#[must_use]
pub fn ease_in_quad(t: f64) -> f64 {
    t * t
}

/// Easing equation function for a quadratic (t^2) easing out: decelerating to zero velocity.
#[must_use]
pub fn ease_out_quad(t: f64) -> f64 {
    -t * (t - 2.0)
}

/// Easing equation function for a quadratic (t^2) easing in/out: acceleration until halfway,
/// then deceleration.
#[must_use]
pub fn ease_in_out_quad(mut t: f64) -> f64 {
    t *= 2.0;
    if t < 1.0 {
        t * t / 2.0
    } else {
        t -= 1.0;
        -0.5 * (t * (t - 2.0) - 1.0)
    }
}

/// Easing equation function for a quadratic (t^2) easing out/in: deceleration until halfway,
/// then acceleration.
#[must_use]
pub fn ease_out_in_quad(t: f64) -> f64 {
    if t < 0.5 {
        ease_out_quad(t * 2.0) / 2.0
    } else {
        ease_in_quad((t * 2.0) - 1.0) / 2.0 + 0.5
    }
}

/// Easing equation function for a cubic (t^3) easing in: accelerating from zero velocity.
#[must_use]
pub fn ease_in_cubic(t: f64) -> f64 {
    t * t * t
}

/// Easing equation function for a cubic (t^3) easing out: decelerating to zero velocity.
#[must_use]
pub fn ease_out_cubic(mut t: f64) -> f64 {
    t -= 1.0;
    (t * t).mul_add(t, 1.0)
}

/// Easing equation function for a cubic (t^3) easing in/out: acceleration until halfway,
/// then deceleration.
#[must_use]
pub fn ease_in_out_cubic(mut t: f64) -> f64 {
    t *= 2.0;
    if t < 1.0 {
        0.5 * t * t * t
    } else {
        t -= 2.0;
        0.5 * (t * t).mul_add(t, 2.0)
    }
}

/// Easing equation function for a cubic (t^3) easing out/in: deceleration until halfway,
/// then acceleration.
#[must_use]
pub fn ease_out_in_cubic(t: f64) -> f64 {
    if t < 0.5 {
        ease_out_cubic(t * 2.0) / 2.0
    } else {
        ease_in_cubic(t * 2.0 - 1.0) / 2.0 + 0.5
    }
}

/// Easing equation function for a quartic (t^4) easing in: accelerating from zero velocity.
#[must_use]
pub fn ease_in_quart(t: f64) -> f64 {
    t * t * t * t
}

/// Easing equation function for a quartic (t^4) easing out: decelerating to zero velocity.
#[must_use]
pub fn ease_out_quart(mut t: f64) -> f64 {
    t -= 1.0;
    -(t * t * t * t - 1.0)
}

/// Easing equation function for a quartic (t^4) easing in/out: acceleration until halfway,
/// then deceleration.
#[must_use]
pub fn ease_in_out_quart(mut t: f64) -> f64 {
    t *= 2.0;
    if t < 1.0 {
        0.5 * t * t * t * t
    } else {
        t -= 2.0;
        -0.5 * (t * t * t * t - 2.0)
    }
}

/// Easing equation function for a quartic (t^4) easing out/in: deceleration until halfway,
/// then acceleration.
#[must_use]
pub fn ease_out_in_quart(t: f64) -> f64 {
    if t < 0.5 {
        ease_out_quart(2.0 * t) / 2.0
    } else {
        ease_in_quart(2.0 * t - 1.0) / 2.0 + 0.5
    }
}

/// Easing equation function for a quintic (t^5) easing in: accelerating from zero velocity.
#[must_use]
pub fn ease_in_quint(t: f64) -> f64 {
    t * t * t * t * t
}

/// Easing equation function for a quintic (t^5) easing out: decelerating to zero velocity.
#[must_use]
pub fn ease_out_quint(mut t: f64) -> f64 {
    t -= 1.0;
    (t * t * t * t).mul_add(t, 1.0)
}

/// Easing equation function for a quintic (t^5) easing in/out: acceleration until halfway,
/// then deceleration.
#[must_use]
pub fn ease_in_out_quint(mut t: f64) -> f64 {
    t *= 2.0;
    if t < 1.0 {
        0.5 * t * t * t * t * t
    } else {
        t -= 2.0;
        0.5 * (t * t * t * t).mul_add(t, 2.0)
    }
}

/// Easing equation function for a quintic (t^5) easing out/in: deceleration until halfway,
/// then acceleration.
#[must_use]
pub fn ease_out_in_quint(t: f64) -> f64 {
    if t < 0.5 {
        ease_out_quint(2.0 * t) / 2.0
    } else {
        ease_in_quint(2.0 * t - 1.0) / 2.0 + 0.5
    }
}

/// Easing equation function for a sinusoidal (sin(t)) easing in: accelerating from zero velocity.
#[must_use]
pub fn ease_in_sine(t: f64) -> f64 {
    if fuzzy_compare(t, 1.0) {
        1.0
    } else {
        -(t * FRAC_PI_2).cos() + 1.0
    }
}

/// Easing equation function for a sinusoidal (sin(t)) easing out: decelerating to zero velocity.
#[must_use]
pub fn ease_out_sine(t: f64) -> f64 {
    (t * FRAC_PI_2).sin()
}

/// Easing equation function for a sinusoidal (sin(t)) easing in/out: acceleration until halfway,
/// then deceleration.
#[must_use]
pub fn ease_in_out_sine(t: f64) -> f64 {
    -0.5 * ((PI * t).cos() - 1.0)
}

/// Easing equation function for a sinusoidal (sin(t)) easing out/in: deceleration until halfway,
/// then acceleration.
#[must_use]
pub fn ease_out_in_sine(t: f64) -> f64 {
    if t < 0.5 {
        ease_out_sine(2.0 * t) / 2.0
    } else {
        ease_in_sine(2.0 * t - 1.0) / 2.0 + 0.5
    }
}

/// Easing equation function for an exponential (2^t) easing in: accelerating from zero velocity.
#[must_use]
pub fn ease_in_expo(t: f64) -> f64 {
    if fuzzy_is_zero(t) || fuzzy_compare(t, 1.0) {
        t
    } else {
        (10.0 * (t - 1.0)).exp2() - 0.001
    }
}

/// Easing equation function for an exponential (2^t) easing out: decelerating to zero velocity.
#[must_use]
pub fn ease_out_expo(t: f64) -> f64 {
    if fuzzy_compare(t, 1.0) {
        1.0
    } else {
        1.001 * (-(-10.0 * t).exp2() + 1.0)
    }
}

/// Easing equation function for an exponential (2^t) easing in/out: acceleration until halfway,
/// then deceleration.
#[must_use]
pub fn ease_in_out_expo(mut t: f64) -> f64 {
    if fuzzy_is_zero(t) {
        return 0.0;
    }
    if fuzzy_compare(t, 1.0) {
        return 1.0;
    }
    t *= 2.0;
    if t < 1.0 {
        return 0.5 * (10.0 * (t - 1.0)).exp2() - 0.0005;
    }
    0.5 * 1.0005 * (-(-10.0 * (t - 1.0)).exp2() + 2.0)
}

/// Easing equation function for an exponential (2^t) easing out/in: deceleration until halfway,
/// then acceleration.
#[must_use]
pub fn ease_out_in_expo(t: f64) -> f64 {
    if t < 0.5 {
        ease_out_expo(2.0 * t) / 2.0
    } else {
        ease_in_expo(2.0 * t - 1.0) / 2.0 + 0.5
    }
}

/// Easing equation function for a circle (sqrt(1-t^2)) easing in: accelerating from zero velocity.
#[must_use]
pub fn ease_in_circ(t: f64) -> f64 {
    -((1.0 - t * t).sqrt() - 1.0)
}

/// Easing equation function for a circle (sqrt(1-t^2)) easing out: decelerating to zero velocity.
#[must_use]
pub fn ease_out_circ(mut t: f64) -> f64 {
    t -= 1.0;
    (1.0 - t * t).sqrt()
}

/// Easing equation function for a circle (sqrt(1-t^2)) easing in/out: acceleration until halfway,
/// then deceleration.
#[must_use]
pub fn ease_in_out_circ(mut t: f64) -> f64 {
    t -= 2.0;
    if t < 1.0 {
        -0.5 * ((1.0 - t * t).sqrt() - 1.0)
    } else {
        t -= 2.0;
        0.5 * ((1.0 - t * t).sqrt() + 1.0)
    }
}

/// Easing equation function for a circle (sqrt(1-t^2)) easing out/in: deceleration until halfway,
/// then acceleration.
#[must_use]
pub fn ease_out_in_circ(t: f64) -> f64 {
    if t < 0.5 {
        ease_out_circ(2.0 * t) / 2.0
    } else {
        ease_in_circ(2.0 * t - 1.0) / 2.0 + 0.5
    }
}

fn ease_in_elastic_helper(t: f64, b: f64, c: f64, d: f64, mut amplitude: f64, period: f64) -> f64 {
    if fuzzy_is_zero(t) {
        return b;
    }
    let mut t_adj = t / d;
    if fuzzy_compare(t_adj, 1.0) {
        return b + c;
    }

    let step = if amplitude < c.abs() {
        amplitude = c;
        period / 4.0
    } else {
        period / (2.0 * PI) * (c / amplitude).asin()
    };

    t_adj -= 1.0;
    -(amplitude * (10.0 * t_adj).exp2() * ((t_adj * d - step) * (2.0 * PI) / period).sin()) + b
}

/// Easing equation function for an elastic (exponentially decaying sine wave) easing in:
/// accelerating from zero velocity.
#[must_use]
pub fn ease_in_elastic(t: f64, amplitude: f64, period: f64) -> f64 {
    ease_in_elastic_helper(t, 0.0, 1.0, 1.0, amplitude, period)
}

fn ease_out_elastic_helper(
    t: f64,
    _b: f64,
    c: f64,
    _d: f64,
    mut amplitude: f64,
    period: f64,
) -> f64 {
    if fuzzy_is_zero(t) {
        return 0.0;
    }
    if fuzzy_compare(t, 1.0) {
        return c;
    }

    let step = if amplitude < c {
        amplitude = c;
        period / 4.0
    } else {
        period / (2.0 * PI) * (c / amplitude).asin()
    };

    (amplitude * (-10.0 * t).exp2()).mul_add(((t - step) * (2.0 * PI) / period).sin(), c)
}

/// Easing equation function for an elastic (exponentially decaying sine wave) easing out:
/// decelerating to zero velocity.
#[must_use]
pub fn ease_out_elastic(t: f64, amplitude: f64, period: f64) -> f64 {
    ease_out_elastic_helper(t, 0.0, 1.0, 1.0, amplitude, period)
}

/// Easing equation function for an elastic (exponentially decaying sine wave) easing in/out:
/// acceleration until halfway, then deceleration.
#[must_use]
pub fn ease_in_out_elastic(mut t: f64, mut amplitude: f64, period: f64) -> f64 {
    if fuzzy_is_zero(t) {
        return 0.0;
    }
    t *= 2.0;
    if fuzzy_compare(t, 2.0) {
        return 1.0;
    }

    let s = if amplitude < 1.0 {
        amplitude = 1.0;
        period / 4.0
    } else {
        period / (2.0 * PI) * (1.0 / amplitude).asin()
    };

    if t < 1.0 {
        return -0.5
            * (amplitude
                * (10.0 * (t - 1.0)).exp2()
                * ((t - 1.0 - s) * (2.0 * PI) / period).sin());
    }
    (amplitude * (-10.0 * (t - 1.0)).exp2() * ((t - 1.0 - s) * (2.0 * PI) / period).sin())
        .mul_add(0.5, 1.0)
}

/// Easing equation function for an elastic (exponentially decaying sine wave) easing out/in:
/// deceleration until halfway, then acceleration.
#[must_use]
pub fn ease_out_in_elastic(t: f64, amplitude: f64, period: f64) -> f64 {
    if t < 0.5 {
        return ease_out_elastic_helper(t * 2.0, 0.0, 0.5, 1.0, amplitude, period);
    }
    ease_in_elastic_helper(2.0 * t - 1.0, 0.5, 0.5, 1.0, amplitude, period)
}

/// Easing equation function for a back (overshooting cubic easing: (s+1)*t^3 - s*t^2) easing in:
/// accelerating from zero velocity.
///
/// Returns the correct value.
///
/// # Arguments
/// * `t` - Current time (in frames or seconds)
/// * `s` - Overshoot ammount: higher s means greater overshoot (0 produces cubic easing with no overshoot,
///       and the default value of 1.70158 produces an overshoot of 10 percent).
#[must_use]
pub fn ease_in_back(t: f64, s: f64) -> f64 {
    t * t * ((s + 1.0) * t - s)
}

/// Easing equation function for a back (overshooting cubic easing: (s+1)*t^3 - s*t^2) easing out:
/// decelerating to zero velocity.
///
/// Returns the correct value.
///
/// # Arguments
/// * `t` - Current time (in frames or seconds)
/// * `s` - Overshoot ammount: higher s means greater overshoot (0 produces cubic easing with no overshoot,
///       and the default value of 1.70158 produces an overshoot of 10 percent).
#[must_use]
pub fn ease_out_back(mut t: f64, s: f64) -> f64 {
    t -= 1.0;
    (t * t).mul_add((s + 1.0).mul_add(t, s), 1.0)
}

/// Easing equation function for a back (overshooting cubic easing: (s+1)*t^3 - s*t^2) easing in/out:
/// acceleration until halfway, then deceleration.
///
/// Returns the correct value.
///
/// # Arguments
/// * `t` - Current time (in frames or seconds)
/// * `s` - Overshoot ammount: higher s means greater overshoot (0 produces cubic easing with no overshoot,
///       and the default value of 1.70158 produces an overshoot of 10 percent).
#[must_use]
pub fn ease_in_out_back(mut t: f64, mut s: f64) -> f64 {
    t *= 2.0;
    if t < 1.0 {
        s *= 1.525;
        0.5 * (t * t * ((s + 1.0) * t - s))
    } else {
        t -= 2.0;
        s *= 1.525;
        0.5 * (t * t).mul_add((s + 1.0).mul_add(t, s), 2.0)
    }
}

/// Easing equation function for a back (overshooting cubic easing: (s+1)*t^3 - s*t^2) easing out/in:
/// deceleration until halfway, then acceleration.
///
/// Returns the correct value.
///
/// # Arguments
/// * `t` - Current time (in frames or seconds)
/// * `s` - Overshoot ammount: higher s means greater overshoot (0 produces cubic easing with no overshoot,
///       and the default value of 1.70158 produces an overshoot of 10 percent).
#[must_use]
pub fn ease_out_in_back(t: f64, s: f64) -> f64 {
    if t < 0.5 {
        return ease_out_back(2.0 * t, s) / 2.0;
    }
    ease_in_back(2.0 * t - 1.0, s) / 2.0 + 0.5
}

fn ease_out_bounce_helper(mut t: f64, c: f64, a: f64) -> f64 {
    if fuzzy_compare(t, 1.0) {
        return c;
    }
    if t < (4.0 / 11.0) {
        c * (7.5625 * t * t)
    } else if t < (8.0 / 11.0) {
        t -= 6.0 / 11.0;
        (-a).mul_add(1.0 - (7.5625 * t).mul_add(t, 0.75), c)
    } else if t < (10.0 / 11.0) {
        t -= 9.0 / 11.0;
        (-a).mul_add(1.0 - (7.5625 * t).mul_add(t, 0.9375), c)
    } else {
        t -= 21.0 / 22.0;
        (-a).mul_add(1.0 - (7.5625 * t).mul_add(t, 0.984_375), c)
    }
}

/// Easing equation function for a bounce (exponentially decaying parabolic bounce) easing in:
/// accelerating from zero velocity.
#[must_use]
pub fn ease_in_bounce(t: f64, amplitude: f64) -> f64 {
    1.0 - ease_out_bounce_helper(1.0 - t, 1.0, amplitude)
}

/// Easing equation function for a bounce (exponentially decaying parabolic bounce) easing out:
/// decelerating to zero velocity.
#[must_use]
pub fn ease_out_bounce(t: f64, amplitude: f64) -> f64 {
    ease_out_bounce_helper(t, 1.0, amplitude)
}

/// Easing equation function for a bounce (exponentially decaying parabolic bounce) easing in/out:
/// acceleration until halfway, then deceleration.
#[must_use]
pub fn ease_in_out_bounce(t: f64, amplitude: f64) -> f64 {
    if t < 0.5 {
        ease_in_bounce(2.0 * t, amplitude) / 2.0
    } else if fuzzy_compare(t, 1.0) {
        1.0
    } else {
        ease_out_bounce(2.0 * t - 1.0, amplitude) / 2.0 + 0.5
    }
}

/// Easing equation function for a bounce (exponentially decaying parabolic bounce) easing out/in:
/// deceleration until halfway, then acceleration.
#[must_use]
pub fn ease_out_in_bounce(t: f64, amplitude: f64) -> f64 {
    if t < 0.5 {
        ease_out_bounce_helper(t * 2.0, 0.5, amplitude)
    } else {
        1.0 - ease_out_bounce_helper(2.0 - 2.0 * t, 0.5, amplitude)
    }
}

#[inline]
fn sin_progress(value: f64) -> f64 {
    ((value * PI) - FRAC_PI_2).sin() / 2.0 + 0.5
}

#[inline]
fn smooth_begin_end_mix_factor(value: f64) -> f64 {
    let max_val = (1.0 - value * 2.0 + 0.3).max(0.0);
    max_val.min(1.0)
}

/// Easing function that starts growing slowly, then increases in speed. At the end of the curve
/// the speed will be constant.
///
/// `SmoothBegin` blends Smooth and Linear Interpolation.
/// - Progress 0 - 0.3      : Smooth only
/// - Progress 0.3 - ~ 0.5  : Mix of Smooth and Linear
/// - Progress ~ 0.5  - 1   : Linear only
#[must_use]
pub fn ease_in_curve(t: f64) -> f64 {
    let sin_progress_val = sin_progress(t);
    let mix = smooth_begin_end_mix_factor(t);
    sin_progress_val.mul_add(mix, t * (1.0 - mix))
}

/// Easing function that starts growing steadily, then ends slowly. The speed will be constant
/// at the beginning of the curve.
#[must_use]
pub fn ease_out_curve(t: f64) -> f64 {
    let sin_progress_val = sin_progress(t);
    let mix = smooth_begin_end_mix_factor(1.0 - t);
    sin_progress_val.mul_add(mix, t * (1.0 - mix))
}

/// Easing function where the value grows sinusoidally.
///
/// Note that the calculated end value will be 0 rather than 1.
#[must_use]
pub fn ease_sine_curve(t: f64) -> f64 {
    (((t * PI * 2.0) - FRAC_PI_2).sin() + 1.0) / 2.0
}

/// Easing function where the value grows cosinusoidally.
///
/// Note that the calculated start value will be 0.5 and the end value will be 0.5
/// contrary to the usual 0 to 1 easing curve.
#[must_use]
pub fn ease_cosine_curve(t: f64) -> f64 {
    (((t * PI * 2.0) - FRAC_PI_2).cos() + 1.0) / 2.0
}
