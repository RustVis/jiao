// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

//! Jiao is a cross platform 2D painting engine.
//!
//! - [Documentation](https://docs.rs/jiao)
//! - [Release notes](https://github.com/RustVis/jiao/releases)
//!
//! Currently these bindings are supported:
//! - cairo (and gtk)
//! - skia (via rust-skia crate)
//! - Qt
//! - web canvas (wasm)
//!
//! ## Usage
//! Add this to `Cargo.toml`:
//! ```toml
//! [dependencies]
//! jiao = "0.3"
//! ```

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
pub mod error;
pub mod event;
pub mod interpolate;
pub mod kernel;
pub mod math3d;
pub mod painting;
pub mod platforms;
pub mod shapes;
pub mod util;
