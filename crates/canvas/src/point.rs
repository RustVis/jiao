// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct Point {
    x: i32,
    y: i32,
}

impl Point {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    pub fn set_x(&mut self, x: i32) {
        self.x = x;
    }

    pub fn x(&self) -> i32 {
        self.x
    }

    pub fn set_y(&mut self, y: i32) {
        self.y = y;
    }

    pub fn y(&self) -> i32 {
        self.y
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct PointF {
    x: f64,
    y: f64,
}

impl PointF {
    pub fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }

    pub fn set_x(&mut self, x: f64) {
        self.x = x;
    }

    pub fn x(&self) -> f64 {
        self.x
    }

    pub fn set_y(&mut self, y: f64) {
        self.y = y;
    }

    pub fn y(&self) -> f64 {
        self.y
    }
}
