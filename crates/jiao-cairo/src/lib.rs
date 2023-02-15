// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

pub mod direct_paint_context;
pub mod direct_painter;
pub mod error;
pub mod paint_context;
pub mod paint_device;
pub mod painter;

use jiao::platforms::Features;

pub use self::direct_paint_context::DirectPaintContext;
pub use self::direct_painter::DirectPainter;
pub use self::error::CairoError;
pub use self::paint_context::PaintContext;
pub use self::paint_device::{ImagePaintDevice, PaintDevice, PdfPaintDevice, SvgPaintDevice};
pub use self::painter::Painter;
pub use self::painter::Path;

pub const FEATURES: &Features = &Features {
    filter: false,
    ssr: true,
    input_event: true,
};
