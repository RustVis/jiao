// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

use crate::core::canvas::Canvas;
use crate::core::data::Data;
use crate::core::point::Point;
use crate::core::rect::Rect;

/// Annotate the canvas by associating the specified URL with the
/// specified rectangle (in local coordinates, just like `draw_rect`).
///
/// The URL is expected to be escaped and be valid 7-bit ASCII.
///
/// If the backend of this canvas does not support annotations, this call is safely ignored.
///
/// The caller is responsible for managing its ownership of the Data.
pub fn annotate_rect_with_url(_canvas: &mut Canvas, _rect: &Rect, _data: &mut Data) {
    unimplemented!()
}

/// Annotate the canvas by associating a name with the specified point.
///
/// If the backend of this canvas does not support annotations, this call is safely ignored.
///
/// The caller is responsible for managing its ownership of the Data.
pub fn annotate_named_destination(_canvas: &mut Canvas, _point: &Point, _data: &mut Data) {
    unimplemented!()
}

/// Annotate the canvas by making the specified rectangle link to a named destination.
///
/// If the backend of this canvas does not support annotations, this call is safely ignored.
///
/// The caller is responsible for managing its ownership of the Data.
pub fn annotate_link_to_destination(_canvas: &Canvas, _rect: &Rect, _data: &mut Data) {
    unimplemented!()
}
