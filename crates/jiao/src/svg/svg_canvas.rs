// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by LGPL-3.0 License that can be found
// in the LICENSE file.

#![allow(clippy::module_name_repetitions)]

use bitflags::bitflags;

use crate::core::canvas::Canvas;
use crate::core::rect::Rect;

bitflags! {
    #[derive(Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Flag : u8 {
        /// emit text as <path>s
        const ConvertTextToPaths = 0x01;

        /// suppress newlines and tabs in output
        const NoPrettyXml = 0x02;

        /// use relative commands for path encoding
        const RelativePathEncoding = 0x04;
    }
}

/// Returns a new canvas that will generate SVG commands from its draw calls, and send
/// them to the provided stream.
///
/// Ownership of the stream is not transfered, and it must remain valid
/// for the lifetime of the returned canvas.
///
/// The canvas may buffer some drawing calls, so the output is not guaranteed to be valid
/// or complete until the canvas instance is deleted.
///
/// The 'bounds' parameter defines an initial SVG viewport (`viewBox` attribute on the root
/// SVG element).
#[must_use]
pub fn make(_bounds: &Rect, /* WStream* stream */ _flags: Flag) -> Canvas {
    unimplemented!()
}
