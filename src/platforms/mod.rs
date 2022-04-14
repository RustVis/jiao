// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

cfg_if::cfg_if! {
    if #[cfg(feature = "cairo")] {
        pub mod cr;
        pub use cr::paint_context::PaintContext;
        pub use cr::paint_device::{
            PaintDevice,
            ImagePaintDevice,
            SvgPaintDevice,
            PdfPaintDevice
        };
        pub use cr::painter::Painter;
        pub use cr::painter::Path;
    } else if #[cfg(feature = "qt")] {
        pub mod qt;
        pub use qt::paint_context::PaintContext;
        pub use qt::painter::Painter;
        pub use qt::painter::Path;
    } else if #[cfg(feature = "skia")] {
        pub mod skia;
        pub use skia::paint_context::PaintContext;
        pub use skia::paint_device::PaintDevice;
        pub use skia::painter::Painter;
        pub use skia::painter::Path;
    } else if #[cfg(feature = "web")] {
        pub mod web;
        pub use web::paint_context::PaintContext;
        pub use web::paint_device::PaintDevice;
        pub use web::painter::Painter;
        pub use web::painter::Path;
    }
}
