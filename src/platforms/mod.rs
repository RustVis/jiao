// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

cfg_if::cfg_if! {
    if #[cfg(feature = "web")] {
        pub mod web;
        pub type PaintContext = web::paint_context::PaintContext;
        pub type PaintDevice = web::paint_device::PaintDevice;
        pub type Painter = web::painter::Painter;
        pub type Path = web::painter::Path;
    } else if #[cfg(feature = "skia")] {
        pub mod skia;
        pub type PaintContext = skia::paint_context::PaintContext;
        pub type PaintDevice = skia::paint_device::PaintDevice;
        pub type Painter = skia::painter::Painter;
        pub type Path = skia::painter::Path;
    }
}
