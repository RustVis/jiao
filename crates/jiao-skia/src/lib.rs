// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

pub mod paint_device;
pub mod painter;
pub mod path;
mod util;

use jiao::platforms::Features;

pub use self::paint_device::{ImagePaintDevice, PaintDevice, SvgPaintDevice};
pub use self::painter::Painter;
pub use self::path::Path;

pub const FEATURES: &Features = &Features {
    filter: false,
    ssr: true,
    input_event: false,
};
