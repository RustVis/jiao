// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

cfg_if::cfg_if! {
    if #[cfg(feature = "cairo")] {
        pub mod cairo;
        pub use self::cairo::{
            PaintContext,
            PaintDevice,
            ImagePaintDevice,
            SvgPaintDevice,
            PdfPaintDevice,
            Painter,
            Path,
        };
    } else if #[cfg(feature = "qt")] {
        pub mod qt;
        pub use self::qt::{PaintContext, Painter, Path};
    } else if #[cfg(feature = "skia")] {
        pub mod skia;
        pub use self::skia::{
            PaintContext,
            ImagePaintDevice,
            PaintDevice,
            SvgPaintDevice,
            Painter,
            Path,
        };
    } else if #[cfg(feature = "web")] {
        pub mod web;
        pub use self::web::{PaintContext, PaintDevice, Painter, Path};
    } else {
        pub type Path = crate::kernel::generic_path::GenericPath;
    }
}
