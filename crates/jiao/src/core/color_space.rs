// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

#![allow(dead_code)]

#[derive(Debug, Clone)]
pub struct ColorSpace {
    transfer_fn_hash: u32,
    to_xyzd_50_hash: u32,
}
