// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that can be found
// in the LICENSE file.

//! Jiao is a cross platform 2D graphics library.
//!
//! - [Documentation](https://docs.rs/jiao)
//! - [Release notes](https://github.com/RustVis/jiao/releases)
//!
//! ## Usage
//! Add this to `Cargo.toml`:
//! ```toml
//! [dependencies]
//! jiao = "0.4"
//! ```

#![deny(
    warnings,
    clippy::all,
    clippy::cargo,
    clippy::nursery,
    clippy::pedantic
)]
#![allow(dead_code)]

pub(crate) mod base;
pub mod codec;
pub mod core;
pub mod effects;
pub mod encode;
pub mod gpu;
pub mod image;
pub mod pdf;
pub mod shaders;
pub mod sksl;
pub mod svg;
pub mod text;
pub mod utils;
