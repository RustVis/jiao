// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

#![allow(clippy::module_name_repetitions)]

pub mod error;
pub mod platforms;
mod shape_manager;
mod shape_trait;

pub use self::shape_manager::ShapeManager;
pub use self::shape_trait::ShapeTrait;
