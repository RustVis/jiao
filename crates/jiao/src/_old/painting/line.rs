// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that can be found
// in the LICENSE file.

//! To store painter properties.

/// Specifies how to render the endpoints of the path when stroking.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LineCap {
    /// The ends of lines are squared off at the endpoints. Default value.
    Butt,

    /// The ends of lines are rounded.
    Round,

    /// The ends of lines are squared off by adding a box with an equal width
    /// and half the height of the line's thickness.
    Square,
}

/// Specifies how to render the junction of two lines when stroking.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LineJoin {
    /// Connected segments are joined by extending their outside edges
    /// to connect at a single point, with the effect of filling an additional
    /// lozenge-shaped area. This setting is affected by the miterLimit property.
    ///
    /// Default value.
    Miter,

    /// Rounds off the corners of a shape by filling an additional sector
    /// of disc centered at the common endpoint of connected segments.
    ///
    /// The radius for these rounded corners is equal to the line width.
    Round,

    /// Fills an additional triangular area between the common endpoint
    /// of connected segments, and the separate outside rectangular corners of each segment.
    Bevel,
}
