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

mod error;
pub mod paint_device;
pub mod painter;
pub mod path;

use jiao::platforms::Features;

pub use self::paint_device::PaintDevice;
pub use self::painter::Painter;
pub use self::path::Path;
pub use error::Error;

pub const FEATURES: &Features = &Features {
    filter: true,
    ssr: false,
    input_event: true,
};
