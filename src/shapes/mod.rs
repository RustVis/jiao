// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

#![allow(clippy::module_name_repetitions)]

pub mod bracket;
pub mod circle;
pub mod droplet;
pub mod ellipse;
pub mod grid;
pub mod line;
mod path2d;
pub mod polygon;
pub mod rect;
pub mod rounded_rect;
pub mod star;
pub mod text;

pub use bracket::BracketShape;
pub use circle::CircleShape;
pub use droplet::DropletShape;
pub use ellipse::EllipseShape;
pub use grid::GridShape;
pub use line::LineShape;
use path2d::Path2D;
pub use polygon::PolygonShape;
pub use rect::RectShape;
pub use rounded_rect::RoundedRectShape;
pub use star::StarShape;
pub use text::TextShape;
