// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

use crate::kernel::generic_path::{GenericPath, GenericPathToken};
use crate::kernel::{PainterTrait, PathTrait};

// Re-export GenericPath as Path
pub type Path = GenericPath;

#[derive(Debug, Clone)]
pub struct Painter {
    context: cairo::Context,
}

impl Painter {
    /// # Panics
    /// Got panic if failed to create a new cairo context with specific `surface`.
    #[must_use]
    pub fn new(surface: &cairo::Surface) -> Self {
        let context = cairo::Context::new(surface).unwrap();
        Self { context }
    }

    fn draw_path(&mut self, path: &Path) {
        for token in path.tokens() {
            match token {
                GenericPathToken::ClosePath => break,
                GenericPathToken::MoveTo(point) => {
                    self.context.move_to(point.x(), point.y());
                }
                GenericPathToken::LineTo(point) => {
                    self.context.line_to(point.x(), point.y());
                }
                GenericPathToken::Rect(rect) => {
                    self.context
                        .rectangle(rect.x(), rect.y(), rect.width(), rect.height());
                }
                GenericPathToken::CubicTo(_cubic_to) => {
                    todo!()
                }
                GenericPathToken::QuadTo(_quad_to) => {
                    todo!()
                }
                GenericPathToken::Arc(_arc) => {
                    todo!()
                }
                GenericPathToken::ArcTo(_arc_to) => {
                    todo!()
                }
                GenericPathToken::Ellipse(_ellipse) => {
                    todo!()
                }
            }
        }
    }
}

impl PainterTrait for Painter {
    #[inline]
    fn save(&mut self) {
        // TODO(Shaohua): Catch errors
        let _ = self.context.save();
    }

    #[inline]
    fn restore(&mut self) {
        // TODO(Shaohua): Catch errors
        let _ = self.context.restore();
    }

    fn clear_all(&mut self) {
        // TODO(Shaohua): Set color first.
        // TODO(Shaohua): Catch errors
        let _ = self.context.paint();
    }

    #[inline]
    fn clip(&mut self) {
        self.context.clip();
    }

    #[inline]
    fn fill(&mut self, path: &dyn PathTrait) {
        let path_ref = path.as_any().downcast_ref::<Path>().unwrap();
        self.draw_path(path_ref);
        // TODO(Shaohua): Catch errors
        let _ = self.context.fill();
    }

    #[inline]
    fn stroke(&mut self, path: &dyn PathTrait) {
        let path_ref = path.as_any().downcast_ref::<Path>().unwrap();
        self.draw_path(path_ref);
        // TODO(Shaohua): catch errors
        let _ = self.context.stroke();
    }

    #[inline]
    fn rotate(&mut self, angle: f64) {
        self.context.rotate(angle);
    }

    #[inline]
    fn scale(&mut self, x: f64, y: f64) {
        self.context.scale(x, y);
    }

    #[inline]
    fn translate(&mut self, x: f64, y: f64) {
        self.context.translate(x, y);
    }
}
