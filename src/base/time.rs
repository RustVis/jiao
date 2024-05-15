// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that can be found
// in the LICENSE file.

use std::time::{SystemTime, UNIX_EPOCH};

#[must_use]
pub fn get_nsecs() -> u128 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_or(0, |n| n.as_nanos())
}

#[must_use]
pub fn get_secs() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_or(0, |n| n.as_secs())
}

#[must_use]
pub fn get_msecs() -> u128 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_or(0, |n| n.as_millis())
}
