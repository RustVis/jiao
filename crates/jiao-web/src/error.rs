// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

#[derive(Debug)]
pub struct Error(String);

impl Error {
    pub fn new(err: &str) -> Self {
        Self(err.to_string())
    }
}

impl From<wasm_bindgen::JsValue> for Error {
    fn from(value: wasm_bindgen::JsValue) -> Self {
        Self(format!("{value:?}"))
    }
}

impl From<web_sys::Element> for Error {
    fn from(value: web_sys::Element) -> Self {
        Self(format!("{value:?}"))
    }
}

impl From<js_sys::Object> for Error {
    fn from(value: js_sys::Object) -> Self {
        Self(format!("{value:?}"))
    }
}
