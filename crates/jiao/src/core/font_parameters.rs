// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by LGPL-3.0 License that can be found
// in the LICENSE file.

pub mod variation {
    use crate::core::types::FourByteTag;

    /// Parameters in a variation font axis.
    #[derive(Debug, Default, Clone, PartialEq)]
    pub struct Axis {
        /// Four character identifier of the font axis (weight, width, slant, italic...).
        pub tag: FourByteTag,

        /// Minimum value supported by this axis.
        pub min: f32,

        /// Default value set by this axis.
        pub def: f32,

        /// Maximum value supported by this axis. The maximum can equal the minimum.
        pub max: f32,

        /// Attributes for a font axis.
        flags: u16,
    }

    impl Axis {
        pub(crate) const HIDDEN: u16 = 0x0001;

        #[must_use]
        pub fn new() -> Self {
            Self::default()
        }

        #[must_use]
        pub const fn from(tag: FourByteTag, min: f32, def: f32, max: f32, hidden: bool) -> Self {
            let flags = if hidden { Self::HIDDEN } else { 0 };
            Self {
                tag,
                min,
                def,
                max,
                flags,
            }
        }

        /// Return whether this axis is recommended to be remain hidden in user interfaces.
        #[must_use]
        pub const fn is_hidden(&self) -> bool {
            self.flags & Self::HIDDEN == Self::HIDDEN
        }

        /// Set this axis to be remain hidden in user interfaces.
        pub fn set_hidden(&mut self, hidden: bool) {
            self.flags = if hidden {
                self.flags | Self::HIDDEN
            } else {
                self.flags & !Self::HIDDEN
            };
        }
    }
}
