// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

use core::f64;

/// Easing equation function for a simple linear tweening, with no easing.
///
/// Returns current value.
///
/// # Arguments
/// * `progress` - Current time (in frames or seconds)
pub fn ease_none(progress: f64) -> f64 {
    progress
}

/// Easing equation function for a quadratic (t^2) easing in: accelerating from zero velocity.
pub fn ease_in_quad(t: f64) -> f64 {
    t * t
}

/// Easing equation function for a quadratic (t^2) easing out: decelerating to zero velocity.
pub fn ease_out_quad(t: f64) -> f64 {
    -t * (t - 2.0)
}

/// Easing equation function for a quadratic (t^2) easing in/out: acceleration until halfway, then deceleration.
pub fn ease_in_out_quad(mut t: f64) -> f64 {
    t *= 2.0;
    if t < 1.0 {
        t * t / 2.0
    } else {
        t -= 1.0;
        -0.5 * (t * (t - 2.0) - 1.0)
    }
}

/// Easing equation function for a quadratic (t^2) easing out/in: deceleration until halfway, then acceleration.
pub fn ease_out_in_quad(mut t: f64) -> f64 {
    if t < 0.5 {
        ease_out_quad(t * 2.0) / 2.0
    } else {
        ease_in_quad((t * 2.0) - 1.0) / 2.0 + 0.5
    }
}

/// Easing equation function for a cubic (t^3) easing in: accelerating from zero velocity.
pub fn ease_in_cubic(t: f64) -> f64 {
    t * t * t
}

/// Easing equation function for a cubic (t^3) easing out: decelerating to zero velocity.
pub fn ease_out_cubic(mut t: f64) -> f64 {
    t -= 1.0;
    t * t * t + 1.0
}

/// Easing equation function for a cubic (t^3) easing in/out: acceleration until halfway, then deceleration.
pub fn ease_in_out_cubic(mut t: f64) -> f64 {
    t *= 2.0;
    if t < 1.0 {
        0.5 * t * t * t
    } else {
        t -= 2.0;
        0.5 * (t * t * t + 2.0)
    }
}

/// Easing equation function for a cubic (t^3) easing out/in: deceleration until halfway, then acceleration.
pub fn ease_out_in_cubic(t: f64) -> f64 {
    if t < 0.5 {
        ease_out_cubic(t * 2.0) / 2.0
    } else {
        ease_in_cubic(t * 2.0 - 1.0) / 2.0 + 0.5
    }
}

/// Easing equation function for a quartic (t^4) easing in: accelerating from zero velocity.
pub fn ease_in_quart(t: f64) -> f64 {
    t * t * t * t
}

/// Easing equation function for a quartic (t^4) easing out: decelerating to zero velocity.
pub fn ease_out_quart(mut t: f64) -> f64 {
    t -= 1.0;
    -(t * t * t * t - 1.0)
}

/// Easing equation function for a quartic (t^4) easing in/out: acceleration until halfway, then deceleration.
pub fn ease_in_out_quart(mut t: f64) -> f64 {
    t *= 2.0;
    if t < 1.0 {
        0.5 * t * t * t * t
    } else {
        t -= 2.0;
        -0.5 * (t * t * t * t - 2.0)
    }
}

/// Easing equation function for a quartic (t^4) easing out/in: deceleration until halfway, then acceleration.
pub fn ease_out_in_quart(t: f64) -> f64 {
    if t < 0.5 {
        ease_out_quart(2.0 * t) / 2.0
    } else {
        ease_in_quart(2.0 * t - 1.0) / 2.0 + 0.5
    }
}

/// Easing equation function for a quintic (t^5) easing in: accelerating from zero velocity.
pub fn ease_in_quint(t: f64) -> f64 {
    t * t * t * t * t
}

/// Easing equation function for a quintic (t^5) easing out: decelerating to zero velocity.
pub fn ease_out_quint(mut t: f64) -> f64 {
    t -= 1.0;
    t * t * t * t * t + 1.0
}

/// Easing equation function for a quintic (t^5) easing in/out: acceleration until halfway, then deceleration.
pub fn ease_in_out_quint(mut t: f64) -> f64 {
    t *= 2.0;
    if t < 1.0 {
        0.5 * t * t * t * t * t
    } else {
        t -= 2.0;
        0.5 * (t * t * t * t * t + 2.0)
    }
}

/// Easing equation function for a quintic (t^5) easing out/in: deceleration until halfway, then acceleration.
pub fn ease_out_in_quint(t: f64) -> f64 {
    if t < 0.5 {
        ease_out_quint(2.0 * t) / 2.0
    } else {
        ease_in_quint(2.0 * t - 1.0) / 2.0 + 0.5
    }
}

/// Easing equation function for a sinusoidal (sin(t)) easing in: accelerating from zero velocity.
pub fn ease_in_sine(t: f64) -> f64 {
    if t == 1.0 {
        1.0
    } else {
        -(t * f64::consts::FRAC_PI_2).cos() + 1.0
    }
}
