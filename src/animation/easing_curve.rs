// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

/// The type of easing curve.
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

    BezierSpline,
    TCBSpline,
    NCurveTypes,

    Custom,
}

impl Default for EasingCurveType {
    fn default() -> Self {
        EasingCurveType::Linear
    }
}
