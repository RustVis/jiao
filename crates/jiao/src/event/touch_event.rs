// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

#![allow(clippy::struct_excessive_bools)]

/// The `TouchEvent` interface represents an event which is sent when the state
/// of contacts with a touch-sensitive surface changes.
#[derive(Debug, Clone, PartialEq)]
pub struct TouchEvent {
    changed_touches: TouchList,
    target_touches: TouchList,
    touches: TouchList,

    alt_key: bool,
    ctrl_key: bool,
    meta_key: bool,
    shift_key: bool,
}

impl TouchEvent {
    /// Indicating whether or not the alt key was down when the touch event was fired.
    #[inline]
    #[must_use]
    pub const fn alt_key(&self) -> bool {
        self.alt_key
    }

    /// Indicating whether or not the control key was down when the touch event was fired.
    #[inline]
    #[must_use]
    pub const fn ctrl_key(&self) -> bool {
        self.ctrl_key
    }

    /// Indicating whether or not the meta key is enabled when the touch event is created.
    #[inline]
    #[must_use]
    pub const fn meta_key(&self) -> bool {
        self.meta_key
    }

    /// Indicating whether or not the shift key is enabled when the touch event is created.
    #[inline]
    #[must_use]
    pub const fn shift_key(&self) -> bool {
        self.shift_key
    }

    #[must_use]
    pub const fn changed_touches(&self) -> &TouchList {
        &self.changed_touches
    }

    #[must_use]
    pub const fn target_touches(&self) -> &TouchList {
        &self.target_touches
    }

    #[must_use]
    pub const fn touches(&self) -> &TouchList {
        &self.touches
    }
}

pub type TouchList = Vec<Touch>;

/// `Touch` represents a single contact point on a touch-sensitive device.
///
/// The contact point is commonly a finger or stylus and the device
/// may be a touchscreen or trackpad.
#[derive(Debug, Clone, PartialEq)]
pub struct Touch {
    identifier: i32,
    client_x: i32,
    client_y: i32,
    page_x: i32,
    page_y: i32,
    screen_x: i32,
    screen_y: i32,

    radius_x: f64,
    radius_y: f64,
    rotation_angle: f64,
    force: f32,
}

impl Touch {
    /// Returns the X coordinate of the touch point relative to the viewport,
    /// not including any scroll offset.
    #[inline]
    #[must_use]
    pub const fn client_x(&self) -> i32 {
        self.client_x
    }

    /// Returns the Y coordinate of the touch point relative to the viewport,
    /// not including any scroll offset.
    #[inline]
    #[must_use]
    pub const fn client_y(&self) -> i32 {
        self.client_y
    }

    /// The X (horizontal) coordinate of the touch point,
    /// relative to the left edge of the entire document or top window.
    #[inline]
    #[must_use]
    pub const fn page_x(&self) -> i32 {
        self.page_x
    }

    /// The Y (vertical) coordinate of the touch point,
    /// relative to the left edge of the entire document or top window.
    #[inline]
    #[must_use]
    pub const fn page_y(&self) -> i32 {
        self.page_y
    }

    /// The horizontal coordinate (offset) of the touch pointer in global (screen) coordinates.
    #[inline]
    #[must_use]
    pub const fn screen_x(&self) -> i32 {
        self.screen_x
    }

    /// The vertical coordinate (offset) of the touch pointer in global (screen) coordinates.
    #[inline]
    #[must_use]
    pub const fn screen_y(&self) -> i32 {
        self.screen_y
    }

    /// Returns the X radius of the ellipse that most closely circumscribes the area
    /// of contact with the touch surface.
    #[inline]
    #[must_use]
    pub const fn radius_x(&self) -> f64 {
        self.radius_x
    }

    /// Returns the Y radius of the ellipse that most closely circumscribes the area
    /// of contact with the touch surface.
    #[inline]
    #[must_use]
    pub const fn radius_y(&self) -> f64 {
        self.radius_y
    }

    /// Returns the rotation angle, in degrees, of the contact area ellipse
    /// defined by `radius_x` and `radius_y`.
    ///
    /// The value may be between 0 and 90.
    ///
    /// Together, these three values describe an ellipse that approximates the size
    /// and shape of the area of contact between the user and the screen.
    /// This may be a relatively large ellipse representing the contact
    /// between a fingertip and the screen or a small area representing the tip
    /// of a stylus, for example.
    #[inline]
    #[must_use]
    pub const fn rotation_angle(&self) -> f64 {
        self.rotation_angle
    }

    /// The amount of pressure the user is applying to the touch surface for a touch point.
    #[inline]
    #[must_use]
    pub const fn force(&self) -> f32 {
        self.force
    }
}
