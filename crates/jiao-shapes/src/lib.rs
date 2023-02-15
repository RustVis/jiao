// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

#![allow(clippy::module_name_repetitions)]

mod bracket;
mod circle;
mod donut;
mod droplet;
mod ellipse;
mod grid;
mod line;
mod platforms;
mod polygon;
mod rect;
mod round_rect;
mod shape_manager;
mod shape_trait;
mod star;
mod text;

pub use self::bracket::BracketShape;
pub use self::circle::CircleShape;
pub use self::donut::DonutShape;
pub use self::droplet::DropletShape;
pub use self::ellipse::EllipseShape;
pub use self::grid::GridShape;
pub use self::line::LineShape;
pub use self::polygon::PolygonShape;
pub use self::rect::RectShape;
pub use self::round_rect::RoundRectShape;
pub use self::shape_manager::ShapeManager;
pub use self::shape_trait::ShapeTrait;
pub use self::star::StarShape;
pub use self::text::TextShape;
