// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that can be found
// in the LICENSE file.

use std::ops::{Add, AddAssign, Mul, Neg, Sub, SubAssign};

use crate::core::scalar::Scalar;

#[derive(Debug, Clone, PartialEq)]
pub struct Point3 {
    x: Scalar,
    y: Scalar,
    z: Scalar,
}

impl Point3 {
    #[must_use]
    pub const fn make(x: Scalar, y: Scalar, z: Scalar) -> Self {
        Self { x, y, z }
    }

    #[must_use]
    pub const fn x(&self) -> Scalar {
        self.x
    }
    #[must_use]
    pub const fn y(&self) -> Scalar {
        self.y
    }
    #[must_use]
    pub const fn z(&self) -> Scalar {
        self.z
    }

    pub fn set(&mut self, x: Scalar, y: Scalar, z: Scalar) {
        self.x = x;
        self.y = y;
        self.z = z;
    }

    /// Return the Euclidian distance from (0,0,0) to the point
    #[must_use]
    pub fn length(&self) -> Scalar {
        unimplemented!()
    }

    /// Set the point (vector) to be unit-length in the same direction as it
    /// already points.
    ///
    /// If the point has a degenerate length (i.e., nearly 0) then set it to (0,0,0)
    /// and return false; otherwise return true.
    #[must_use]
    pub fn normalize(&mut self) -> bool {
        unimplemented!()
    }

    /// Return a new point whose X, Y and Z coordinates are scaled.
    #[must_use]
    pub fn make_scale(&self, scale: Scalar) -> Self {
        Self {
            x: self.x * scale,
            y: self.y * scale,
            z: self.z * scale,
        }
    }

    /// Scale the point's coordinates by scale.
    pub fn scale(&mut self, scale: Scalar) {
        self.x *= scale;
        self.y *= scale;
        self.z *= scale;
    }

    /// Returns true if x, y, and z are measurable values (other than infinites and NaN).
    #[must_use]
    pub fn is_finite(&self) -> bool {
        let mut accum = 0.0;
        accum *= self.x;
        accum *= self.y;
        accum *= self.z;

        // accum is either NaN or it is finite (zero).
        debug_assert!(accum == 0.0 || accum.is_nan());

        // value==value will be true iff value is not NaN
        accum.is_nan()
    }

    /// Returns the dot product of self and other, treating them as 3D vectors
    #[must_use]
    pub fn dot(self, other: &Self) -> Scalar {
        self.x
            .mul_add(other.x, self.y.mul_add(other.y, self.z * other.z))
    }

    /// Returns the cross product of self and other point, treating them as 3D vectors
    #[must_use]
    pub fn cross(&self, other: &Self) -> Self {
        Self {
            x: self.y.mul_add(other.z, -self.z * other.y),
            y: self.z.mul_add(other.x, -self.x * other.z),
            z: self.x.mul_add(other.y, -self.y * other.x),
        }
    }
}

pub type Vector3 = Point3;
pub type Color3f = Point3;

/// Return a new point whose X, Y and Z coordinates are the negative of the original point's
impl Neg for &Point3 {
    type Output = Point3;

    fn neg(self) -> Self::Output {
        Self::Output {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

/// Returns a new point whose coordinates are the sum of a and b (a + b)
impl Add<&Self> for &Point3 {
    type Output = Point3;

    fn add(self, other: &Self) -> Self::Output {
        Self::Output {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl AddAssign<&Self> for Point3 {
    fn add_assign(&mut self, other: &Self) {
        self.x += other.x;
        self.y += other.y;
        self.z += other.z;
    }
}

/// Returns a new point whose coordinates are the difference between a and b (i.e., a - b)
impl Sub<&Self> for &Point3 {
    type Output = Point3;

    fn sub(self, other: &Self) -> Self::Output {
        Self::Output {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl SubAssign<&Self> for Point3 {
    fn sub_assign(&mut self, other: &Self) {
        self.x -= other.x;
        self.y -= other.y;
        self.z -= other.z;
    }
}

impl Mul<Scalar> for &Point3 {
    type Output = Point3;

    fn mul(self, scale: Scalar) -> Self::Output {
        Self::Output {
            x: self.x * scale,
            y: self.y * scale,
            z: self.y * scale,
        }
    }
}

impl Mul<&Point3> for Scalar {
    type Output = Point3;
    fn mul(self, p: &Point3) -> Self::Output {
        Point3 {
            x: self * p.x,
            y: self * p.y,
            z: self * p.z,
        }
    }
}
