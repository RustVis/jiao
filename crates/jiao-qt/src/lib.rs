// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

pub mod paint_context;
pub mod painter;
pub mod path;

use jiao::platforms::Features;

pub use self::paint_context::PaintContext;
pub use self::painter::Painter;
pub use self::path::Path;

pub const FEATURES: &Features = &Features {
    filter: true,
    ssr: true,
    /// Currently user input is not supported, as it's not so easy to override
    /// methods in QWidget in rust.
    input_event: false,
};
