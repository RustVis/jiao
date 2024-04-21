// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that can be found
// in the LICENSE file.

use crate::core::point::Point;
use crate::core::scalar::{Scalar, ScalarExt};
use crate::core::size::Size;

/// A compressed form of a rotation+scale matrix.
///
/// [ scos     -ssin    tx ]
/// [ ssin      scos    ty ]
/// [    0        0     1 ]
#[derive(Debug, Default, Clone, PartialEq)]
pub struct RsxForm {
    pub scos: Scalar,
    pub ssin: Scalar,
    pub tx: Scalar,
    pub ty: Scalar,
}

impl RsxForm {
    #[must_use]
    #[inline]
    pub const fn from(scos: Scalar, ssin: Scalar, tx: Scalar, ty: Scalar) -> Self {
        Self { scos, ssin, tx, ty }
    }

    /// Initialize a new xform based on the scale, rotation (in radians), final tx,ty location
    /// and anchor-point ax,ay within the src quad.
    ///
    /// Note: the anchor point is not normalized (e.g. 0...1) but is in pixels of the src image.
    #[must_use]
    #[inline]
    pub fn from_radians(
        scale: Scalar,
        radians: Scalar,
        tx: Scalar,
        ty: Scalar,
        ax: Scalar,
        ay: Scalar,
    ) -> Self {
        let ssin = radians.sin() * scale;
        let scos = radians.cos() * scale;
        Self::from(
            scos,
            ssin,
            ssin.mul_add(ay, (-scos).mul_add(ax, tx)),
            scos.mul_add(-ay, (-ssin).mul_add(ax, ty)),
        )
    }

    #[must_use]
    #[inline]
    pub fn rect_stays_rect(&self) -> bool {
        self.scos.fuzzy_zero() || self.ssin.fuzzy_zero()
    }

    #[inline]
    pub fn set_identity(&mut self) {
        self.scos = 1.0;
        self.ssin = 0.0;
        self.tx = 0.0;
        self.ty = 0.0;
    }

    #[inline]
    pub fn set(&mut self, scos: Scalar, ssin: Scalar, tx: Scalar, ty: Scalar) {
        self.scos = scos;
        self.ssin = ssin;
        self.tx = tx;
        self.ty = ty;
    }

    #[must_use]
    pub fn to_quad(&self, width: Scalar, height: Scalar) -> [Point; 4] {
        let m00 = self.scos;
        let m01 = -self.ssin;
        let m02 = self.tx;
        let m10 = -m01;
        let m11 = m00;
        let m12 = self.ty;

        [
            Point::from_xy(m02, m12),
            Point::from_xy(m00.mul_add(width, m02), m10.mul_add(width, m12)),
            Point::from_xy(
                m00.mul_add(width, m01 * height) + m02,
                m10.mul_add(width, m11 * height) + m12,
            ),
            Point::from_xy(m01.mul_add(height, m02), m11.mul_add(height, m12)),
        ]
    }

    #[inline]
    #[must_use]
    pub fn to_quad_with_size(&self, size: &Size) -> [Point; 4] {
        self.to_quad(size.width(), size.height())
    }

    #[must_use]
    pub fn to_tri_strip(&self, width: Scalar, height: Scalar) -> [Point; 4] {
        let m00 = self.scos;
        let m01 = -self.ssin;
        let m02 = self.tx;
        let m10 = -m01;
        let m11 = m00;
        let m12 = self.ty;

        [
            Point::from_xy(m02, m12),
            Point::from_xy(m01.mul_add(height, m02), m11.mul_add(height, m12)),
            Point::from_xy(m00.mul_add(width, m02), m10.mul_add(width, m12)),
            Point::from_xy(
                m00.mul_add(width, m01 * height) + m02,
                m10.mul_add(width, m11 * height) + m12,
            ),
        ]
    }
}
