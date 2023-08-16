// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

use crate::core::bitmap::Bitmap;

/// ColorTable holds the lookup tables for each channel (ARGB) used to define the filter behavior
/// of `ColorFilters::Table`, and provides a way to share the table data between client code and
/// the returned ColorFilter.
///
/// Once created, an ColorTable is immutable.
pub struct ColorTable {
    /// A 256x4 A8 image
    table: Bitmap,
}

impl ColorTable {
    /// Creates a new ColorTable with `table` used for all four channels.
    ///
    /// The table is copied into the `ColorTable`.
    pub fn from_table(table: &[u8; 256]) -> Self {
        Self::from_tables(table, table, table, table)
    }

    /// Creates a new ColorTable with the per-channel lookup tables.
    ///
    /// Each non-null table is copied into the ColorTable.
    /// Null parameters are interpreted as the identity table.
    pub fn from_tables(
        table_a: &[u8; 256],
        table_r: &[u8; 256],
        table_g: &[u8; 256],
        table_b: &[u8; 256],
    ) -> Self {
        unimplemented!()
    }

    /// Per-channel constant value lookup (0-255).
    #[must_use]
    pub fn alpha_table(&self) -> &[u8] {
        self.table.get_addr8(0, 0)
    }

    #[must_use]
    pub fn red_table(&self) -> &[u8] {
        self.table.get_addr8(0, 1)
    }

    #[must_use]
    pub fn green_table(&self) -> &[u8] {
        self.table.get_addr8(0, 2)
    }

    #[must_use]
    pub fn blue_table(&self) -> &[u8] {
        self.table.get_addr8(0, 3)
    }

    #[must_use]
    fn from_bitmap(table: Bitmap) -> Self {
        Self { table }
    }

    /// The returned Bitmap is immutable.
    #[must_use]
    fn bitmap(&self) -> &Bitmap {
        &self.table
    }
}
