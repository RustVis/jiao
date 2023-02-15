// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

use cpp_core::CppBox;
use jiao::base::PointF;
use jiao::kernel::{PainterTrait, PathTrait};
use qt_core::{QPointF, QString};
use qt_gui::q_painter::RenderHint;
use qt_gui::QPainter;
use std::fmt;

use super::Path;

pub struct Painter {
    painter: CppBox<QPainter>,
}

impl Default for Painter {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Debug for Painter {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Painter").finish()
    }
}

impl Painter {
    #[must_use]
    pub fn new() -> Self {
        let painter = unsafe { QPainter::new_0a() };

        Self { painter }
    }

    pub fn painter(&mut self) -> &CppBox<QPainter> {
        &self.painter
    }

    pub fn set_default_hints(&mut self) {
        unsafe {
            self.painter
                .set_render_hint_2a(RenderHint::Antialiasing, true);
            self.painter
                .set_render_hint_2a(RenderHint::TextAntialiasing, true);
            self.painter
                .set_render_hint_2a(RenderHint::LosslessImageRendering, true);
        }
    }
}

impl PainterTrait for Painter {
    #[inline]
    fn save(&mut self) {
        unsafe {
            self.painter.save();
        }
    }

    #[inline]
    fn restore(&mut self) {
        unsafe {
            self.painter.restore();
        }
    }

    fn clear_all(&mut self) {
        todo!()
    }

    #[inline]
    fn clip(&mut self) {
        todo!()
    }

    #[inline]
    fn fill(&mut self, path: &dyn PathTrait) {
        let path_ref = path.as_any().downcast_ref::<Path>().unwrap();
        unsafe {
            let brush = self.painter.brush();
            self.painter.fill_path(&path_ref.path, brush);
        }
    }

    #[inline]
    fn stroke(&mut self, path: &dyn PathTrait) {
        let path_ref = path.as_any().downcast_ref::<Path>().unwrap();
        unsafe {
            let pen = self.painter.pen();
            self.painter.stroke_path(&path_ref.path, pen);
        }
    }

    #[inline]
    fn rotate(&mut self, angle: f64) {
        unsafe {
            self.painter.rotate(angle);
        }
    }

    #[inline]
    fn scale(&mut self, sx: f64, sy: f64) {
        unsafe {
            self.painter.scale(sx, sy);
        }
    }

    #[inline]
    fn translate(&mut self, point: PointF) {
        unsafe {
            self.painter.translate_2_double(point.x(), point.y());
        }
    }

    fn draw_text(&mut self, text: &str, position: PointF) {
        unsafe {
            let q_point = QPointF::new_2a(position.x(), position.y());
            let q_str = QString::from_std_str(&text);
            self.painter.draw_text_q_point_f_q_string(&q_point, &q_str);
        }
    }
}
