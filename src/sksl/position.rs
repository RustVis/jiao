// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that can be found
// in the LICENSE file.

#![allow(
    clippy::cast_possible_truncation,
    clippy::cast_possible_wrap,
    clippy::cast_sign_loss
)]

use std::cmp::{self, Ordering};

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct Position {
    start_offset: i32,
    length: u8,
}

pub const MAX_OFFSET: i32 = 0x007F_FFFF;

impl Default for Position {
    fn default() -> Self {
        Self::new()
    }
}

impl cmp::PartialOrd for Position {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.start_offset.partial_cmp(&other.start_offset)
    }
}

impl Position {
    #[must_use]
    #[inline]
    pub const fn new() -> Self {
        Self {
            start_offset: -1,
            length: 0,
        }
    }

    #[must_use]
    #[inline]
    pub const fn from_range(start_offset: i32, end_offset: i32) -> Self {
        debug_assert!(start_offset <= end_offset);
        debug_assert!(start_offset <= MAX_OFFSET);
        let length = end_offset - start_offset;
        let length = if length > 0xFF { 0xFF } else { length };
        Self {
            start_offset,
            length: length as u8,
        }
    }

    #[must_use]
    #[inline]
    pub const fn valid(self) -> bool {
        self.start_offset != -1
    }

    #[must_use]
    pub fn line(&self, source: &str) -> i32 {
        debug_assert!(self.valid());
        if self.start_offset == -1 {
            return -1;
        }
        if source.is_empty() {
            return -1;
        }

        // we allow the offset to equal the length, because that's where TK_END_OF_FILE is reported
        let source_len = source.len() as i32;
        debug_assert!(self.start_offset <= source_len);
        let offset = self.start_offset.min(source_len) as usize;
        let mut line = 1;
        for chr in source[..offset].chars() {
            if chr == '\n' {
                line += 1;
            }
        }
        line
    }

    #[must_use]
    pub const fn start_offset(self) -> i32 {
        debug_assert!(self.valid());
        self.start_offset
    }

    #[must_use]
    pub const fn end_offset(self) -> i32 {
        debug_assert!(self.valid());
        self.start_offset + self.length as i32
    }

    /// Returns the position from this through, and including the entirety of, end.
    #[must_use]
    pub fn range_through(self, end: Self) -> Self {
        if self.start_offset == -1 || end.start_offset == -1 {
            return self;
        }

        debug_assert!(
            self.start_offset() <= end.start_offset() && self.end_offset() <= end.end_offset(),
            "Invalid range: ({}-{}) - ({}-{})",
            self.start_offset(),
            self.end_offset(),
            end.start_offset(),
            end.end_offset()
        );
        Self::from_range(self.start_offset(), end.end_offset())
    }

    /// Returns a position representing the character immediately after this position
    #[must_use]
    #[inline]
    pub const fn after(self) -> Self {
        let end_offset = self.end_offset();
        Self::from_range(end_offset, end_offset + 1)
    }
}
