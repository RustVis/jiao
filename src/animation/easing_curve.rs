// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

use super::easing_curve_funcs as inner;
use crate::base::point::PointF;

const DEFAULT_AMPLITUDE: f64 = 1.0;
const DEFAULT_OVERSHOOT: f64 = 1.70158;
const DEFAULT_PERIOD: f64 = 0.3;

/// The type of easing curve.
// TODO(Shaohua): Add images to rust doc.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EasingCurveType {
    /// Easing curve for a linear (t) function: velocity is constant.
    Linear,

    /// Easing curve for a quadratic (t^2) function: accelerating from zero velocity.
    InQuad,

    /// Easing equation function for a quadratic (t^2) easing out: decelerating to zero velocity.
    OutQuad,

    /// Easing equation function for a quadratic (t^2) easing in/out: acceleration until halfway,
    /// then deceleration.
    InOutQuad,

    /// Easing equation function for a cubic (t^3) easing in: accelerating from zero velocity.
    OutInQuad,

    /// Easing equation function for a cubic (t^3) easing out: decelerating to zero velocity.
    InCubic,

    /// Easing equation function for a cubic (t^3) easing out: decelerating to zero velocity.
    OutCubic,

    /// Easing equation function for a cubic (t^3) easing in/out: acceleration until halfway,
    /// then deceleration.
    InOutCubic,

    /// Easing equation function for a cubic (t^3) easing out/in: deceleration until halfway,
    /// then acceleration.
    OutInCubic,

    /// Easing equation function for a quartic (t^4) easing in: accelerating from zero velocity.
    InQuart,

    /// Easing equation function for a quartic (t^4) easing out: decelerating to zero velocity.
    OutQuart,

    /// Easing equation function for a quartic (t^4) easing in/out: acceleration until halfway,
    /// then deceleration.
    InOutQuart,

    /// Easing equation function for a quartic (t^4) easing out/in: deceleration until halfway,
    /// then acceleration.
    OutInQuart,

    /// Easing equation function for a quintic (t^5) easing in: accelerating from zero velocity.
    InQuint,

    /// Easing equation function for a quintic (t^5) easing out: decelerating to zero velocity.
    OutQuint,

    /// Easing equation function for a quintic (t^5) easing in/out: acceleration until halfway,
    /// then deceleration.
    InOutQuint,

    /// Easing equation function for a quintic (t^5) easing out/in: deceleration until halfway,
    /// then acceleration.
    OutInQuint,

    /// Easing equation function for a sinusoidal (sin(t)) easing in: accelerating from zero velocity.
    InSine,

    /// Easing equation function for a sinusoidal (sin(t)) easing out: decelerating to zero velocity.
    OutSine,

    /// Easing equation function for a sinusoidal (sin(t)) easing in/out: acceleration until halfway,
    /// then deceleration.
    InOutSine,

    /// Easing equation function for a sinusoidal (sin(t)) easing out/in: deceleration until halfway,
    /// then acceleration.
    OutInSine,

    /// Easing equation function for an exponential (2^t) easing in: accelerating from zero velocity.
    InExpo,

    /// Easing equation function for an exponential (2^t) easing out: decelerating to zero velocity.
    OutExpo,

    /// Easing equation function for an exponential (2^t) easing in/out: acceleration until halfway,
    /// then deceleration.
    InOutExpo,

    /// Easing equation function for an exponential (2^t) easing out/in: deceleration until halfway,
    /// then acceleration.
    OutInExpo,

    /// Easing equation function for a circular (sqrt(1-t^2)) easing in: accelerating from zero velocity.
    InCirc,

    /// Easing equation function for a circular (sqrt(1-t^2)) easing out: decelerating to zero velocity.
    OutCirc,

    /// Easing equation function for a circular (sqrt(1-t^2)) easing in/out: acceleration until halfway,
    /// then deceleration.
    InOutCirc,

    /// Easing equation function for a circular (sqrt(1-t^2)) easing out/in: deceleration until halfway,
    /// then acceleration.
    OutInCirc,

    /// Easing equation function for an elastic (exponentially decaying sine wave) easing in:
    /// accelerating from zero velocity.
    InElastic,

    /// Easing equation function for an elastic (exponentially decaying sine wave) easing out:
    /// decelerating to zero velocity.
    OutElastic,

    /// Easing equation function for an elastic (exponentially decaying sine wave) easing in/out:
    /// acceleration until halfway, then deceleration.
    InOutElastic,

    /// Easing equation function for an elastic (exponentially decaying sine wave) easing out/in:
    /// deceleration until halfway, then acceleration.
    OutInElastic,

    /// Easing equation function for a back (overshooting cubic easing: (s+1)*t^3 - s*t^2) easing in:
    /// accelerating from zero velocity.
    InBack,

    /// Easing equation function for a back (overshooting cubic easing: (s+1)*t^3 - s*t^2) easing out:
    /// decelerating to zero velocity.
    OutBack,

    /// Easing equation function for a back (overshooting cubic easing: (s+1)*t^3 - s*t^2) easing in/out:
    /// acceleration until halfway, then deceleration.
    InOutBack,

    /// Easing equation function for a back (overshooting cubic easing: (s+1)*t^3 - s*t^2) easing out/in:
    /// deceleration until halfway, then acceleration.
    OutInBack,

    /// Easing equation function for a bounce (exponentially decaying parabolic bounce) easing in:
    /// accelerating from zero velocity.
    InBounce,

    /// Easing equation function for a bounce (exponentially decaying parabolic bounce) easing out:
    /// decelerating to zero velocity.
    OutBounce,

    /// Easing equation function for a bounce (exponentially decaying parabolic bounce) easing in/out:
    /// acceleration until halfway, then deceleration.
    InOutBounce,

    /// Easing equation function for a bounce (exponentially decaying parabolic bounce) easing out/in:
    /// deceleration until halfway, then acceleration.
    OutInBounce,

    /// Easing function that starts growing slowly, then increases in speed. At the end of the curve
    /// the speed will be constant.
    InCurve,

    /// Easing function that starts growing steadily, then ends slowly. The speed will be constant
    /// at the beginning of the curve.
    OutCurve,

    /// Easing function where the value grows sinusoidally.
    ///
    /// Note that the calculated end value will be 0 rather than 1.
    SineCurve,

    /// Easing function where the value grows cosinusoidally.
    ///
    /// Note that the calculated start value will be 0.5 and the end value will be 0.5
    /// contrary to the usual 0 to 1 easing curve.
    CosineCurve,

    /// Allows defining a custom easing curve using a cubic bezier spline.
    BezierSpline,

    /// Allows defining a custom easing curve using a TCB spline.
    TCBSpline,

    /// This is returned if the user specified a custom curve type with `set_custom_type()`.
    ///
    /// Note that you cannot call `set_type()` with this value, but `get_type()` can return it.
    Custom,
}

impl Default for EasingCurveType {
    fn default() -> Self {
        EasingCurveType::Linear
    }
}

/// The EasingCurve struct provides easing curves for controlling animation.
///
/// Easing curves describe a function that controls how the speed of the interpolation
/// between 0 and 1 should be. Easing curves allow transitions from one value to another
/// to appear more natural than a simple constant speed would allow. The EasingCurve struct
/// is usually used in conjunction with the PropertyAnimation struct but can be used on its own.
/// It is usually used to accelerate the interpolation from zero velocity (ease in) or
/// decelerate to zero velocity (ease out). Ease in and ease out can also be combined
/// in the same easing curve.
///
/// To calculate the speed of the interpolation, the easing curve provides the function
/// `value_for_progress()`, where the `progress` argument specifies the progress
/// of the interpolation: 0 is the start value of the interpolation, 1 is the end value
/// of the interpolation. The returned value is the effective progress of the interpolation.
/// If the returned value is the same as the input value for all input values the easing curve
/// is a linear curve. This is the default behaviour.
///
/// The ability to set an amplitude, overshoot, or period depends on the EasingCurveType.
/// Amplitude access is available to curves that behave as springs such as elastic and bounce curves.
/// Changing the amplitude changes the height of the curve. Period access is only available
/// to elastic curves and setting a higher period slows the rate of bounce. Only curves
/// that have "boomerang" behaviors such as the `InBack`, `OutBack`, `InOutBack`, and `OutInBack`
/// have overshoot settings. These curves will interpolate beyond the end points and
/// return to the end point, acting similar to a boomerang.
#[derive(Debug, Clone, PartialEq)]
pub struct EasingCurve {
    curve_type: EasingCurveType,
    amplitude: f64,
    overshoot: f64,
    period: f64,
    custom_func: Option<EasingFunction>,
}

/// Function for custom easing curve type.
///
/// `progress` and the return value are considered to be normalized between 0 and 1
/// (In some cases the return value can be outside that range).
pub type EasingFunction = fn(progress: f64) -> f64;

impl Default for EasingCurve {
    fn default() -> Self {
        Self {
            curve_type: EasingCurveType::Linear,
            amplitude: DEFAULT_AMPLITUDE,
            overshoot: DEFAULT_OVERSHOOT,
            period: DEFAULT_PERIOD,
            custom_func: None,
        }
    }
}

impl EasingCurve {
    pub fn new(curve_type: EasingCurveType) -> Self {
        Self {
            curve_type,
            ..Self::default()
        }
    }

    /// Returns the amplitude.
    ///
    /// This is not applicable for all curve types.
    /// It is only applicable for bounce and elastic curves.
    pub fn amplitude(&self) -> f64 {
        self.amplitude
    }

    /// Returns the function pointer to the custom easing curve.
    ///
    /// If `get_type()` does not return `EasingCurveType::Custom`, this function will return `None`.
    pub fn custom_type(&self) -> Option<EasingFunction> {
        self.custom_func
    }

    /// Returns the overshoot.
    ///
    /// This is not applicable for all curve types.
    ///
    /// It is only applicable if `get_type()` is:
    /// - `EasingCurveType::InBack`
    /// - `EasingCurveType::OutBack`
    /// - `EasingCurveType::InOutBack`
    /// - `EasingCurveType::OutInBack`
    pub fn overshoot(&self) -> f64 {
        self.overshoot
    }

    /// Returns the period.
    ///
    /// This is not applicable for all curve types.
    ///
    /// It is only applicable if `get_type()` is:
    /// - `EasingCurveType::InElastic`
    /// - `EasingCurveType::OutElastic`
    /// - `EasingCurveType::InOutElastic`
    /// - `EasingCurveType::OutInElastic`
    pub fn period(&self) -> f64 {
        self.period
    }

    /// Sets the amplitude to `amplitude`.
    ///
    /// This will set the amplitude of the bounce or the amplitude of the elastic "spring" effect.
    /// The higher the number, the higher the amplitude.
    pub fn set_amplitude(&mut self, amplitude: f64) {
        self.amplitude = amplitude;
    }

    /// Sets a custom easing curve that is defined by the user in the function `func`.
    ///
    /// After calling this function, `get_type()` will return `EasingCurveType::Custom`.
    pub fn set_custom_type(&mut self, func: EasingFunction) {
        self.custom_func = Some(func);
        self.curve_type = EasingCurveType::Custom;
    }

    /// Sets the overshoot to overshoot.
    /// 0 produces no overshoot, and the default value of 1.70158 produces an overshoot of 10 percent.
    pub fn set_overshoot(&mut self, overshoot: f64) {
        self.overshoot = overshoot;
    }

    /// Sets the period to period.
    ///
    /// Setting a small period value will give a high frequency of the curve.
    /// A large period will give it a small frequency.
    pub fn set_period(&mut self, period: f64) {
        self.period = period;
    }

    /// Sets the type of the easing curve to type.
    pub fn set_type(&mut self, curve_type: EasingCurveType) {
        debug_assert_ne!(curve_type, EasingCurveType::Custom);
        self.curve_type = curve_type;
    }

    /// Swaps curve other with this curve.
    ///
    /// This operation is very fast and never fails.
    pub fn swap(&mut self, other: &mut Self) {
        unimplemented!()
    }

    /// Returns the cubic bezier spline that defines a custom easing curve.
    ///
    /// If the easing curve does not have a custom bezier easing curve the list is empty.
    pub fn to_cubic_spline(&self) -> Vec<PointF> {
        unimplemented!()
    }

    /// Returns the type of the easing curve.
    pub fn get_type(&self) -> EasingCurveType {
        self.curve_type
    }

    /// Return the effective progress for the easing curve at `progress`.
    ///
    /// Whereas progress must be between 0 and 1, the returned effective progress
    /// can be outside those bounds. For example, `EasingCurveType::InBack` will return
    /// negative values in the beginning of the function.
    pub fn value_for_progress(&self, progress: f64) -> f64 {
        match self.curve_type {
            EasingCurveType::Linear => inner::ease_none(progress),
            EasingCurveType::InQuad => inner::ease_in_quad(progress),
            EasingCurveType::OutQuad => inner::ease_out_quad(progress),
            EasingCurveType::InOutQuad => inner::ease_in_out_quad(progress),
            EasingCurveType::OutInQuad => inner::ease_out_in_quad(progress),
            EasingCurveType::InCubic => inner::ease_in_cubic(progress),
            EasingCurveType::OutCubic => inner::ease_out_cubic(progress),
            EasingCurveType::InOutCubic => inner::ease_in_out_cubic(progress),
            EasingCurveType::OutInCubic => inner::ease_out_in_cubic(progress),
            EasingCurveType::InQuart => inner::ease_in_quart(progress),
            EasingCurveType::OutQuart => inner::ease_out_quart(progress),
            EasingCurveType::InOutQuart => inner::ease_in_out_quart(progress),
            EasingCurveType::OutInQuart => inner::ease_out_in_quart(progress),
            EasingCurveType::InQuint => inner::ease_in_quint(progress),
            EasingCurveType::OutQuint => inner::ease_out_quint(progress),
            EasingCurveType::InOutQuint => inner::ease_in_out_quint(progress),
            EasingCurveType::OutInQuint => inner::ease_out_in_quint(progress),
            EasingCurveType::InSine => inner::ease_in_sine(progress),
            EasingCurveType::OutSine => inner::ease_out_sine(progress),
            EasingCurveType::InOutSine => inner::ease_in_out_sine(progress),
            EasingCurveType::OutInSine => inner::ease_out_in_sine(progress),
            EasingCurveType::InExpo => inner::ease_in_expo(progress),
            EasingCurveType::OutExpo => inner::ease_out_expo(progress),
            EasingCurveType::InOutExpo => inner::ease_in_out_expo(progress),
            EasingCurveType::OutInExpo => inner::ease_out_in_expo(progress),
            EasingCurveType::InCirc => inner::ease_in_circ(progress),
            EasingCurveType::OutCirc => inner::ease_out_circ(progress),
            EasingCurveType::InOutCirc => inner::ease_in_out_circ(progress),
            EasingCurveType::OutInCirc => inner::ease_out_in_circ(progress),
            EasingCurveType::InCurve => inner::ease_in_curve(progress),
            EasingCurveType::OutCurve => inner::ease_out_curve(progress),
            EasingCurveType::SineCurve => inner::ease_sine_curve(progress),
            EasingCurveType::CosineCurve => inner::ease_cosine_curve(progress),

            // Complex curves.
            EasingCurveType::InBack
            | EasingCurveType::OutBack
            | EasingCurveType::InOutBack
            | EasingCurveType::OutInBack => self.back_ease_value(progress),

            EasingCurveType::InBounce
            | EasingCurveType::OutBounce
            | EasingCurveType::InOutBounce
            | EasingCurveType::OutInBounce => self.bounce_ease_value(progress),

            EasingCurveType::InElastic
            | EasingCurveType::OutElastic
            | EasingCurveType::InOutElastic
            | EasingCurveType::OutInElastic => self.elastic_ease_value(progress),

            _ => progress,
        }
    }
}

impl EasingCurve {
    /// Get value for back easing curves.
    fn back_ease_value(&self, progress: f64) -> f64 {
        // The *back() functions are not always precise on the endpoints, so handle explicitly
        if progress < 0.0 {
            return 0.0;
        }
        if progress > 1.0 {
            return 1.0;
        }
        let overshoot = if self.overshoot < 0.0 {
            DEFAULT_OVERSHOOT
        } else {
            self.overshoot
        };

        match self.curve_type {
            EasingCurveType::InBack => inner::ease_in_back(progress, overshoot),
            EasingCurveType::OutBack => inner::ease_out_back(progress, overshoot),
            EasingCurveType::InOutBack => inner::ease_in_out_back(progress, overshoot),
            EasingCurveType::OutInBack => inner::ease_out_in_back(progress, overshoot),
            _ => progress,
        }
    }

    /// Get value for bounce easing curves.
    fn bounce_ease_value(&self, progress: f64) -> f64 {
        let amplitude = if self.amplitude < 0.0 {
            1.0
        } else {
            self.amplitude
        };
        match self.curve_type {
            EasingCurveType::InBounce => inner::ease_in_bounce(progress, amplitude),
            EasingCurveType::OutBounce => inner::ease_out_bounce(progress, amplitude),
            EasingCurveType::InOutBounce => inner::ease_in_out_bounce(progress, amplitude),
            EasingCurveType::OutInBounce => inner::ease_out_in_bounce(progress, amplitude),
            _ => progress,
        }
    }

    /// Get value for elastic easing curves.
    fn elastic_ease_value(&self, progress: f64) -> f64 {
        let period = if self.period < 0.0 {
            DEFAULT_PERIOD
        } else {
            self.period
        };
        let amplitude = if self.amplitude < 0.0 {
            DEFAULT_AMPLITUDE
        } else {
            self.amplitude
        };
        match self.curve_type {
            EasingCurveType::InElastic => inner::ease_in_elastic(progress, amplitude, period),
            EasingCurveType::OutElastic => inner::ease_out_elastic(progress, amplitude, period),
            EasingCurveType::InOutElastic => {
                inner::ease_in_out_elastic(progress, amplitude, period)
            }
            EasingCurveType::OutInElastic => {
                inner::ease_out_in_elastic(progress, amplitude, period)
            }
            _ => progress,
        }
    }
}
