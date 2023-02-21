// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

#[derive(Debug)]
pub struct Error(cairo::Error);

impl From<cairo::Error> for Error {
    fn from(err: cairo::Error) -> Self {
        Self(err)
    }
}
