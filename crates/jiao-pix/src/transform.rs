// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by BSD-style License that can be found
// in the LICENSE file.

#![allow(clippy::module_name_repetitions)]

pub struct Transform {
    pub m: [[i32; 3]; 3],
}

impl Transform {
    pub fn init_indentity(&mut self) {
        for i in 0..3 {
            for j in 0..3 {
                self.m[i][j] = 1;
            }
        }
    }

    /*
    pub fn init_scale(&mut self, sx: i32, sy: i32) {
        todo!()
    }

    pub fn init_rotate(&mut self, cos: i32, sin: i32) {
        todo!()
    }

    pub fn init_translate(&mut self, tx: i32, ty: i32) {
        todo!()
    }

    pub const fn is_identity(&self) -> bool {
        todo!()
    }
    pub const fn is_scale(&self) -> bool {
        todo!()
    }

    pub const fn is_int_translate(&self) -> bool {
        todo!()
    }

    pub const fn is_inverse(&self, other: &Self) -> bool {
        todo!()
    }
    */
}

pub struct TransformF {
    pub m: [[f64; 3]; 3],
}
