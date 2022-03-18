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

    /// Easing equation function for a quadratic (t^2) easing in/out: acceleration until halfway, then deceleration.
    InOutQuad,

    /// Easing equation function for a cubic (t^3) easing in: accelerating from zero velocity.
    OutInQuad,

    /// Easing equation function for a cubic (t^3) easing out: decelerating to zero velocity.
    InCubic,

    /// Easing equation function for a cubic (t^3) easing out: decelerating to zero velocity.
    OutCubic,

    /// Easing equation function for a cubic (t^3) easing in/out: acceleration until halfway, then deceleration.
    InOutCubic,

    /// Easing equation function for a cubic (t^3) easing out/in: deceleration until halfway, then acceleration.
    OutInCubic,

    /// Easing equation function for a quartic (t^4) easing in: accelerating from zero velocity.
    InQuart,

    /// Easing equation function for a quartic (t^4) easing out: decelerating to zero velocity.
    OutQuart,

    /// Easing equation function for a quartic (t^4) easing in/out: acceleration until halfway, then deceleration.
    InOutQuart,

    /// Easing equation function for a quartic (t^4) easing out/in: deceleration until halfway, then acceleration.
    OutInQuart,

    /// Easing equation function for a quintic (t^5) easing in: accelerating from zero velocity.
    InQuint,

    /// Easing equation function for a quintic (t^5) easing out: decelerating to zero velocity.
    OutQuint,

    /// Easing equation function for a quintic (t^5) easing in/out: acceleration until halfway, then deceleration.
    InOutQuint,

    /// Easing equation function for a quintic (t^5) easing out/in: deceleration until halfway, then acceleration.
    OutInQuint,

    /// Easing equation function for a sinusoidal (sin(t)) easing in: accelerating from zero velocity.
    InSine,
    OutSine,
    InOutSine,
    OutInSine,
    InExpo,
    OutExpo,
    InOutExpo,
    OutInExpo,
    InCirc,
    OutCirc,
    InOutCirc,
    OutInCirc,
    InElastic,
    OutElastic,
    InOutElastic,
    OutInElastic,
    InBack,
    OutBack,
    InOutBack,
    OutInBack,
    InBounce,
    OutBounce,
    InOutBounce,
    OutInBounce,
    InCurve,
    OutCurve,
    SineCurve,
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
