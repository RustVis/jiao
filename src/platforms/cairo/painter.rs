// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

use crate::error::Error;
use crate::kernel::generic_path::{GenericPath, GenericPathToken};
use crate::kernel::{PainterTrait, PathTrait};

// Re-export GenericPath as Path
pub type Path = GenericPath;

#[derive(Debug, Clone)]
pub struct Painter {
    context: cairo::Context,
}

impl Painter {
    /// # Errors
    /// Returns error if failed to create a new cairo context with specific `surface`.
    #[must_use]
    pub fn new(surface: &cairo::Surface) -> Result<Self, Error> {
        let context = cairo::Context::new(surface)?;
        Ok(Self { context })
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
                GenericPathToken::CubicTo(cubic) => {
                    self.context.curve_to(
                        cubic.p1.x(),
                        cubic.p1.y(),
                        cubic.p2.x(),
                        cubic.p2.y(),
                        cubic.end_point.x(),
                        cubic.end_point.y(),
                    );
                }
                GenericPathToken::QuadTo(quad) => {
                    self.context.curve_to(
                        quad.control_point.x(),
                        quad.control_point.y(),
                        quad.control_point.x(),
                        quad.control_point.y(),
                        quad.end_point.x(),
                        quad.end_point.y(),
                    );
                }
                GenericPathToken::Arc(_arc) => {
                    todo!()
                }
                GenericPathToken::ArcTo(_arc_to) => {
                    todo!()
                }
                GenericPathToken::Ellipse(ellipse) => {
                    debug_assert!(ellipse.radius_y > 0.0);
                    let scale = ellipse.radius_x / ellipse.radius_y;
                    self.context.scale(1.0, scale);
                    self.context.arc()
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
