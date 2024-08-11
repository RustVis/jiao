// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that can be found
// in the LICENSE file.

#[derive(Debug, Default, Clone, Copy, Eq, PartialEq, Hash)]
pub enum FilterMode {
    /// Single sample point (nearest neighbor)
    #[default]
    Nearest,

    /// Interporate between 2x2 sample points (bilinear interpolation)
    Linear,
}

#[derive(Debug, Default, Clone, Copy, Eq, PartialEq, Hash)]
pub enum MipmapMode {
    /// Ignore mipmap levels, sample from the "base"
    #[default]
    None,

    /// Sample from the nearest level
    Nearest,

    /// Interpolate between the two nearest levels
    Linear,
}

/// Specify B and C (each between 0...1) to create a shader that applies the corresponding
/// cubic reconstruction filter to the image.
///
/// Example values:
/// ```txt
/// B = 1/3, C = 1/3        "Mitchell" filter
/// B = 0,   C = 1/2        "Catmull-Rom" filter
/// ```
///
/// See ["Reconstruction Filters in Computer Graphics" Don P. Mitchell, Arun N. Netravali, 1988](
/// https://www.cs.utexas.edu/~fussell/courses/cs384g-fall2013/lectures/mitchell/Mitchell.pdf)
///
/// [Desmos worksheet](https://www.desmos.com/calculator/aghdpicrvr)
/// [Nice overview](https://entropymine.com/imageworsener/bicubic/)
#[derive(Debug, Default, Clone, PartialEq)]
pub struct CubicResampler {
    val_b: f32,
    val_c: f32,
}

impl CubicResampler {
    #[must_use]
    pub const fn new() -> Self {
        Self {
            val_b: 0.0,
            val_c: 0.0,
        }
    }

    /// Historic default for `FilterQuality::High`
    #[must_use]
    #[inline]
    pub fn mitchell() -> Self {
        Self {
            val_b: 1.0 / 3.0,
            val_c: 1.0 / 3.0,
        }
    }

    #[must_use]
    #[inline]
    pub fn catmull_rom() -> Self {
        Self {
            val_b: 0.0,
            val_c: 1.0 / 2.0,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct SamplingOptions {
    max_aniso: i32,
    use_cubic: bool,
    cubic: CubicResampler,
    filter: FilterMode,
    mipmap: MipmapMode,
}

impl Default for SamplingOptions {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

impl SamplingOptions {
    #[must_use]
    #[inline]
    pub const fn new() -> Self {
        Self {
            max_aniso: 0,
            use_cubic: false,
            cubic: CubicResampler::new(),
            filter: FilterMode::Nearest,
            mipmap: MipmapMode::None,
        }
    }

    #[must_use]
    pub const fn with_modes(filter: FilterMode, mipmap: MipmapMode) -> Self {
        Self {
            filter,
            mipmap,
            ..Self::new()
        }
    }

    #[must_use]
    pub const fn with_filter(filter: FilterMode) -> Self {
        Self {
            filter,
            ..Self::new()
        }
    }

    #[must_use]
    pub const fn with_resampler(cubic: CubicResampler) -> Self {
        Self {
            cubic,
            use_cubic: true,
            ..Self::new()
        }
    }

    #[must_use]
    pub fn with_aniso(&self, max_aniso: i32) -> Self {
        Self {
            max_aniso: max_aniso.max(1),
            ..Self::new()
        }
    }

    #[must_use]
    #[inline]
    pub const fn is_aniso(&self) -> bool {
        self.max_aniso != 0
    }
}
