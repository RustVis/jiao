// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

use crate::core::scalar::Scalar;

#[derive(Debug, Clone, PartialEq)]
pub struct V2 {
    x: f32,
    y: f32,
}

impl V2 {
    #[must_use]
    pub fn dot(&self, other: &Self) -> Scalar {
        self.x.mul_add(other.x, self.y * other.y)
    }

    #[must_use]
    pub fn cross(&self, other: &Self) -> Scalar {
        self.x.mul_add(other.y, -self.y * other.x)
    }

    #[must_use]
    pub fn normalize(&self) -> Self {
        self * (1.0 / self.length())
    }

    #[must_use]
    pub fn length_squared(&self) -> Scalar {
        self.dot(self)
    }

    #[must_use]
    pub fn length(&self) -> Scalar {
        self.length_squared().sqrt()
    }
}

impl Neg for &V2 {
    type Output = V2;

    fn neg(self) -> Self::Output {
        Self::Output {
            x: self.x,
            y: self.y,
        }
    }
}

impl Add<Self> for &V2 {
    type Output = V2;

    fn add(self, other: Self) -> Self::Output {
        Self::Output {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Sub<Self> for &V2 {
    type Output = V2;

    fn sub(self, other: Self) -> Self::Output {
        Self::Output {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl Mul<Self> for &V2 {
    type Output = V2;

    fn mul(self, other: Self) -> Self::Output {
        Self::Output {
            x: self.x * other.x,
            y: self.x * other.y,
        }
    }
}

impl Mul<Scalar> for &V2 {
    type Output = V2;

    fn mul(self, scale: Scalar) -> Self::Output {
        Self::Output {
            x: self.x * scale,
            y: self.x * scale,
        }
    }
}

impl Mul<V2> for Scalar {
    type Output = V2;

    fn mul(self, v: V2) -> Self::Output {
        Self::Output {
            x: self * v.x,
            y: self * v.y,
        }
    }
}

impl Div<Scalar> for V2 {
    type Output = Self;

    fn div(self, scale: Scalar) -> Self {
        debug_assert!(scale != 0.0);

        Self {
            x: self.x / scale,
            y: self.x / scale,
        }
    }
}

impl Div<V2> for Scalar {
    type Output = V2;

    fn div(self, v: V2) -> Self::Output {
        debug_assert!(v.x != 0.0);
        debug_assert!(v.y != 0.0);

        Self::Output {
            x: self / v.x,
            y: self / v.y,
        }
    }
}

impl AddAssign<&Self> for V2 {
    fn add_assign(&mut self, other: &Self) {
        self.x += other.x;
        self.y += other.y;
    }
}

impl SubAssign<&Self> for V2 {
    fn sub_assign(&mut self, other: &Self) {
        self.x -= other.x;
        self.y -= other.y;
    }
}

impl MulAssign<&Self> for V2 {
    fn mul_assign(&mut self, other: &Self) {
        self.x *= other.x;
        self.y *= other.y;
    }
}

impl MulAssign<Scalar> for V2 {
    fn mul_assign(&mut self, scale: Scalar) {
        self.x *= scale;
        self.y *= scale;
    }
}

impl DivAssign<Scalar> for V2 {
    fn div_assign(&mut self, scale: Scalar) {
        debug_assert!(scale != 0.0);

        self.x /= scale;
        self.y /= scale;
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct V3 {
    x: f32,
    y: f32,
    z: f32,
}

impl V3 {
    #[must_use]
    pub fn dot(&self, other: &Self) -> Scalar {
        self.x
            .mul_add(other.x, self.y.mul_add(other.y, self.z * other.z))
    }

    #[must_use]
    pub fn cross(&self, other: &Self) -> Self {
        Self {
            x: self.y.mul_add(other.z, -self.z * other.y),
            y: self.z.mul_add(other.x, -self.x * other.z),
            z: self.x.mul_add(other.y, -self.y - other.x),
        }
    }

    #[must_use]
    pub fn normalize(&self) -> Self {
        self * (1.0 / self.length())
    }

    #[must_use]
    pub fn length_squared(&self) -> Scalar {
        self.dot(self)
    }

    #[must_use]
    pub fn length(&self) -> Scalar {
        self.dot(self).sqrt()
    }
}

impl Neg for &V3 {
    type Output = V3;

    fn neg(self) -> Self::Output {
        Self::Output {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl Add<Self> for &V3 {
    type Output = V3;

    fn add(self, other: Self) -> Self::Output {
        Self::Output {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl Sub<Self> for &V3 {
    type Output = V3;

    fn sub(self, other: Self) -> Self::Output {
        Self::Output {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl Mul<Self> for &V3 {
    type Output = V3;

    fn mul(self, other: Self) -> Self::Output {
        Self::Output {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z,
        }
    }
}

impl Mul<Scalar> for &V3 {
    type Output = V3;

    fn mul(self, scale: Scalar) -> Self::Output {
        Self::Output {
            x: self.x * scale,
            y: self.y * scale,
            z: self.z * scale,
        }
    }
}

impl Mul<&V3> for Scalar {
    type Output = V3;

    fn mul(self, v: &V3) -> Self::Output {
        Self::Output {
            x: self * v.x,
            y: self * v.y,
            z: self * v.z,
        }
    }
}

impl AddAssign<&Self> for V3 {
    fn add_assign(&mut self, other: &Self) {
        self.x += other.x;
        self.y += other.y;
        self.z += other.z;
    }
}

impl SubAssign<&Self> for V3 {
    fn sub_assign(&mut self, other: &Self) {
        self.x -= other.x;
        self.y -= other.y;
        self.z -= other.z;
    }
}

impl MulAssign<&Self> for V3 {
    fn mul_assign(&mut self, other: &Self) {
        self.x *= other.x;
        self.y *= other.y;
        self.z *= other.z;
    }
}

impl MulAssign<Scalar> for V3 {
    fn mul_assign(&mut self, scale: Scalar) {
        self.x *= scale;
        self.y *= scale;
        self.z *= scale;
    }
}
