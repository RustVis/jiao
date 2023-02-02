// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

#![deny(
    warnings,
    clippy::all,
    clippy::cargo,
    clippy::nursery,
    clippy::pedantic
)]

pub mod paint_context;
pub mod paint_device;
pub mod painter;

pub use self::paint_context::PaintContext;
pub use self::paint_device::{ImagePaintDevice, PaintDevice, SvgPaintDevice};
pub use self::painter::Painter;
pub use self::painter::Path;
