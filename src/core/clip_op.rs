// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that can be found
// in the LICENSE file.

#[repr(u8)]
#[derive(Debug, Default, Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum ClipOp {
    #[default]
    Difference = 0,
    Intersect = 1,
}

pub const MAX_CLIP_OP: usize = ClipOp::Intersect as usize;
