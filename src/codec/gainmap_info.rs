// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that can be found
// in the LICENSE file.

use crate::core::color::Color4f;
use crate::core::color_space::ColorSpace;

/// Gainmap rendering parameters.
///
/// Suppose our display has HDR to SDR ratio of H and we wish to display an image with gainmap on this display.
/// Let B be the pixel value from the base image in a color space that has the primaries of the base image
/// and a linear transfer function.
/// Let G be the pixel value from the gainmap. Let D be the output pixel in the same color space as B.
/// The value of D is computed as follows:
///
/// First, let W be a weight parameter determing how much the gainmap will be applied.
///   W = clamp((log(H)                - log(fDisplayRatioSdr)) /
///             (log(fDisplayRatioHdr) - log(fDisplayRatioSdr), 0, 1)
///
/// Next, let L be the gainmap value in log space. We compute this from the value G that was
/// sampled from the texture as follows:
///   L = mix(log(fGainmapRatioMin), log(fGainmapRatioMax), pow(G, fGainmapGamma))
///
/// Finally, apply the gainmap to compute D, the displayed pixel. If the base image is SDR then
/// compute:
///   D = (B + fEpsilonSdr) * exp(L * W) - fEpsilonHdr
/// If the base image is HDR then compute:
///   D = (B + fEpsilonHdr) * exp(L * (W - 1)) - fEpsilonSdr
///
/// In the above math, `log()` is a natural logarithm and `exp()` is natural exponentiation. Note,
/// however, that the base used for the `log()` and `exp()` functions does not affect the results of
/// the computation (it cancels out, as long as the same base is used throughout).
///
/// This product includes Gain Map technology under license by Adobe.
#[derive(Debug, PartialEq)]
pub struct GainmapInfo {
    /// Parameters for converting the gainmap from its image encoding to log space.
    ///
    /// These are specified per color channel. The alpha value is unused.
    pub ratio_min: Color4f,
    pub ratio_max: Color4f,
    pub gamma: Color4f,

    /// Parameters sometimes used in gainmap computation to avoid numerical instability.
    pub epsilon_sdr: Color4f,
    pub epsilon_hdr: Color4f,

    /// If the output display's HDR to SDR ratio is less or equal than fDisplayRatioSdr then the SDR
    /// rendition is displayed.
    ///
    /// If the output display's HDR to SDR ratio is greater or equal than
    /// fDisplayRatioHdr then the HDR rendition is displayed. If the output display's HDR to SDR
    /// ratio is between these values then an interpolation between the two is displayed using the
    /// math above.
    pub display_ratio_sdr: f32,
    pub display_ratio_hdr: f32,

    pub base_image_type: BaseImageType,

    /// If specified, color space to apply the gainmap in, otherwise the base image's color space
    /// is used.
    ///
    /// Only the color primaries are used, the transfer function is irrelevant.
    pub math_color_space: Option<ColorSpace>,
}

impl Default for GainmapInfo {
    fn default() -> Self {
        Self {
            ratio_min: Color4f::from_rgba(1.0, 1.0, 1.0, 1.0),
            ratio_max: Color4f::from_rgba(2.0, 2.0, 2.0, 1.0),
            gamma: Color4f::from_rgba(1.0, 1.0, 1.0, 1.0),

            epsilon_sdr: Color4f::from_rgba(0.0, 0.0, 0.0, 1.0),
            epsilon_hdr: Color4f::from_rgba(0.0, 0.0, 0.0, 1.0),

            display_ratio_sdr: 1.0,
            display_ratio_hdr: 2.0,

            base_image_type: BaseImageType::SDR,
            math_color_space: None,
        }
    }
}

/// Whether the base image is the SDR image or the HDR image.
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum BaseImageType {
    SDR,
    HDR,
}
