// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

#![allow(clippy::cast_possible_truncation)]
#![allow(clippy::module_name_repetitions)]

pub type Scalar = f32;

#[must_use]
pub const fn scalar_from_int(x: i32) -> Scalar {
    x as Scalar
}

pub trait ScalarExt {
    #[must_use]
    fn ceil_to_int(self) -> i32;

    #[must_use]
    fn floor_to_int(self) -> i32;

    #[must_use]
    fn round_to_int(self) -> i32;
}

impl ScalarExt for Scalar {
    fn ceil_to_int(self) -> i32 {
        self.ceil() as i32
    }

    fn floor_to_int(self) -> i32 {
        self.floor() as i32
    }

    fn round_to_int(self) -> i32 {
        self.round() as i32
    }
}
