// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

use super::painter::PathTrait;
use crate::base::{PointF, RectF};

#[derive(Debug, Clone)]
pub struct GenericPathCubicTo {
    pub p1: PointF,
    pub p2: PointF,
    pub end_point: PointF,
}

#[derive(Debug, Clone)]
pub struct GenericPathQuadTo {
    pub control_point: PointF,
    pub end_point: PointF,
}

#[derive(Debug, Clone)]
pub struct GenericPathArc {
    pub center: PointF,
    pub radius: f64,
    pub start_angle: f64,
    pub end_angle: f64,
}

#[derive(Debug, Clone)]
pub struct GenericPathArcTo {
    pub p1: PointF,
    pub p2: PointF,
    pub radius: f64,
}

#[derive(Debug, Clone)]
pub struct GenericPathEllipse {
    pub center: PointF,
    pub radius_x: f64,
    pub radius_y: f64,
    pub rotation: f64,
    pub start_angle: f64,
    pub end_angle: f64,
}

#[derive(Debug, Clone)]
pub enum GenericPathToken {
    MoveTo(PointF),
    LineTo(PointF),
    Rect(RectF),
    CubicTo(GenericPathCubicTo),
    QuadTo(GenericPathQuadTo),
    Arc(GenericPathArc),
    ArcTo(GenericPathArcTo),
    Ellipse(GenericPathEllipse),
    ClosePath,
}

#[derive(Debug, Clone)]
pub struct GenericPath {
    tokens: Vec<GenericPathToken>,
}

impl GenericPath {
    /// Create an empty generic path object.
    pub fn new() -> Self {
        Self { tokens: Vec::new() }
    }

    /// Get internal tokens.
    pub fn tokens(&self) -> &[GenericPathToken] {
        &self.tokens
    }
}

impl PathTrait for GenericPath {
    fn clear(&mut self) {
        self.tokens.clear();
    }

    fn add_path(&mut self, other: &Self) {
        self.tokens.extend_from_slice(&other.tokens);
    }

    fn close_path(&mut self) {
        self.tokens.push(GenericPathToken::ClosePath);
    }

    fn move_to(&mut self, point: PointF) {
        self.tokens.push(GenericPathToken::MoveTo(point));
    }

    fn line_to(&mut self, point: PointF) {
        self.tokens.push(GenericPathToken::LineTo(point));
    }

    fn rect_f64(&mut self, x: f64, y: f64, width: f64, height: f64) {
        self.rect(&RectF::from(x, y, width, height));
    }

    fn rect(&mut self, rect: &RectF) {
        self.tokens.push(GenericPathToken::Rect(rect.clone()));
    }

    fn cubic_to(&mut self, p1: PointF, p2: PointF, end_point: PointF) {
        self.tokens
            .push(GenericPathToken::CubicTo(GenericPathCubicTo {
                p1,
                p2,
                end_point,
            }));
    }

    fn cubic_to_f64(&mut self, cp1x: f64, cp1y: f64, cp2x: f64, cp2y: f64, x: f64, y: f64) {
        self.cubic_to(
            PointF::from(cp1x, cp1y),
            PointF::from(cp2x, cp2y),
            PointF::from(x, y),
        );
    }

    fn quad_to(&mut self, control_point: PointF, end_point: PointF) {
        self.tokens
            .push(GenericPathToken::QuadTo(GenericPathQuadTo {
                control_point,
                end_point,
            }));
    }

    fn quad_to_f64(&mut self, cpx: f64, cpy: f64, x: f64, y: f64) {
        self.quad_to(PointF::from(cpx, cpy), PointF::from(x, y));
    }

    fn arc(&mut self, center: PointF, radius: f64, start_angle: f64, end_angle: f64) {
        self.tokens.push(GenericPathToken::Arc(GenericPathArc {
            center,
            radius,
            start_angle,
            end_angle,
        }));
    }

    fn arc_f64(&mut self, x: f64, y: f64, radius: f64, start_angle: f64, end_angle: f64) {
        self.arc(PointF::from(x, y), radius, start_angle, end_angle);
    }

    fn arc_to(&mut self, p1: PointF, p2: PointF, radius: f64) {
        self.tokens
            .push(GenericPathToken::ArcTo(GenericPathArcTo { p1, p2, radius }));
    }

    fn arc_to_f64(&mut self, x1: f64, y1: f64, x2: f64, y2: f64, radius: f64) {
        self.arc_to(PointF::from(x1, y1), PointF::from(x2, y2), radius);
    }

    fn ellipse(
        &mut self,
        center: PointF,
        radius_x: f64,
        radius_y: f64,
        rotation: f64,
        start_angle: f64,
        end_angle: f64,
    ) {
        self.tokens
            .push(GenericPathToken::Ellipse(GenericPathEllipse {
                center,
                radius_x,
                radius_y,
                rotation,
                start_angle,
                end_angle,
            }));
    }

    fn ellipse_f64(
        &mut self,
        center_x: f64,
        center_y: f64,
        radius_x: f64,
        radius_y: f64,
        rotation: f64,
        start_angle: f64,
        end_angle: f64,
    ) {
        self.ellipse(
            PointF::from(center_x, center_y),
            radius_x,
            radius_y,
            rotation,
            start_angle,
            end_angle,
        );
    }
}
