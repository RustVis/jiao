// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

use cairo::Context;

use super::traits::Canvas;

pub struct CairoContext(Context);

impl Canvas for CairoContext {
    #[inline]
    fn save(&mut self) {
        // TODO(Shaohua):
        self.0.save();
    }

    #[inline]
    fn restore(&mut self) {
        // TODO(Shaohua):
        self.0.restore();
    }

    #[inline]
    fn clip(&mut self) {
        self.0.clip();
    }

    #[inline]
    fn fill(&mut self) {
        self.0.fill();
    }

    #[inline]
    fn stroke(&mut self) {
        self.0.stroke();
    }

    #[inline]
    fn begin_path(&mut self) {
        self.0.new_path();
    }

    #[inline]
    fn close_path(&mut self) {
        self.0.close_path();
    }

    #[inline]
    fn line_to(&mut self, x: f64, y: f64) {
        self.0.line_to(x, y);
    }

    #[inline]
    fn move_to(&mut self, x: f64, y: f64) {
        self.0.move_to(x, y);
    }
}
