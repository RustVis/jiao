// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that can be found
// in the LICENSE file.

use std::ops::{Add, AddAssign, Mul, Neg, Sub, SubAssign};

use crate::core::scalar::{Scalar, SCALAR_NEARLY_ZERO};

#[derive(Debug, Clone, PartialEq)]
pub struct Point3 {
    x: Scalar,
    y: Scalar,
    z: Scalar,
}

impl Point3 {
    #[must_use]
    #[inline]
    pub const fn from(x: Scalar, y: Scalar, z: Scalar) -> Self {
        Self { x, y, z }
    }

    #[must_use]
    #[inline]
    pub const fn x(&self) -> Scalar {
        self.x
    }

    #[must_use]
    #[inline]
    pub const fn y(&self) -> Scalar {
        self.y
    }

    #[must_use]
    #[inline]
    pub const fn z(&self) -> Scalar {
        self.z
    }

    #[inline]
    pub fn set(&mut self, x: Scalar, y: Scalar, z: Scalar) {
        self.x = x;
        self.y = y;
        self.z = z;
    }

    /// Return the Euclidian distance from (0,0,0) to (x,y,z)
    #[must_use]
    #[allow(clippy::cast_possible_truncation)]
    pub fn from_length(x: Scalar, y: Scalar, z: Scalar) -> Scalar {
        let mag_sq = get_length_squared(x, y, z);
        if mag_sq.is_finite() {
            mag_sq.sqrt()
        } else {
            let xx = f64::from(x);
            let yy = f64::from(y);
            let zz = f64::from(z);
            (xx.mul_add(xx, yy.mul_add(yy, zz * zz))).sqrt() as f32
        }
    }

    /// Return the Euclidian distance from (0,0,0) to the point
    #[must_use]
    #[inline]
    pub fn length(&self) -> Scalar {
        Self::from_length(self.x, self.y, self.z)
    }

    /// Set the point (vector) to be unit-length in the same direction as it
    /// already points.
    ///
    /// If the point has a degenerate length (i.e., nearly 0) then set it to (0,0,0)
    /// and return false; otherwise return true.
    #[must_use]
    pub fn normalize(&mut self) -> bool {
        // We have to worry about 2 tricky conditions:
        // 1. underflow of magSq (compared against nearlyzero^2)
        // 2. overflow of magSq (compared w/ isfinite)
        //
        // If we underflow, we return false. If we overflow, we compute again using
        // doubles, which is much slower (3x in a desktop test) but will not overflow.
        let mut mag_sq = 0.0;
        if is_length_nearly_zero(self.x, self.y, self.z, &mut mag_sq) {
            self.set(0.0, 0.0, 0.0);
            return false;
        }

        // sqrtf does not provide enough precision; since sqrt takes a double,
        // there's no additional penalty to storing invScale in a double
        let inv_scale = if mag_sq.is_finite() {
            f64::from(mag_sq)
        } else {
            // our magSq step overflowed to infinity, so use doubles instead.
            // much slower, but needed when x, y or z is very large, otherwise we
            // divide by inf. and return (0,0,0) vector.
            let xx = f64::from(self.x);
            let yy = f64::from(self.y);
            let zz = f64::from(self.z);
            xx.mul_add(xx, yy.mul_add(yy, zz * zz))
        };

        // using a float instead of a double for scale loses too much precision
        #[allow(clippy::cast_possible_truncation)]
        let scale = (1.0 / inv_scale.sqrt()) as f32;
        self.scale(scale);
        if !self.x.is_finite() || !self.y.is_finite() || !self.z.is_finite() {
            self.set(0.0, 0.0, 0.0);
            false
        } else {
            true
        }
    }

    /// Return a new point whose X, Y and Z coordinates are scaled.
    #[must_use]
    #[inline]
    pub fn make_scale(&self, scale: Scalar) -> Self {
        Self {
            x: self.x * scale,
            y: self.y * scale,
            z: self.z * scale,
        }
    }

    /// Scale the point's coordinates by scale.
    #[inline]
    pub fn scale(&mut self, scale: Scalar) {
        self.x *= scale;
        self.y *= scale;
        self.z *= scale;
    }

    /// Returns true if x, y, and z are measurable values (other than infinites and NaN).
    #[must_use]
    #[inline]
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

    /// Returns the dot product of a and b, treating them as 3D vectors
    #[must_use]
    #[inline]
    pub fn dot_product(a: &Self, b: &Self) -> Scalar {
        a.x.mul_add(b.x, a.y.mul_add(b.y, a.z * b.z))
    }

    /// Returns the dot product of self and other, treating them as 3D vectors
    #[must_use]
    #[inline]
    pub fn dot(&self, other: &Self) -> Scalar {
        Self::dot_product(self, other)
    }

    /// Returns the cross product of a and b, treating them as 3D vectors
    #[must_use]
    #[inline]
    pub fn cross_product(a: &Self, b: &Self) -> Self {
        Self {
            x: a.y.mul_add(b.z, -a.z * b.y),
            y: a.z.mul_add(b.x, -a.x * b.z),
            z: a.x.mul_add(b.y, -a.y * b.x),
        }
    }

    /// Returns the cross product of self and other point, treating them as 3D vectors
    #[must_use]
    pub fn cross(&self, other: &Self) -> Self {
        Self::cross_product(self, other)
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

/// Add other's coordinates to the point's
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

/// Subtract other's coordinates from the point's
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

// Returns the square of the Euclidian distance to (x,y,z).
fn get_length_squared(x: f32, y: f32, z: f32) -> f32 {
    x.mul_add(x, y.mul_add(y, z * z))
}

// Calculates the square of the Euclidian distance to (x,y,z) and stores it in
// *lengthSquared.  Returns true if the distance is judged to be "nearly zero".
//
// This logic is encapsulated in a helper method to make it explicit that we
// always perform this check in the same manner, to avoid inconsistencies
// (see http://code.google.com/p/skia/issues/detail?id=560 ).
fn is_length_nearly_zero(x: f32, y: f32, z: f32, length_squared: &mut f32) -> bool {
    *length_squared = get_length_squared(x, y, z);
    *length_squared <= (SCALAR_NEARLY_ZERO * SCALAR_NEARLY_ZERO)
}
