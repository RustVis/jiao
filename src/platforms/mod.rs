// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

#[cfg(feature = "skia")]
pub mod skia;

#[cfg(feature = "web")]
pub mod web;

#[cfg(feature = "web")]
pub type Path = web::Path;

#[cfg(feature = "web")]
pub type Painter = web::Painter;
