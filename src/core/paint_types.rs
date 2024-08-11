// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that can be found
// in the LICENSE file.

/// `PaintStyle` set Style to fill, stroke, or both fill and stroke geometry.
///
/// The stroke and fill share all paint attributes; for instance,
/// they are drawn with the same color.
///
/// Use `StrokeAndFill` to avoid hitting the same pixels twice with a stroke draw
/// and a fill draw.
#[repr(u8)]
#[derive(Debug, Default, Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum PaintStyle {
    /// Set to fill geometry
    #[default]
    Fill,

    /// Set to stroke geometry
    Stroke,

    /// Sets to stroke and fill geometry
    StrokeAndFill,
}

/// `StrokeJoin` specifies how corners are drawn when a shape is stroked.
/// Join affects the four corners of a stroked rectangle, and the connected segments in a
/// stroked path.
///
/// Choose miter join to draw sharp corners.
/// Choose round join to draw a circle with a radius equal to the stroke width
/// on top of the corner.
/// Choose bevel join to minimally connect the thick strokes.
///
/// The fill path constructed to describe the stroked path respects the join setting
/// but may not contain the actual join.
/// For instance, a fill path constructed with round joins does not necessarily
/// include circles at each connected segment.
#[repr(u8)]
#[derive(Debug, Default, Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum StrokeJoin {
    /// Extends to miter limit
    #[default]
    Miter,

    /// Adds circle
    Round,

    /// connects outside edges
    Bevel,
}

/// `StrokeCap` draws at the beginning and end of an open path contour.
#[repr(u8)]
#[derive(Debug, Default, Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum StrokeCap {
    /// No stroke extension
    #[default]
    Butt,

    /// Adds circle
    Round,

    /// Adds square
    Square,
}
