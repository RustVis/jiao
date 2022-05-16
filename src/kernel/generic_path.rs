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
    #[must_use]
    pub fn new() -> Self {
        Self { tokens: Vec::new() }
    }

    /// Get internal tokens.
    #[must_use]
    pub fn tokens(&self) -> &[GenericPathToken] {
        &self.tokens
    }
}

impl PathTrait for GenericPath {
    fn clear(&mut self) {
        self.tokens.clear();
    }

    fn add_path(&mut self, other: &Self) {
        // TODO(Shaohua): Check close-path is the last element or not.
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

    fn quad_to(&mut self, control_point: PointF, end_point: PointF) {
        self.tokens
            .push(GenericPathToken::QuadTo(GenericPathQuadTo {
                control_point,
                end_point,
            }));
    }

    fn arc(&mut self, center: PointF, radius: f64, start_angle: f64, end_angle: f64) {
        self.tokens.push(GenericPathToken::Arc(GenericPathArc {
            center,
            radius,
            start_angle,
            end_angle,
        }));
    }

    fn arc_to(&mut self, p1: PointF, p2: PointF, radius: f64) {
        self.tokens
            .push(GenericPathToken::ArcTo(GenericPathArcTo { p1, p2, radius }));
    }

    fn ellipse(
        &mut self,
        center: PointF,
        radius_x: f64,
        radius_y: f64,
        start_angle: f64,
        end_angle: f64,
    ) {
        self.tokens
            .push(GenericPathToken::Ellipse(GenericPathEllipse {
                center,
                radius_x,
                radius_y,
                start_angle,
                end_angle,
            }));
    }
}
