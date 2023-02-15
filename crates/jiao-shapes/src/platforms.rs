// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

cfg_if::cfg_if! {
    if #[cfg(feature = "cairo")] {
        pub use jiao_cairo::{
            PaintContext,
            PaintDevice,
            ImagePaintDevice,
            SvgPaintDevice,
            PdfPaintDevice,
            Painter,
            Path,
        };
    } else if #[cfg(feature = "qt")] {
        pub use jiao_qt::{PaintContext, Painter, Path,};
    } else if #[cfg(feature = "skia")] {
        pub use jiao_skia::{
            PaintContext,
            ImagePaintDevice,
            PaintDevice,
            SvgPaintDevice,
            Painter,
            Path,
        };
    } else if #[cfg(feature = "web")] {
        pub use jiao_web::{PaintContext, PaintDevice, Painter, Path};
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
