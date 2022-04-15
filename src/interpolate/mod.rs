// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

pub mod color;

pub type ReturnFunc = Box<dyn Fn(f64) -> f64>;

pub fn constant(a: f64) -> ReturnFunc {
    Box::new(move |_t: f64| a)
}
