// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License
// that can be found in the LICENSE file.

use std::f64::consts::PI;

use crate::base::PointF;
use crate::kernel::generic_path::{GenericPath, GenericPathToken};
use crate::kernel::{PainterTrait, PathTrait};

// Re-export GenericPath as Path
pub type Path = GenericPath;

#[derive(Debug, Clone)]
pub struct DirectPainter<'a> {
    pub context: &'a cairo::Context,
}

impl<'a> DirectPainter<'a> {
    pub fn new(context: &'a cairo::Context) -> Self {
        Self { context }
    }

    fn draw_path(&mut self, path: &Path) {
        for token in path.tokens() {
            match token {
                GenericPathToken::ClosePath => {
                    self.context.close_path();
                }
                GenericPathToken::MoveTo(point) => {
                    self.context.move_to(point.x(), point.y());
                }
                GenericPathToken::LineTo(point) => {
                    self.context.line_to(point.x(), point.y());
                }
                GenericPathToken::AddRect(rect) => {
                    self.context
                        .rectangle(rect.x(), rect.y(), rect.width(), rect.height());
                }
                GenericPathToken::AddRoundRect(rrect) => {
                    let x = rrect.rect.x();
                    let y = rrect.rect.y();
                    let width = rrect.rect.width();
                    let height = rrect.rect.height();
                    let radius = rrect.radius;

                    self.context.new_sub_path();
                    self.context.arc(
                        x + width - radius,
                        y + radius,
                        radius,
                        -90.0_f64.to_radians(),
                        0.0_f64.to_radians(),
                    );
                    self.context.arc(
                        x + width - radius,
                        y + height - radius,
                        radius,
                        0.0_f64.to_radians(),
                        90.0_f64.to_radians(),
                    );
                    self.context.arc(
                        x + radius,
                        y + height - radius,
                        radius,
                        90.0_f64.to_radians(),
                        180.0_f64.to_radians(),
                    );
                    self.context.arc(
                        x + radius,
                        y + radius,
                        radius,
                        180.0_f64.to_radians(),
                        270.0_f64.to_radians(),
                    );
                    self.context.close_path();
                }
                GenericPathToken::AddCircle(circle) => {
                    self.context.arc(
                        circle.center.x(),
                        circle.center.y(),
                        circle.radius,
                        0.0,
                        2.0 * PI,
                    );
                }
                GenericPathToken::AddEllipse(rect) => {
                    debug_assert!(rect.height() > 0.0);
                    let center = rect.center();
                    let scale = rect.width() / rect.height();
                    let radius = rect.width();
                    self.context.scale(1.0, scale);
                    self.context
                        .arc(center.x(), center.y(), radius, 0.0, 2.0 * PI);
                }
                GenericPathToken::Arc(arc) => {
                    let center = arc.rect.center();
                    let scale = arc.rect.width() / arc.rect.height();
                    let radius = arc.rect.width();
                    self.context.scale(1.0, scale);
                    self.context.arc(
                        center.x(),
                        center.y(),
                        radius,
                        arc.start_angle,
                        arc.end_angle,
                    );
                }
                GenericPathToken::ArcTo(_arc_to) => {
                    todo!()
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
            }
        }
    }
}

impl<'a> PainterTrait for DirectPainter<'a> {
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
    fn scale(&mut self, sx: f64, sy: f64) {
        self.context.scale(sx, sy);
    }

    #[inline]
    fn translate(&mut self, point: PointF) {
        self.context.translate(point.x(), point.y());
    }
}
