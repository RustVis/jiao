// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

cfg_if::cfg_if! {
    if #[cfg(feature = "cairo")] {
        mod cairo;
        pub use cairo::CairoPaintContext;

        pub use jiao_cairo::{
            PaintDevice,
            ImagePaintDevice,
            SvgPaintDevice,
            PdfPaintDevice,
            Painter,
            Path,
        };
    } else if #[cfg(feature = "qt")] {
        mod qt;
        pub use qt::QtPaintContext;

        pub use jiao_qt::{Painter, Path,};
    } else if #[cfg(feature = "skia")] {
        mod skia;
        pub use skia::SkiaPaintContext;

        pub use jiao_skia::{
            ImagePaintDevice,
            PaintDevice,
            SvgPaintDevice,
            Painter,
            Path,
        };
    } else if #[cfg(feature = "web")] {
        mod web;
        pub use web::WebPaintContext;

        pub use jiao_web::{PaintDevice, Painter, Path};
    } else {
        pub type Path = jiao::kernel::generic_path::GenericPath;
        use jiao::platforms::Features;
        pub const FEATURES: &Features = &Features {
            filter: false,
            ssr: false,
            input_event: false,
        };
    }
}
