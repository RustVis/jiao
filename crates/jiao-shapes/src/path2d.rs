// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

cfg_if::cfg_if! {
    if #[cfg(feature = "cairo")] {
        pub type Path2D = jiao_cairo::Path;
    } else if #[cfg(feature = "qt")] {
        pub type Path2D = jiao_qt::Path;
    } else if #[cfg(feature = "skia")] {
        pub type Path2D = jiao_skia::Path;
    } else if #[cfg(feature = "web")] {
        pub type Path2D = jiao_web::Path;
    } else {
        pub type Path2D = jiao::kernel::generic_path::GenericPath;
    }
}
