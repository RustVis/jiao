// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

#![allow(clippy::cast_possible_truncation)]

use skia_safe::PaintStyle;
use std::cell::RefCell;
use std::rc::Rc;

use super::Path;
use crate::kernel::{PainterTrait, PathTrait};

#[derive(Debug)]
enum CanvasWrapper {
    Surface(skia_safe::Surface),
    SvgCanvas(Rc<RefCell<skia_safe::svg::Canvas>>),
}

#[derive(Debug)]
pub struct Painter {
    canvas: CanvasWrapper,
    paint: skia_safe::Paint,
}

impl Painter {
    #[must_use]
    pub fn from_surface(surface: skia_safe::Surface) -> Self {
        let mut paint = skia_safe::Paint::default();
        paint.set_anti_alias(true);
        let canvas = CanvasWrapper::Surface(surface);
        Self { canvas, paint }
    }

    pub fn from_svg_canvas(canvas: Rc<RefCell<skia_safe::svg::Canvas>>) -> Self {
        let mut paint = skia_safe::Paint::default();
        paint.set_anti_alias(true);
        let canvas = CanvasWrapper::SvgCanvas(canvas);
        Self { canvas, paint }
    }

    fn canvas(&mut self) -> &mut skia_safe::Canvas {
        match &mut self.canvas {
            CanvasWrapper::Surface(surface) => surface.canvas(),
            CanvasWrapper::SvgCanvas(_svg_canvas) => todo!(),
        }
    }
}

impl PainterTrait for Painter {
    #[inline]
    fn save(&mut self) {
        self.canvas().save();
    }

    #[inline]
    fn restore(&mut self) {
        self.canvas().restore();
    }

    fn clear_all(&mut self) {
        log::info!("Painter::clear_all()");
        self.canvas().clear(skia_safe::Color::WHITE);
    }

    #[inline]
    fn clip(&mut self) {
        todo!()
    }

    #[inline]
    fn fill(&mut self, path: &dyn PathTrait) {
        let path_ref = path.as_any().downcast_ref::<Path>().unwrap();
        self.paint.set_style(PaintStyle::Fill);
        match &mut self.canvas {
            CanvasWrapper::Surface(surface) => {
                surface.canvas().draw_path(path_ref.path(), &self.paint);
            }
            CanvasWrapper::SvgCanvas(_svg_canvas) => todo!(),
        }
    }

    #[inline]
    fn stroke(&mut self, path: &dyn PathTrait) {
        let path_ref = path.as_any().downcast_ref::<Path>().unwrap();
        self.paint.set_style(PaintStyle::Stroke);
        match &mut self.canvas {
            CanvasWrapper::Surface(surface) => {
                surface.canvas().draw_path(path_ref.path(), &self.paint);
            }
            CanvasWrapper::SvgCanvas(_svg_canvas) => todo!(),
        }
    }

    #[inline]
    fn rotate(&mut self, angle: f64) {
        self.canvas().rotate(angle as f32, None);
    }

    #[inline]
    fn scale(&mut self, sx: f64, sy: f64) {
        self.canvas().scale((sx as f32, sy as f32));
    }

    #[inline]
    fn translate(&mut self, point: PointF) {
        self.canvas()
            .translate((point.x() as f32, point.y() as f32));
    }
}
