// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

use std::error;
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ErrorKind {
    // For status == 400.
    BadRequest,

    // For status == 401.
    Unauthorized,

    // For status == 403.
    Forbidden,

    // For status == 404.
    NotFound,

    // For status == 500.
    InternalServerError,

    JsError,
    DeserializeError,
    RequestError,
    ResponseError,
    UrlParamError,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FetchError {
    kind: ErrorKind,
    message: String,
}

impl FetchError {
    #[must_use]
    pub const fn new(kind: ErrorKind) -> Self {
        Self {
            kind,
            message: String::new(),
        }
    }

    #[must_use]
    pub const fn from_string(kind: ErrorKind, message: String) -> Self {
        Self { kind, message }
    }

    #[must_use]
    pub const fn kind(&self) -> ErrorKind {
        self.kind
    }

    #[must_use]
    pub fn message(&self) -> &str {
        &self.message
    }
}

impl fmt::Display for FetchError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}: {}", self.kind, self.message)
    }
}

impl error::Error for FetchError {}
