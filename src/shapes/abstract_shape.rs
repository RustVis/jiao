// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

use crate::kernel::PainterTrait;

pub trait AbstractShape {
    fn update(&mut self, painter: &mut dyn PainterTrait);
}
