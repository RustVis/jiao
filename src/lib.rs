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
#![allow(clippy::manual_range_contains)]

pub mod animation;
pub mod base;
pub mod event;
pub mod interpolate;
pub mod kernel;
pub mod math3d;
pub mod painting;
pub mod platforms;
pub mod shapes;
pub mod util;
