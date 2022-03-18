// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

/// This enum type defines what happens to the aspect ratio when scaling an rectangle.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AspectRatioMode {
    /// The size is scaled freely. The aspect ratio is not preserved.
    IgnoreAspectRatio,

    /// The size is scaled to a rectangle as large as possible inside a given rectangle,
    /// preserving the aspect ratio.
    KeepAspectRatio,

    /// The size is scaled to a rectangle as small as possible outside a given rectangle,
    /// preserving the aspect ratio.
    KeepAspectRatioByExpanding,
}

impl Default for AspectRatioMode {
    fn default() -> Self {
        Self::IgnoreAspectRatio
    }
}
