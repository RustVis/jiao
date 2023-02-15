// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

pub mod generic_path;
mod paint_context;
mod painter;

pub use self::paint_context::PaintContextTrait;
pub use self::painter::{PainterTrait, PathTrait};
