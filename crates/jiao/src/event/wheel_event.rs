// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

/// The `WheelEvent` interface represents events that occur due to the user
/// moving a mouse wheel or similar input device.
#[derive(Debug, Clone, PartialEq)]
pub struct WheelEvent {
    /// Returns a double representing the horizontal scroll amount.
    delta_x: f64,

    /// Returns a double representing the vertical scroll amount.
    delta_y: f64,

    /// Returns a double representing the scroll amount for the z-axis.
    delta_z: f64,
}

impl WheelEvent {
    #[inline]
    #[must_use]
    pub const fn new(delta_x: f64, delta_y: f64, delta_z: f64) -> Self {
        Self {
            delta_x,
            delta_y,
            delta_z,
        }
    }

    #[inline]
    #[must_use]
    pub const fn delta_x(&self) -> f64 {
        self.delta_x
    }

    #[inline]
    #[must_use]
    pub const fn delta_y(&self) -> f64 {
        self.delta_y
    }

    #[inline]
    #[must_use]
    pub const fn delta_z(&self) -> f64 {
        self.delta_z
    }
}
