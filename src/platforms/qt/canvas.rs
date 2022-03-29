// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

use qt_gui::QPainter;

use super::traits::Canvas;

pub struct QtCanvas(QPainter);

impl Canvas for QtCanvas {
    #[inline]
    fn save(&mut self) {
        unsafe {
            self.0.save();
        }
    }

    #[inline]
    fn restore(&mut self) {
        unsafe {
            self.0.restore();
        }
    }

    #[inline]
    fn clip(&mut self) {
        //self.0.clip();
    }

    #[inline]
    fn fill(&mut self) {
        //self.0.fill();
    }

    #[inline]
    fn stroke(&mut self) {
        //self.0.stroke();
    }

    #[inline]
    fn rotate(&mut self, angle: f64) {
        // TODO(Shaohua): Convert angle to radian.
        unsafe {
            self.0.rotate(angle);
        }
    }

    #[inline]
    fn scale(&mut self, x: f64, y: f64) {
        unsafe {
            self.0.scale(x, y);
        }
    }

    #[inline]
    fn translate(&mut self, x: f64, y: f64) {
        unsafe {
            self.0.translate_2_double(x, y);
        }
    }
}
