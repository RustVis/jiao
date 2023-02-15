// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

#[derive(Debug)]
pub struct CairoError(cairo::Error);

impl From<CairoError> for jiao::error::Error {
    fn from(err: CairoError) -> Self {
        Self::from_string(jiao::error::ErrorKind::CairoError, format!("{:?}", err.0))
    }
}

impl From<cairo::Error> for CairoError {
    fn from(err: cairo::Error) -> Self {
        Self(err)
    }
}
