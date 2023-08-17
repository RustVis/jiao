// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

pub mod alpha_type;
pub mod annotation;
//pub mod bitmap;
pub mod blend_mode;
pub mod blur_types;
pub mod canvas;
pub mod clip_op;
pub mod color;
pub mod color_space;
//pub mod color_table;
pub mod capabilities;
pub mod color_type;
pub mod data;
pub mod flattenable;
pub mod font_arguments;
pub mod font_metrics;
pub mod font_parameters;
pub mod font_style;
pub mod font_types;
pub mod image_info;
pub mod irect;
pub mod m44;
pub mod matrix;
pub mod paint;
pub mod paint_types;
pub mod path;
pub mod path_builder;
pub mod path_effect;
pub mod path_types;
pub mod pathops;
pub mod pixel_ref;
//pub mod pixmap;
pub mod point;
pub mod point3;
pub mod rect;
pub mod rrect;
pub mod scalar;
pub mod size;
pub mod surface_props;
pub mod texture_compression_type;
pub mod tile_mode;
pub mod types;
pub mod vertices;

// Private modules
pub(crate) mod cubic_clipper;
pub(crate) mod line_clipper;
pub(crate) mod mask;
pub(crate) mod path_builder_priv;
