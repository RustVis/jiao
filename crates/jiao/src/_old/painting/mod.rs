// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by LGPL-3.0 License that can be found
// in the LICENSE file.

#![allow(clippy::module_name_repetitions)]

pub mod bezier;
pub mod brush;
pub mod color;
pub mod color_constants;
pub mod color_matrix;
pub mod font;
pub mod gradient;
mod line;
pub mod linear_gradient;
pub mod matrix;
pub mod palette;
pub mod path_clipper;
pub mod path_simplifier;
pub mod radial_gradient;
pub mod region;
mod rgb;
mod rgba64;
pub mod text;
pub mod transform;

pub use self::color::Color;
pub use self::line::{LineCap, LineJoin};
pub use self::palette::ColorPalette;
pub use self::rgb::{Rgb, RGB_MASK};
pub use self::rgba64::Rgba64;
pub use self::text::TextAlign;
