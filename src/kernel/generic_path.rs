// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

#![allow(clippy::module_name_repetitions)]

use std::any::Any;

use super::painter::PathTrait;
use crate::base::{PointF, RectF};

#[derive(Debug, Clone)]
pub struct GenericPathRoundedRect {
    pub rect: RectF,
    pub radius: f64,
}

#[derive(Debug, Clone)]
pub struct GenericPathCircle {
    pub center: PointF,
    pub radius: f64,
}

#[derive(Debug, Clone)]
pub struct GenericPathArc {
    pub rect: RectF,
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
    AddRect(RectF),
    AddRoundedRect(GenericPathRoundedRect),
    AddCircle(GenericPathCircle),
    AddEllipse(RectF),
    Arc(GenericPathArc),
    ArcTo(GenericPathArcTo),
    CubicTo(GenericPathCubicTo),
    QuadTo(GenericPathQuadTo),
    ClosePath,
}

#[derive(Debug, Default, Clone)]
pub struct GenericPath {
    tokens: Vec<GenericPathToken>,
}

impl GenericPath {
    /// Create an empty generic path object.
    #[must_use]
    pub const fn new() -> Self {
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

    fn add_path(&mut self, other: &dyn PathTrait) {
        // TODO(Shaohua): Check close-path is the last element or not.
        let other_ref = other.as_any().downcast_ref::<Self>().unwrap();
        self.tokens.extend_from_slice(&other_ref.tokens);
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

    fn add_rect(&mut self, rect: &RectF) {
        self.tokens.push(GenericPathToken::AddRect(rect.clone()));
    }

    fn add_rounded_rect(&mut self, rect: &RectF, radius: f64) {
        self.tokens
            .push(GenericPathToken::AddRoundedRect(GenericPathRoundedRect {
                rect: rect.clone(),
                radius,
            }));
    }

    fn add_circle(&mut self, center: PointF, radius: f64) {
        self.tokens
            .push(GenericPathToken::AddCircle(GenericPathCircle {
                center,
                radius,
            }));
    }

    fn add_ellipse(&mut self, rect: &RectF) {
        self.tokens.push(GenericPathToken::AddEllipse(rect.clone()));
    }

    fn arc(&mut self, rect: &RectF, start_angle: f64, end_angle: f64) {
        self.tokens.push(GenericPathToken::Arc(GenericPathArc {
            rect: rect.clone(),
            start_angle,
            end_angle,
        }));
    }

    fn arc_to(&mut self, p1: PointF, p2: PointF, radius: f64) {
        self.tokens
            .push(GenericPathToken::ArcTo(GenericPathArcTo { p1, p2, radius }));
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

    fn as_any(&self) -> &dyn Any {
        self
    }
}
