// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License
// that can be found in the LICENSE file.

use super::Path2D;
use crate::base::{PointF, RectF, SizeF};
use crate::kernel::{PainterTrait, PathTrait, ShapeTrait};

#[derive(Debug)]
pub struct DropletShape {
    center: PointF,
    size: SizeF,
    is_inverted: bool,

    path: Path2D,
    path_is_dirty: bool,
}

impl DropletShape {
    pub fn new(center: PointF, size: SizeF) -> Self {
        let path = Path2D::new();

        Self {
            center,
            size,
            is_inverted: false,
            path,
            path_is_dirty: true,
        }
    }

    pub fn from_rect(rect: &RectF) -> Self {
        Self::new(rect.center(), rect.size())
    }

    /// Get center point of the droplet shape.
    #[must_use]
    pub const fn center(&self) -> PointF {
        self.center
    }

    /// Update center point of the droplet shape.
    pub fn set_center(&mut self, center: PointF) {
        self.center = center;
        self.path_is_dirty = true;
    }

    /// Get size of the  shape.
    #[must_use]
    pub const fn size(&self) -> &SizeF {
        &self.size
    }

    /// Set radius of the droplet shape.
    pub fn set_size(&mut self, size: SizeF) {
        self.size = size;
        self.path_is_dirty = true;
    }

    fn update_path(&mut self) {
        if !self.path_is_dirty {
            return;
        }
        let x = self.center.x();
        let y = self.center.y();
        let w = self.size.width();
        let h = self.size.height();

        self.path.move_to(PointF::from(x, y + w));
        self.path.cubic_to(
            PointF::from(x + w, y + w),
            PointF::from(x + w * 3.0 / 2.0, y - w / 3.0),
            PointF::from(x, y - h),
        );
        self.path.cubic_to(
            PointF::from(x - w * 3.0 / 2.0, y - w / 3.0),
            PointF::from(x - w, y + w),
            PointF::from(x, y + w),
        );
        self.path.close_path();

        if self.is_inverted {
            // TODO(Shaohua): reflect
        }

        self.path_is_dirty = false;
    }
}

impl ShapeTrait for DropletShape {
    fn bounding_rect(&self) -> RectF {
        RectF::from_size(self.center, self.size)
    }

    fn repaint(&mut self, painter: &mut dyn PainterTrait) {
        self.update_path();
        painter.stroke(&self.path);
    }
}
