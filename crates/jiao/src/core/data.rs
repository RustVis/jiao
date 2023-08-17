// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

/// Data holds an immutable data buffer.
///
/// Not only is the data immutable, but the actual ptr that is returned
/// (by data() or bytes()) is guaranteed to always be the same for the life
/// of this instance.
#[derive(Debug, Default, Clone, PartialEq)]
pub struct Data {
    data: Vec<u8>,
}

impl Data {
    /// Returns the number of bytes stored.
    #[must_use]
    pub fn len(&self) -> usize {
        self.data.len()
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    /// Returns the ptr to the data.
    #[must_use]
    pub fn data(&self) -> &[u8] {
        &self.data
    }

    /// Like data(), returns a read-only ptr into the data
    #[must_use]
    pub fn bytes(&self) -> &[u8] {
        &self.data
    }

    /**
     *  USE WITH CAUTION.
     *  This call will assert that the refcnt is 1, as a precaution against modifying the
     *  contents when another client/thread has access to the data.
     */
    pub fn data_mut(&mut self) -> &mut [u8] {
        &mut self.data
    }

    /// Helper to copy a range of the data into a caller-provided buffer.
    ///
    /// Returns the actual number of bytes copied, after clamping offset and
    /// length to the size of the data. If buffer is empty, it is ignored, and
    /// only the computed number of bytes is returned.
    #[must_use]
    pub fn copy_range(&self, _offset: usize, _buffer: &mut [u8]) -> usize {
        unimplemented!()
    }

    /// Create a new dataref by copying the specified data
    #[must_use]
    pub fn from(data: &[u8]) -> Self {
        Self {
            data: data.to_vec(),
        }
    }

    #[must_use]
    pub fn with_capacity(cap: usize) -> Self {
        Self {
            data: Vec::with_capacity(cap),
        }
    }

    /// Create a new data with zero-initialized contents.
    ///
    /// The caller should call writable_data() to write into the buffer,
    /// but this must be done before another ref() is made.
    #[must_use]
    pub fn with_zero_initialized(len: usize) -> Self {
        Self { data: vec![0; len] }
    }

    /// Create a new dataref by copying the specified string.
    ///
    /// The returned Data will have size() equal to length of string.
    #[must_use]
    pub fn from_string(s: &str) -> Self {
        Self::from(s.as_bytes())
    }

    /// Call this when the data parameter is already const and will outlive
    /// the lifetime of the Data.
    ///
    /// Suitable for with const globals.
    #[must_use]
    pub fn from_data(data: Vec<u8>) -> Self {
        Self { data }
    }

    /// Create a new dataref using a subset of the data in the specified src dataref.
    pub fn new_subset(src: &Self, offset: usize, length: usize) -> Self {
        Self {
            data: src.data[offset..offset + length].to_vec(),
        }
    }

    /// Returns a new empty dataref (or a reference to a shared empty dataref).
    /// New or shared, the caller must see that unref() is eventually called.
    pub fn new() -> Self {
        Self { data: Vec::new() }
    }
}
