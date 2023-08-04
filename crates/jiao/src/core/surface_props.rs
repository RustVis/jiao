// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

use bitflags::bitflags;

/// Description of how the LCD strips are arranged for each pixel.
///
/// If this is unknown, or the pixels are meant to be "portable" and/or transformed
/// before showing (e.g. rotated, scaled) then use `Unknown`.
#[repr(u8)]
#[derive(Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
pub enum PixelGeometry {
    Unknown,
    RgbH,
    BgrH,
    RgbV,
    BgrV,
}

impl Default for PixelGeometry {
    fn default() -> Self {
        Self::Unknown
    }
}

impl PixelGeometry {
    /// Returns true iff geo is a known geometry and is RGB.
    #[must_use]
    pub const fn is_rgb(self) -> bool {
        matches!(self, Self::RgbH | Self::RgbV)
    }

    /// Returns true iff geo is a known geometry and is BGR.
    #[must_use]
    pub const fn is_bgr(self) -> bool {
        matches!(self, Self::BgrH | Self::BgrV)
    }

    /// Returns true iff geo is a known geometry and is horizontal.
    #[must_use]
    pub const fn is_h(self) -> bool {
        matches!(self, Self::RgbH | Self::BgrH)
    }

    /// Returns true iff geo is a known geometry and is vertical.
    #[must_use]
    pub const fn is_v(self) -> bool {
        matches!(self, Self::RgbV | Self::BgrV)
    }
}

bitflags! {
    #[derive(Debug, Default, Clone, Copy, Eq, PartialEq, Hash)]
    pub struct Flags: u8 {
        const UseDeviceIndependentFonts = 1 << 0;

        /// Use internal MSAA to render to non-MSAA GPU surfaces.
        const DynamicMSAA = 1 << 1;

        /// If set, all rendering will have dithering enabled.
        /// Currently this only impacts GPU backends
        const AlwaysDither = 1 << 2;
    }
}

/// `SurfaceProps` describes properties and constraints of a given Surface.
///
/// The rendering engine can parse these during drawing, and can sometimes
/// optimize its performance (e.g. disabling an expensive feature).
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct SurfaceProps {
    flags: Flags,
    pixel_geometry: PixelGeometry,
}

impl Default for SurfaceProps {
    fn default() -> Self {
        Self::new()
    }
}

impl SurfaceProps {
    /// No flags, unknown pixel geometry.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            flags: Flags::empty(),
            pixel_geometry: PixelGeometry::Unknown,
        }
    }

    #[must_use]
    pub const fn with_geometry(flags: Flags, pixel_geometry: PixelGeometry) -> Self {
        Self {
            flags,
            pixel_geometry,
        }
    }

    #[must_use]
    pub const fn flags(&self) -> Flags {
        self.flags
    }

    #[must_use]
    pub const fn pixel_geometry(&self) -> PixelGeometry {
        self.pixel_geometry
    }

    #[must_use]
    pub const fn is_use_device_independent_fonts(&self) -> bool {
        self.flags.contains(Flags::UseDeviceIndependentFonts)
    }

    #[must_use]
    pub const fn is_always_dither(&self) -> bool {
        self.flags.contains(Flags::AlwaysDither)
    }
}
