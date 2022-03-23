// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

use core::ops;

/// The GenericMatrix struct is a template that represents a NxM transformation matrix
/// with N columns and M rows.
///
/// # Arguments
/// - N: Number of columns
/// - M: Number of rows
/// - T: Element type that is visible
pub struct GenericMatrix<N, M, T> {
    // Column-major order to match OpenGL.
    m: [[T; M]; N],
}

impl<N, M, T> GenericMatrix<N, M, T> {
    /// Constructs a NxM identity matrix.
    pub fn new() -> Self {
        unimplemented!()
    }

    /// Constructs a matrix from the given N * M floating-point values.
    ///
    /// The contents of the array values is assumed to be in row-major order.
    pub fn from(values: &[T]) -> Self {
        unimplemented!()
    }

    /// Retrieves the N * M items in this matrix and copies them to values in row-major order.
    pub fn copy_data_to(&self, values: &mut [T]) {
        unimplemented!()
    }

    /// Fills all elements of this matrix with value.
    pub fn fill(&mut self, value: T) {
        unimplemented!()
    }

    /// Returns true if this matrix is the identity; false otherwise.
    pub fn is_identity(&self) -> bool {
        unimplemented!()
    }

    /// Sets this matrix to the identity.
    pub fn set_identity(&mut self) {
        unimplemented!()
    }

    /// Returns this matrix, transposed about its diagonal.
    pub fn transposed(&self) -> GenericMatrix<M, N, T> {
        unimplemented!()
    }
}

/*
impl<T> ops::Deref for GenericMatrix<N, M, T> {
    type Target = [T];
    fn deref(&self) -> &Self::Target {
        unimplemented!()
    }
}

impl<T> ops::DerefMut for GenericMatrix<N, M, T> {
    type Target = [T];
    fn deref_mut(&mut self) -> &mut Self::Target {
        unimplemented!()
    }
}
*/
