// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

//! Canvas provides an interface for drawing, and how the drawing is clipped and transformed.
//!
//! Canvas contains a stack of Matrix and clip values.

//! Canvas and Paint together provide the state to draw into Surface or `BaseDevice`.
//! Each Canvas draw call transforms the geometry of the object by the concatenation of all
//! Matrix values in the stack. The transformed geometry is clipped by the intersection
//! of all of clip values in the stack. The Canvas draw calls use Paint to supply drawing
//! state such as color, Typeface, text size, stroke width, Shader and so on.

//! To draw to a pixel-based destination, create raster surface or GPU surface.
//! Request Canvas from Surface to obtain the interface to draw.
//! Canvas generated by raster surface draws to memory visible to the CPU.
//! Canvas generated by GPU surface uses Vulkan or OpenGL to draw to the GPU.

//! To draw to a document, obtain Canvas from SVG canvas, document PDF, or `PictureRecorder`.
//! Document based Canvas and other Canvas subclasses reference `BaseDevice` describing the
//! destination.

//! Canvas can be constructed to draw to Bitmap without first creating raster surface.
//! This approach may be deprecated in the future.

use crate::core::image_info::ImageInfo;

pub struct Canvas {}

impl Canvas {
    /// Returns `ImageInfo` for Canvas.
    ///
    /// If Canvas is not associated with raster surface or GPU surface,
    /// returned `ColorType` is set to `ColorType::Unknown`.
    ///
    /// Returns dimensions and `ColorType` of Canvas
    #[must_use]
    pub fn image_info(&self) -> ImageInfo {
        unimplemented!()
    }
}
