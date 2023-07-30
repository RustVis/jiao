// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by BSD-style License that can be found
// in the LICENSE file.

pub const MAX_INDEXED: usize = 256;

pub type IndexType = u8;

pub struct Indexed {
    pub color: bool,
    pub rgba: [u32; MAX_INDEXED],
    pub ent: [u8; 32768],
}
