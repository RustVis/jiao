// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that can be found
// in the LICENSE file.

//! Blends are operators that take in two colors (source, destination) and return a new color.

/// Many of color operators operate the same on all 4 components: red, green, blue, alpha. For these,
/// we just document what happens to one component, rather than naming each one separately.
///
/// Different `ColorTypes` have different representations for color components:
///     8-bit: 0..255
///     6-bit: 0..63
///     5-bit: 0..31
///     4-bit: 0..15
///    floats: 0...1
///
/// The documentation is expressed as if the component values are always 0..1 (floats).
///
/// For brevity, the documentation uses the following abbreviations
/// s  : source
/// d  : destination
/// sa : source alpha
/// da : destination alpha
///
/// Results are abbreviated
/// r  : if all 4 components are computed in the same manner
/// ra : result alpha component
/// rc : result "color": red, green, blue components
#[repr(u8)]
#[derive(Debug, Default, Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum BlendMode {
    /// r = 0
    #[default]
    Clear,

    /// r = s
    Src,

    /// r = d
    Dst,

    /// r = s + (1-sa)*d
    SrcOver,

    /// r = d + (1-da)*s
    DstOver,

    /// r = s * da
    SrcIn,

    /// r = d * sa
    DstIn,

    ///  r = s * (1-da)
    SrcOut,

    /// r = d * (1-sa)
    DstOut,

    /// r = s*da + d*(1-sa)
    SrcATop,

    /// r = d*sa + s*(1-da)
    DstATop,

    /// r = s*(1-da) + d*(1-sa)
    Xor,

    /// r = min(s + d, 1)
    Plus,

    /// r = s*d
    Modulate,

    /// r = s + d - s*d
    Screen,

    /// multiply or screen, depending on destination
    Overlay,

    /// rc = s + d - max(s*da, d*sa), ra = `SrcOver`
    Darken,

    /// rc = s + d - min(s*da, d*sa), ra = `SrcOver`
    Lighten,

    /// brighten destination to reflect source
    ColorDodge,

    /// darken destination to reflect source
    ColorBurn,

    /// multiply or screen, depending on source
    HardLight,

    /// lighten or darken, depending on source
    SoftLight,

    /// rc = s + d - 2*(min(s*da, d*sa)), ra = `SrcOver`
    Difference,

    /// rc = s + d - two(s*d), ra = `SrcOver`
    Exclusion,

    /// r = s*(1-da) + d*(1-sa) + s*d
    Multiply,

    /// hue of source with saturation and luminosity of destination
    Hue,

    /// saturation of source with hue and luminosity of destination
    Saturation,

    /// hue and saturation of source with luminosity of destination
    Color,

    /// luminosity of source with hue and saturation of destination
    Luminosity,
}

/// last porter duff blend mode
pub const LAST_COEFF_MODE: BlendMode = BlendMode::Screen;

/// last blend mode operating separately on components
pub const LAST_SEPARABLE_MODE: BlendMode = BlendMode::Multiply;

/// last valid value
pub const LAST_BLEND_MODE: BlendMode = BlendMode::Luminosity;

pub const BLEND_MODE_COUNT: usize = LAST_BLEND_MODE as usize + 1;

/// For Porter-Duff `BlendModes` (those <= `LAST_COEFF_MODE`), these coefficients describe the blend
/// equation used.
///
/// Coefficient-based blend modes specify an equation:
/// ('dstCoeff' * dst + 'srcCoeff' * src), where the coefficient values are constants, functions of
/// the src or dst alpha, or functions of the src or dst color.
#[repr(u8)]
#[derive(Debug, Default, Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum BlendModeCoeff {
    /// 0
    #[default]
    Zero,

    /// 1
    One,

    /// src color
    SC,

    /// inverse src color (i.e. 1 - sc)
    ISC,

    /// dst color
    DC,

    /// inverse dst color (i.e. 1 - dc)
    IDC,

    /// src alpha
    SA,

    /// inverse src alpha (i.e. 1 - sa)
    ISA,

    /// dst alpha
    DA,

    /// inverse dst alpha (i.e. 1 - da)
    IDA,
}

pub const COEFF_COUNT: usize = BlendModeCoeff::IDA as usize + 1;

impl BlendMode {
    /// Returns true if `mode` is a coefficient-based blend mode (<= `LAST_COEFF_MODE`).
    ///
    /// If true is returned, the mode's src and dst coefficient functions are set in `src` and `dst`.
    ///
    /// If `out_src` is true, the `out` is as src.
    /// Else the `out` is as dest.
    #[must_use]
    pub fn as_coeff(self, out: &mut BlendModeCoeff, out_src: bool) -> bool {
        let coeffs = [
            // For Porter-Duff blend functions, color = src * src coeff + dst * dst coeff
            // src coeff                  dst coeff                     blend func
            // ----------------------     -----------------------       ----------
            [BlendModeCoeff::Zero, BlendModeCoeff::Zero], // clear
            [BlendModeCoeff::One, BlendModeCoeff::Zero],  // src
            [BlendModeCoeff::Zero, BlendModeCoeff::One],  // dst
            [BlendModeCoeff::One, BlendModeCoeff::ISA],   // src-over
            [BlendModeCoeff::IDA, BlendModeCoeff::One],   // dst-over
            [BlendModeCoeff::DA, BlendModeCoeff::Zero],   // src-in
            [BlendModeCoeff::Zero, BlendModeCoeff::SA],   // dst-in
            [BlendModeCoeff::IDA, BlendModeCoeff::Zero],  // src-out
            [BlendModeCoeff::Zero, BlendModeCoeff::ISA],  // dst-out
            [BlendModeCoeff::DA, BlendModeCoeff::ISA],    // src-atop
            [BlendModeCoeff::IDA, BlendModeCoeff::SA],    // dst-atop
            [BlendModeCoeff::IDA, BlendModeCoeff::ISA],   // xor
            [BlendModeCoeff::One, BlendModeCoeff::One],   // plus
            [BlendModeCoeff::Zero, BlendModeCoeff::SC],   // modulate
            [BlendModeCoeff::One, BlendModeCoeff::ISC],   // screen
        ];

        if self > Self::Screen {
            return false;
        }

        let index = self as usize;
        if out_src {
            *out = coeffs[index][0];
        } else {
            *out = coeffs[index][1];
        }
        true
    }

    /// Returns name of blendMode.
    #[must_use]
    #[inline]
    pub const fn name(self) -> &'static str {
        match self {
            Self::Clear => "Clear",
            Self::Src => "Src",
            Self::Dst => "Dst",
            Self::SrcOver => "SrcOver",
            Self::DstOver => "DstOver",
            Self::SrcIn => "SrcIn",
            Self::DstIn => "DstIn",
            Self::SrcOut => "SrcOut",
            Self::DstOut => "DstOut",
            Self::SrcATop => "SrcATop",
            Self::DstATop => "DstATop",
            Self::Xor => "Xor",
            Self::Plus => "Plus",
            Self::Modulate => "Modulate",
            Self::Screen => "Screen",

            Self::Overlay => "Overlay",
            Self::Darken => "Darken",
            Self::Lighten => "Lighten",
            Self::ColorDodge => "ColorDodge",
            Self::ColorBurn => "ColorBurn",
            Self::HardLight => "HardLight",
            Self::SoftLight => "SoftLight",
            Self::Difference => "Difference",
            Self::Exclusion => "Exclusion",
            Self::Multiply => "Multiply",

            Self::Hue => "Hue",
            Self::Saturation => "Saturation",
            Self::Color => "Color",
            Self::Luminosity => "Luminosity",
        }
    }
}
