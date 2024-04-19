// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that can be found
// in the LICENSE file.

pub mod alpha_type;
pub mod annotation;
pub mod bitmap;
pub mod blend_mode;
pub mod blur_types;
pub mod canvas;
pub mod capabilities;
pub mod clip_op;
pub mod color;
pub mod color_filter;
pub mod color_space;
pub mod color_table;
pub mod color_type;
pub mod contour_measure;
pub mod coverage_mode;
pub mod cubic_map;
pub mod data;
pub mod flattenable;
pub mod font_arguments;
pub mod sl_type_shared;
//pub mod font_manager;
pub mod advanced_typeface_metrics;
pub mod font_metrics;
pub mod font_parameters;
pub mod font_style;
pub mod font_types;
pub mod image_filter;
pub mod image_info;
pub mod irect;
pub mod m44;
pub mod mask_filter;
pub mod matrix;
pub mod milestone;
pub mod paint;
pub mod paint_types;
pub mod path;
pub mod path_builder;
pub mod path_effect;
pub mod path_types;
pub mod path_utils;
pub mod pathops;
pub mod pixel_ref;
pub mod pixmap;
pub mod point;
pub mod point3;
pub mod rect;
pub mod rrect;
pub mod sampling_options;
pub mod scalar;
pub mod size;
pub mod surface_props;
pub mod swizzle;
pub mod texture_compression_type;
pub mod tile_mode;
//pub mod typeface;
pub mod types;
pub mod vertices;
pub mod yuva_info;
pub mod yuva_pixmap;

// Private modules
pub(crate) mod color_priv;
pub(crate) mod cubic_clipper;
pub(crate) mod line_clipper;
pub(crate) mod mask;
pub(crate) mod mipmap;
pub(crate) mod path_builder_priv;