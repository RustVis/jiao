// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

cfg_if::cfg_if! {
    if #[cfg(feature = "cairo")] {
        pub mod cairo;
        pub use self::cairo::paint_context::PaintContext;
        pub use self::cairo::paint_device::{
            PaintDevice,
            ImagePaintDevice,
            SvgPaintDevice,
            PdfPaintDevice
        };
        pub use self::cairo::painter::{Painter, Path};
    } else if #[cfg(feature = "qt")] {
        pub mod qt;
        pub use self::qt::paint_context::PaintContext;
        pub use self::qt::painter::{Painter, Path};
    } else if #[cfg(feature = "skia")] {
        pub mod skia;
        pub use self::skia::paint_context::PaintContext;
        pub use self::skia::paint_device::{
            ImagePaintDevice,
            PaintDevice,
            SvgPaintDevice,
        };
        pub use self::skia::painter::{Painter, Path};
    } else if #[cfg(feature = "web")] {
        pub mod web;
        pub use self::web::paint_context::PaintContext;
        pub use self::web::paint_device::PaintDevice;
        pub use self::web::painter::{Painter, Path};
    } else {
        pub type Path = crate::kernel::generic_path::GenericPath;
    }
}
