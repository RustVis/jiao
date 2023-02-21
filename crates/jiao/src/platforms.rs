// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

/// Platform specific features
#[derive(Debug, Default, Clone, PartialEq, Eq)]
#[allow(clippy::struct_excessive_bools)]
pub struct Features {
    /// Support instant image filters, like blur or grayscale.
    pub filter: bool,

    /// Server side rendering.
    ///
    /// Generate images in server side.
    pub ssr: bool,

    /// Support user interactive input events, like mouse event, keyboard event,
    /// wheel event and touch event.
    pub input_event: bool,

    /// Cursor icon is supported or not.
    pub cursor: bool,
}

impl Features {
    #[must_use]
    pub const fn new() -> Self {
        Self {
            filter: false,
            ssr: false,
            input_event: false,
            cursor: false,
        }
    }
}
