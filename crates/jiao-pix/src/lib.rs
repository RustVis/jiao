// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by BSD-style License that can be found
// in the LICENSE file.

#![deny(
    warnings,
    clippy::all,
    clippy::cargo,
    clippy::nursery,
    clippy::pedantic
)]

pub mod r#box;
pub mod color;
pub mod filter;
pub mod fixpoint;
pub mod format;
pub mod image;
pub mod indexed;
pub mod kernel;
pub mod line;
pub mod op;
pub mod point;
//pub mod rectangle;
//pub mod region;
pub mod repeat;
pub mod transform;
//pub mod vector;
