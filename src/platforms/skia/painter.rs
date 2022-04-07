// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

use skia_safe::PaintStyle;

use crate::base::PointF;
use crate::kernel::{PainterTrait, PathTrait};

pub struct Painter {
    paint: skia_safe::Paint,
    surface: skia_safe::Surface,
}

impl Painter {
    pub fn new(surface: skia_safe::Surface) -> Self {
        let mut paint = skia_safe::Paint::default();
        paint.set_anti_alias(true);

        Self { paint, surface }
    }

    fn canvas(&mut self) -> &mut skia_safe::Canvas {
        &mut self.surface.canvas()
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
    fn fill(&mut self, path: &Path) {
        self.paint.set_style(PaintStyle::Fill);
        self.surface.canvas().draw_path(path.path(), &self.paint);
    }

    #[inline]
    fn stroke(&mut self, path: &Path) {
        self.paint.set_style(PaintStyle::Stroke);
        self.canvas().draw_path(path.path(), &self.paint);
    }

    #[inline]
    fn rotate(&mut self, angle: f64) {
        self.canvas().rotate(angle as f32, None);
    }

    #[inline]
    fn scale(&mut self, x: f64, y: f64) {
        self.canvas().scale((x as f32, y as f32));
    }

    #[inline]
    fn translate(&mut self, x: f64, y: f64) {
        self.canvas().translate((x as f32, y as f32));
    }
}

pub struct Path {
    p: skia_safe::Path,
}

impl Path {
    pub fn new() -> Self {
        let p = skia_safe::Path::new();
        Self { p }
    }

    pub fn path(&self) -> &skia_safe::Path {
        &self.p
    }
}

impl PathTrait for Path {
    #[inline]
    fn close(&mut self) {
        self.p.close();
    }

    #[inline]
    fn line_to(&mut self, point: PointF) {
        self.p.line_to((point.x() as f32, point.y() as f32));
    }

    #[inline]
    fn move_to(&mut self, point: PointF) {
        self.p.move_to((point.x() as f32, point.y() as f32));
    }
}
