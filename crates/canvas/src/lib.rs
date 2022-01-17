// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

mod traits;
pub use traits::Canvas;

#[cfg(feature = "gtk")]
pub mod gtk_canvas;
