// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

use jiao::base::RectF;
use jiao::kernel::PainterTrait;

pub trait ShapeTrait: std::fmt::Debug {
    /// Returns the bounding rectangle of this shape object as a rectangle with floating point precision.
    fn bounding_rect(&self) -> RectF;

    /// Repaint shape object.
    fn repaint(&mut self, painter: &mut dyn PainterTrait);
}
