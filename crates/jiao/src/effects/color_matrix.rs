// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by LGPL-3.0 License that can be found
// in the LICENSE file.

use crate::core::image_info::YuvColorSpace;

#[derive(Debug, Clone, PartialEq)]
pub struct ColorMatrix {
    mat: [f32; 20],
}

impl Default for ColorMatrix {
    fn default() -> Self {
        Self::new()
    }
}

impl ColorMatrix {
    #[must_use]
    #[rustfmt::skip]
    pub const fn new() -> Self {
        Self {
            mat: [
                1., 0., 0., 0., 0.,
                0., 1., 0., 0., 0.,
                0., 0., 1., 0., 0.,
                0., 0., 0., 1., 0.,
            ]
        }
    }

    #[must_use]
    #[rustfmt::skip]
    #[allow(clippy::too_many_arguments)]
    pub const fn from(
        m00: f32, m01: f32, m02: f32, m03: f32, m04: f32,
        m10: f32, m11: f32, m12: f32, m13: f32, m14: f32,
        m20: f32, m21: f32, m22: f32, m23: f32, m24: f32,
        m30: f32, m31: f32, m32: f32, m33: f32, m34: f32) -> Self {
        Self {
            mat: [
                m00, m01, m02, m03, m04,
                m10, m11, m12, m13, m14,
                m20, m21, m22, m23, m24,
                m30, m31, m32, m33, m34,
            ],
        }
    }

    #[must_use]
    pub fn rgb_to_yuv(_color_space: YuvColorSpace) -> Self {
        unimplemented!()
    }

    #[must_use]
    pub fn yuv_to_rgb(_color_space: YuvColorSpace) -> Self {
        unimplemented!()
    }

    #[rustfmt::skip]
    pub fn set_identity(&mut self) {
        self.mat = [
            1., 0., 0., 0., 0.,
            0., 1., 0., 0., 0.,
            0., 0., 1., 0., 0.,
            0., 0., 0., 1., 0.,
        ];
    }

    pub fn set_scale(&mut self, red_scale: f32, green_scale: f32, blue_scale: f32) {
        self.set_scale_with_alpha(red_scale, green_scale, blue_scale, 1.0);
    }

    pub fn set_scale_with_alpha(
        &mut self,
        _red_scale: f32,
        _green_scale: f32,
        _blue_scale: f32,
        _alpha_scale: f32,
    ) {
        unimplemented!();
    }

    pub fn post_translate(&mut self, _dr: f32, _dg: f32, _db: f32, _da: f32) {
        unimplemented!();
    }

    pub fn set_concat(&mut self, _mat_a: &Self, _mat_b: &Self) {
        unimplemented!();
    }

    pub fn pre_concat(&mut self, _mat: &Self) {
        unimplemented!();
        //this->setConcat(*this, mat)
    }

    pub fn post_concat(&mut self, _mat: &Self) {
        unimplemented!()
        //this->setConcat(mat, *this);
    }

    pub fn set_saturation(&mut self, _sat: f32) {
        unimplemented!()
    }

    pub fn set_row_major(&mut self, src: &[f32; 20]) {
        self.mat = *src;
    }

    #[must_use]
    pub const fn get_row_major(&self) -> &[f32; 20] {
        &self.mat
    }
}
