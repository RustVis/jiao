// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

use std::fmt;
use std::io;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ErrorKind {
    IoError,

    CairoError,
    JsError,
    SkiaError,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Error {
    kind: ErrorKind,
    message: String,
}

impl Error {
    #[must_use]
    pub fn new(kind: ErrorKind, s: &str) -> Self {
        Self {
            kind,
            message: s.to_string(),
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

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}: {}", self.kind, self.message)
    }
}

impl std::error::Error for Error {}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Self {
        Self::from_string(ErrorKind::IoError, format!("{err:?}"))
    }
}

cfg_if::cfg_if! {
    if #[cfg(feature = "cairo")] {
        impl From<cairo::Error> for Error {
            fn from(err: cairo::Error) -> Self {
                Self::from_string(ErrorKind::CairoError, format!("{err:?}"))
            }
        }

        impl From<cairo::IoError> for Error {
            fn from(err: cairo::IoError) -> Self {
                Self::from_string(ErrorKind::IoError, format!("{err:?}"))
            }
        }
    } else if #[cfg(feature = "web")] {
        impl From<wasm_bindgen::JsValue> for Error {
            fn from(value: wasm_bindgen::JsValue) -> Self {
                Self::from_string(ErrorKind::JsError, format!("{value:?}"))
            }
        }

        impl From<web_sys::Element> for Error {
            fn from(value: web_sys::Element) -> Self {
                Self::from_string(ErrorKind::JsError, format!("{value:?}"))
            }
        }

        impl From<js_sys::Object> for Error {
            fn from(value: js_sys::Object) -> Self {
                Self::from_string(ErrorKind::JsError, format!("{value:?}"))
            }
        }
    }
}