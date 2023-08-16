// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

// TODO(Shaohua): Make Flattenable inherit from Serialize and Deserialize
//use serde::{Deserialize, Serialize};

pub enum Type {
    ColorFilter,
    Blender,
    Drawable,
    DrawLooper, // deprecated, no longer used internally
    ImageFilter,
    MaskFilter,
    PathEffect,
    Shader,
}

/// Flattenable is the trait for objects that need to be flattened
/// into a data stream for either transport or as part of the key to the font cache.
pub trait Flattenable {
    fn get_flattenable_type(&self) -> Type;
}
