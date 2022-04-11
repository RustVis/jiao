// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

use crate::base::{PointF, RectF, SizeF};
use crate::kernel::{PainterTrait, PathTrait};
use crate::platforms::Path;
use crate::shapes::ShapeTrait;

#[derive(Debug, Clone)]
pub struct BracketShape {
    size: SizeF,
    handle_base: Option<PointF>,
    handle_width: f64,
    corner_radius: f64,

    path_is_dirty: bool,
    path: Path,
}

impl BracketShape {
    /// Create a new bracket.
    pub fn new(
        size: SizeF,
        handle_base: Option<PointF>,
        handle_width: f64,
        corner_radius: f64,
    ) -> Self {
        assert!(handle_width >= 0.0);
        assert!(corner_radius >= 0.0);
        let path = Path::new();
        Self {
            size,
            handle_base,
            handle_width,
            corner_radius,
            path_is_dirty: true,
            path,
        }
    }

    /// Create a new square bracket.
    pub fn new_square_bracket() -> Self {
        let size = SizeF::from(18.0, 124.0);
        Self::new(size, None, 0.0, 0.0)
    }

    /// Create a new rounded square bracket.
    pub fn new_rounded_square_bracket() -> Self {
        let size = SizeF::from(18.0, 124.0);
        Self::new(size, None, 0.0, 17.28)
    }

    /// Create a new indented square bracket.
    pub fn new_indented_square_bracket() -> Self {
        let size = SizeF::from(30.0, 124.0);
        let handle_base = PointF::from(-30.0, 124.0 / 2.0);
        Self::new(size, Some(handle_base), 0.0, 0.0)
    }

    /// Create a new straight curly bracket.
    pub fn new_straight_curly_bracket() -> Self {
        let size = SizeF::from(30.0, 124.0);
        let handle_base = PointF::from(-30.0, 124.0 / 2.0);
        Self::new(size, Some(handle_base), 16.0, 0.0)
    }

    /// Create a new curly bracket.
    pub fn new_curly_bracket() -> Self {
        let size = SizeF::from(30.0, 124.0);
        let handle_base = PointF::from(-30.0, 124.0 / 2.0);
        Self::new(size, Some(handle_base), 16.0, 17.28)
    }

    fn update_path(&mut self) {
        if !self.path_is_dirty {
            return;
        }
        self.path.clear();
        // TODO(Shaohua): Draw bracket.
        self.path_is_dirty = false;
    }
}

impl ShapeTrait for BracketShape {
    fn bounding_rect(&self) -> RectF {
        todo!()
    }

    fn repaint(&mut self, painter: &mut dyn PainterTrait) {
        self.update_path();
        painter.stroke(&self.path);
    }
}
