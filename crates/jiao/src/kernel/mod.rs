// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

pub mod generic_path;
pub mod paint_context;
mod painter;
mod shape_manager;
mod shape_trait;

pub use self::paint_context::PaintContextTrait;
pub use self::painter::{PainterTrait, PathTrait};
pub use self::shape_manager::ShapeManager;
pub use self::shape_trait::ShapeTrait;
