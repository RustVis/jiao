// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by LGPL-3.0 License that can be found
// in the LICENSE file.

#[derive(Debug, Clone, Copy)]
pub struct Capabilities {}

impl Capabilities {
    #[must_use]
    pub const fn raster_backend() -> Self {
        Self {}
    }
}
