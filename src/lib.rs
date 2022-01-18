// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

mod color;
pub use color::Color;
mod point;
pub use point::{Point, PointF};
mod rect;
pub use rect::{Rect, RectF};

mod traits;
pub use traits::{Canvas, Path};

#[cfg(feature = "gtk")]
pub mod gtk_canvas;

#[cfg(feature = "qt")]
pub mod qt_canvas;
