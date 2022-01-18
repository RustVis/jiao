// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Rect {
    x: i32,
    y: i32,
    width: i32,
    height: i32,
}

impl Rect {
    pub fn new(x: i32, y: i32, width: i32, height: i32) -> Self {
        Self {
            x,
            y,
            width,
            height,
        }
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

    pub fn set_width(&mut self, width: i32) {
        self.width = width;
    }

    pub fn width(&self) -> i32 {
        self.width
    }

    pub fn set_height(&mut self, height: i32) {
        self.height = height;
    }
}

#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
pub struct RectF {
    x: f64,
    y: f64,
    width: f64,
    height: f64,
}

impl RectF {
    pub fn new(x: f64, y: f64, width: f64, height: f64) -> Self {
        Self {
            x,
            y,
            width,
            height,
        }
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

    pub fn set_width(&mut self, width: f64) {
        self.width = width;
    }

    pub fn width(&self) -> f64 {
        self.width
    }

    pub fn set_height(&mut self, height: f64) {
        self.height = height;
    }
}
