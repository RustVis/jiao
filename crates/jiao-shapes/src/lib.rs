// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

#![allow(clippy::module_name_repetitions)]

pub mod error;
mod paint_context;
pub mod platforms;
mod shape_manager;
mod shape_trait;
mod shapes;

pub use self::paint_context::PaintContextTrait;
pub use self::shape_manager::ShapeManager;
pub use self::shape_trait::ShapeTrait;
pub use self::shapes::*;
