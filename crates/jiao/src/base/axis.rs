// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

/// This enum type defines three values to represent the three axes in the cartesian coordinate system.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Axis {
    /// The X axis.
    X = 0,

    /// The Y axis.
    Y = 1,

    /// The Z axis.
    Z = 2,
}
