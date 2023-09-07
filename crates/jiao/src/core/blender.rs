// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by LGPL-3.0 License that can be found
// in the LICENSE file.

use crate::core::blend_mode::BlendMode;
use crate::core::flattenable::Flattenable;

/// Blender represents a custom blend function in the pipeline.
///
/// When a Blender is present in a paint, the `BlendMode` is ignored.
/// A blender combines a source color (the result of our paint) and
/// destination color (from the canvas) into a final color.
pub struct Blender {}

impl Blender {
    /// Create a blender that implements the specified `BlendMode`.
    #[must_use]
    pub fn from_mode(_mode: BlendMode) -> Self {
        unimplemented!()
    }
}

impl Flattenable for Blender {}
