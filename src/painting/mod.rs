// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

pub mod bezier;
pub mod brush;
pub mod color;
pub mod color_constants;
pub mod color_matrix;
pub mod font;
pub mod gradient;
pub mod linear_gradient;
pub mod palette;
pub mod path_clipper;
pub mod path_simplifier;
pub mod polygon;
pub mod radial_gradient;
pub mod region;
mod rgb;
mod rgba64;
pub mod transform;

pub use color::Color;
pub use palette::ColorPalette;
pub use rgb::{Rgb, RGB_MASK};
pub use rgba64::Rgba64;
