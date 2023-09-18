// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that can be found
// in the LICENSE file.

#![deny(
    warnings,
    clippy::all,
    clippy::cargo,
    clippy::nursery,
    clippy::pedantic
)]
#![allow(dead_code)]

use std::f32::EPSILON;

/// A row-major 3x3 matrix (ie vals[row][col])
#[derive(Debug, Default, Clone, PartialEq)]
pub struct Matrix3x3 {
    vals: [[f32; 3]; 3],
}

impl Matrix3x3 {
    /// It is _not_ safe to alias the pointers to invert in-place.
    #[must_use]
    pub fn invert(&self) -> Option<Self> {
        unimplemented!()
    }

    #[must_use]
    pub fn concat(&self, _other: &Self) -> Self {
        unimplemented!()
    }

    /// Returns a matrix to adapt XYZ color from given the whitepoint to D50.
    #[must_use]
    pub fn adapt_to_xyzd50(_wx: f32, _wy: f32) -> Option<Self> {
        unimplemented!()
    }

    /// Returns a matrix to convert RGB color into XYZ adapted to D50, given the
    /// primaries and whitepoint of the RGB model.
    #[must_use]
    #[allow(clippy::too_many_arguments)]
    pub const fn primaries_to_xyzd50(
        _rx: f32,
        _ry: f32,
        _gx: f32,
        _gy: f32,
        _bx: f32,
        _by: f32,
        _wx: f32,
        _wy: f32,
    ) -> Option<Self> {
        unimplemented!()
    }
}

/// A row-major 3x4 matrix (ie vals[row][col])
#[derive(Debug, Default, Clone, PartialEq)]
pub struct Matrix3x4 {
    vals: [[f32; 4]; 3],
}

/// A transfer function mapping encoded values to linear values,
/// represented by this 7-parameter piecewise function:
///
/// ```txt
///   linear = sign(encoded) *  (c*|encoded| + f)       , 0 <= |encoded| < d
///          = sign(encoded) * ((a*|encoded| + b)^g + e), d <= |encoded|
/// ```
///
/// (A simple gamma transfer function sets g to gamma and a to 1.)
#[derive(Debug, Default, Clone, PartialEq)]
pub struct TransferFunction {
    g: f32,
    a: f32,
    b: f32,
    c: f32,
    d: f32,
    e: f32,
    f: f32,
}

impl TransferFunction {
    #[must_use]
    #[allow(clippy::suboptimal_flops)]
    pub fn eval(&self, mut x: f32) -> f32 {
        let sign = if x < 0.0 { -1.0 } else { 1.0 };
        x *= sign;

        let mut pq = TF_PQish::default();
        let mut hlg = TF_HLGish::default();
        let type_ = self.classify(Some(&mut pq), Some(&mut hlg));

        match type_ {
            TFType::Invalid => 0.0,

            TFType::HLGish => {
                let k = hlg.k_minus_1 + 1.0;
                let exp = if x * hlg.r <= 1.0 {
                    (x * hlg.r).powf(hlg.g)
                } else {
                    ((x - hlg.c) * hlg.a).exp() + hlg.b
                };
                k * sign * exp
            }

            TFType::HLGinvish => {
                // invert() inverts R, G, and a for HLGinvish so this math is fast.
                let k = hlg.k_minus_1 + 1.0;
                x /= k;
                let exp = if x <= 1.0 {
                    hlg.r * x.powf(hlg.g)
                } else {
                    hlg.a * (x - hlg.b).log2() + hlg.c
                };
                sign * exp
            }

            TFType::SRGBish => {
                let exp = if x < self.d {
                    self.c * x + self.f
                } else {
                    (self.a * x + self.b).powf(self.g) + self.e
                };

                sign * exp
            }

            TFType::PQish => {
                sign * ((pq.a + pq.b * x.powf(pq.c)).max(0.0) / (pq.d + pq.e * x.powf(pq.c)))
                    .powf(pq.f)
            }
        }
    }

    #[must_use]
    pub fn invert(&self) -> Option<Self> {
        unimplemented!()
    }

    /// Identify which kind of transfer function is encoded in an `TransferFunction`
    #[must_use]
    pub fn get_type(&self) -> TFType {
        self.classify(None, None)
    }

    /// We can jam a couple alternate transfer function forms into `TransferFunction`,
    /// including those matching the general forms of the SMPTE ST 2084 PQ function or HLG.
    ///
    /// `PQish`:
    /// ```txt
    ///   max(A + B|encoded|^C, 0)
    ///   linear = sign(encoded) * (------------------------) ^ F
    /// ```
    #[allow(clippy::many_single_char_names)]
    pub fn make_pqish(&mut self, a: f32, b: f32, c: f32, d: f32, e: f32, f: f32) -> bool {
        self.g = TFType::PQish.marker();
        self.a = a;
        self.b = b;
        self.c = c;
        self.d = d;
        self.e = e;
        self.f = f;
        debug_assert!(self.is_pqish());
        true
    }

    /// `HLGish`:
    /// ```txt
    ///   { K * sign(encoded) * ( (R|encoded|)^G )          when 0   <= |encoded| <= 1/R
    ///   linear = { K * sign(encoded) * ( e^(a(|encoded|-c)) + b )  when 1/R <  |encoded|
    /// ```
    #[must_use]
    #[allow(clippy::many_single_char_names)]
    pub fn make_scaled_hlgish(&mut self, k: f32, r: f32, g: f32, a: f32, b: f32, c: f32) -> bool {
        self.g = TFType::HLGish.marker();
        self.a = r;
        self.b = g;
        self.c = a;
        self.d = b;
        self.e = c;
        self.f = k - 1.0;
        debug_assert!(self.is_hlgish());
        true
    }

    /// Compatibility shim with K=1 for old callers.
    #[must_use]
    #[allow(clippy::many_single_char_names)]
    pub fn make_hlgish(&mut self, r: f32, g: f32, a: f32, b: f32, c: f32) -> bool {
        self.make_scaled_hlgish(1.0, r, g, a, b, c)
    }

    /// PQ mapping encoded [0,1] to linear [0,1].
    #[must_use]
    pub fn make_pq(&mut self) -> bool {
        self.make_pqish(
            -107.0 / 128.0,
            1.0,
            32.0 / 2523.0,
            2413.0 / 128.0,
            -2392.0 / 128.0,
            8192.0 / 1305.0,
        )
    }

    /// HLG mapping encoded [0,1] to linear [0,12].
    #[must_use]
    pub fn make_hlg(&mut self) -> bool {
        self.make_hlgish(2.0, 2.0, 1.0 / 0.178_832_77, 0.284_668_92, 0.559_910_7)
    }

    /// Is this an ordinary sRGB-ish transfer function, or one of the HDR forms we support?
    #[must_use]
    pub fn is_srgbish(&self) -> bool {
        self.get_type() == TFType::SRGBish
    }

    #[must_use]
    pub fn is_pqish(&self) -> bool {
        self.get_type() == TFType::PQish
    }

    #[must_use]
    pub fn is_hlgish(&self) -> bool {
        self.get_type() == TFType::HLGish
    }

    #[must_use]
    pub const fn new_srgb() -> Self {
        unimplemented!()
    }

    #[must_use]
    pub const fn new_srgb_inverse() -> Self {
        unimplemented!()
    }

    #[must_use]
    pub const fn new_identity() -> Self {
        unimplemented!()
    }

    #[must_use]
    #[allow(clippy::cast_possible_truncation)]
    #[allow(clippy::cast_precision_loss)]
    #[allow(clippy::cast_sign_loss)]
    fn classify(&self, pq: Option<&mut TF_PQish>, hlg: Option<&mut TF_HLGish>) -> TFType {
        if self.g < 0.0 && (((self.g as i32) as f32) - self.g).abs() < EPSILON {
            // TODO: soundness checks for PQ/HLG like we do for sRGBish?
            match ((-(self.g) as i32) as u8).try_into() {
                Ok(TFType::PQish) => {
                    if let Some(pq) = pq {
                        pq.set_all(self.a);
                    }
                    return TFType::PQish;
                }
                Ok(TFType::HLGish) => {
                    if let Some(hlg) = hlg {
                        hlg.set_all(self.a);
                    }
                    return TFType::HLGish;
                }
                Ok(TFType::HLGinvish) => {
                    if let Some(hlg) = hlg {
                        hlg.set_all(self.a);
                    }
                    return TFType::HLGinvish;
                }
                _ => return TFType::Invalid,
            }
        }

        // Basic soundness checks for sRGBish transfer functions.
        if (self.a + self.b + self.c + self.d + self.e + self.f + self.g).is_finite()
            // a,c,d,g should be non-negative to make any sense.
            && self.a >= 0.0
            && self.c >= 0.0
            && self.d >= 0.0
            && self.g >= 0.0
            // Raising a negative value to a fractional tf->g produces complex numbers.
            && self.a.mul_add(self.d , self.b) >= 0.0
        {
            return TFType::SRGBish;
        }

        TFType::Invalid
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum TFType {
    Invalid = 0,
    SRGBish = 1,
    PQish = 2,
    HLGish = 3,
    HLGinvish = 4,
}

impl Default for TFType {
    fn default() -> Self {
        Self::Invalid
    }
}

impl TFType {
    #[must_use]
    fn marker(self) -> f32 {
        // We'd use different NaNs, but those aren't guaranteed to be preserved by WASM.
        -f32::from(self as u8)
    }
}

impl TryFrom<u8> for TFType {
    type Error = &'static str;

    fn try_from(val: u8) -> Result<Self, Self::Error> {
        match val {
            0 => Ok(Self::Invalid),
            1 => Ok(Self::SRGBish),
            2 => Ok(Self::PQish),
            3 => Ok(Self::HLGish),
            4 => Ok(Self::HLGinvish),
            _ => Err("Invalid TFType value"),
        }
    }
}

// Most transfer functions we work with are sRGBish.
// For exotic HDR transfer functions, we encode them using a tf.g that makes no sense,
// and repurpose the other fields to hold the parameters of the HDR functions.
#[derive(Debug, Default, Clone)]
#[allow(non_camel_case_types)]
struct TF_PQish {
    a: f32,
    b: f32,
    c: f32,
    d: f32,
    e: f32,
    f: f32,
}

impl TF_PQish {
    fn set_all(&mut self, val: f32) {
        self.a = val;
        self.b = val;
        self.c = val;
        self.d = val;
        self.e = val;
        self.f = val;
    }
}

// We didn't originally support a scale factor K for HLG, and instead just stored 0 in
// the unused `f` field of skcms_TransferFunction for HLGish and HLGInvish transfer functions.
// By storing f=K-1, those old unusued f=0 values now mean K=1, a noop scale factor.
#[derive(Debug, Default, Clone)]
#[allow(non_camel_case_types)]
struct TF_HLGish {
    r: f32,
    g: f32,
    a: f32,
    b: f32,
    c: f32,
    k_minus_1: f32,
}

impl TF_HLGish {
    fn set_all(&mut self, val: f32) {
        self.r = val;
        self.g = val;
        self.a = val;
        self.b = val;
        self.c = val;
        self.k_minus_1 = val;
    }
}

/// Unified representation of 'curv' or 'para' tag data, or a 1D table from 'mft1' or 'mft2'
#[repr(C)]
#[derive(Debug, Default, Clone)]
pub struct Curve {
    pub table_entries: u32,
    pub parametric: TransferFunction,
}

impl Curve {
    /// Practical test that answers: Is curve roughly the inverse of `inv_tf`?
    ///
    /// Typically used by passing the inverse of a known parametric transfer function
    /// (like `sRGB`), to determine if a particular curve is very close to `sRGB`.
    #[must_use]
    pub fn are_approximate_inverses(&self, _inv_tf: &TransferFunction) -> bool {
        unimplemented!()
    }

    /// Returns max error.
    #[must_use]
    pub fn approximate(&self, _approx: &TransferFunction) -> Option<f32> {
        unimplemented!()
    }
}

/// Complex transforms between device space (A) and profile connection space (B):
/// A2B:  device -> [ "A" curves -> CLUT ] -> [ "M" curves -> matrix ] -> "B" curves -> PCS
/// B2A:  device <- [ "A" curves <- CLUT ] <- [ "M" curves <- matrix ] <- "B" curves <- PCS
#[derive(Debug, Default, Clone)]
pub struct A2B {
    // Optional: N 1D "A" curves, followed by an N-dimensional CLUT.
    // If input_channels == 0, these curves and CLUT are skipped,
    // Otherwise, input_channels must be in [1, 4].
    pub input_channels: u32,
    pub input_curves: [Curve; 4],
    pub grid_points: [u8; 4],

    pub grid_8: Vec<u8>,
    pub grid_16: Vec<u8>,

    // Optional: 3 1D "M" curves, followed by a color matrix.
    // If matrix_channels == 0, these curves and matrix are skipped,
    // Otherwise, matrix_channels must be 3.
    pub matrix_channels: u32,
    pub matrix_curves: [Curve; 3],
    pub matrix: Matrix3x4,

    // Required: 3 1D "B" curves. Always present, and output_channels must be 3.
    pub output_channels: u32,
    pub output_curves: [Curve; 3],
}

#[derive(Debug, Default, Clone)]
pub struct B2A {
    // Required: 3 1D "B" curves. Always present, and input_channels must be 3.
    pub input_channels: u32,
    pub input_curves: [Curve; 3],

    // Optional: a color matrix, followed by 3 1D "M" curves.
    // If matrix_channels == 0, this matrix and these curves are skipped,
    // Otherwise, matrix_channels must be 3.
    pub matrix_channels: u32,
    pub matrix: Matrix3x4,
    pub matrix_curves: [Curve; 3],

    // Optional: an N-dimensional CLUT, followed by N 1D "A" curves.
    // If output_channels == 0, this CLUT and these curves are skipped,
    // Otherwise, output_channels must be in [1, 4].
    pub output_channels: u32,
    pub grid_points: [u8; 4],
    pub grid_8: Vec<u8>,
    pub grid_16: Vec<u8>,
    pub output_curves: [Curve; 4],
}

#[derive(Debug, Default, Clone, Copy)]
pub struct CICP {
    pub color_primaries: u8,
    pub transfer_characteristics: u8,
    pub matrix_coefficients: u8,
    pub video_full_range_flag: u8,
}

#[derive(Debug, Default, Clone)]
#[allow(clippy::struct_excessive_bools)]
pub struct ICCProfile {
    pub buffer: Vec<u8>,

    pub size: u32,
    pub data_color_space: Signature,
    pub pcs: Signature,
    pub tag_count: u32,

    // parse() will set commonly-used fields for you when possible:
    // If we can parse red, green and blue transfer curves from the profile,
    // trc will be set to those three curves, and has_trc will be true.
    pub has_trc: bool,
    pub trc: [Curve; 3],

    /// If this profile's gamut can be represented by a 3x3 transform to XYZD50,
    /// parse() sets `to_xyzd50` to that transform and has_to_xyzd50 to true.
    pub has_to_xyzd50: bool,
    pub to_xyzd50: Matrix3x3,

    // If the profile has a valid A2B0 or A2B1 tag, Parse() sets A2B to
    // that data, and has_A2B to true.  ParseWithA2BPriority() does the
    // same following any user-provided prioritization of A2B0, A2B1, or A2B2.
    pub has_a2b: bool,
    pub a2b: A2B,

    // If the profile has a valid B2A0 or B2A1 tag, Parse() sets B2A to
    // that data, and has_B2A to true.  ParseWithA2BPriority() does the
    // same following any user-provided prioritization of B2A0, B2A1, or B2A2.
    pub has_b2a: bool,
    pub b2a: B2A,

    // If the profile has a valid CICP tag, Parse() sets CICP to that data,
    // and has_CICP to true.
    pub has_cicp: bool,
    pub cicp: CICP,
}

impl ICCProfile {
    /// The `sRGB` color profile is so commonly used that we offer a canonical `ICCProfile` for it.
    #[must_use]
    pub const fn srgb_profile() -> Self {
        unimplemented!()
    }

    /// Ditto for XYZD50, the most common profile connection space.
    #[must_use]
    pub const fn xyzd50_profile() -> Self {
        unimplemented!()
    }

    /// Practical equality test for two `ICCProfiles`.
    /// The implementation is subject to change, but it will always try to answer
    /// "can I substitute A for B?" and "can I skip transforming from A to B?".
    #[must_use]
    pub fn approximately_equal(&self, _other: &Self) -> bool {
        unimplemented!()
    }

    /// Answering the question for all three TRC curves of the given profile. Again,
    /// passing `sRGB_InverseTransferFunction` as `inv_tf` will answer the question:
    /// "Does this profile have a transfer function that is very close to `sRGB`?"
    #[must_use]
    pub fn trcs_are_approximate_inverse(&self, _inv_tf: &TransferFunction) -> bool {
        unimplemented!()
    }

    #[must_use]
    pub fn get_chad(&self) -> Option<Matrix3x3> {
        unimplemented!()
    }

    #[must_use]
    pub fn get_wtpt(&self) -> Option<[f32; 3]> {
        unimplemented!()
    }

    /// If profile can be used as a destination in Transform, return true.
    /// Otherwise, attempt to rewrite it with approximations where reasonable.
    ///
    /// If successful, return true. If no reasonable approximation exists,
    /// leave the profile unchanged and return false.
    #[must_use]
    pub fn make_usable_as_destination(&mut self) -> bool {
        unimplemented!()
    }

    /// If profile can be used as a destination with a single parametric transfer function (ie for
    /// rasterization), return true.
    /// Otherwise, attempt to rewrite it with approximations where reasonable.
    ///
    /// If successful, return true. If no reasonable approximation exists, leave the
    /// profile unchanged and return false.
    #[must_use]
    pub fn make_usable_as_destination_with_single_curve(&mut self) -> bool {
        unimplemented!()
    }

    /// Parse an ICC profile and return true if possible, otherwise return false.
    ///
    /// Selects an A2B profile (if present) according to priority list (each entry 0-2).
    /// The buffer is not copied; it must remain valid as long as the `ICCProfile` will be used.
    #[must_use]
    pub fn parse_with_a2b_priority(_buf: &[u8], _priorities: &[i32]) -> Option<Self> {
        unimplemented!()
    }

    fn parse(buf: &[u8]) -> Option<Self> {
        // For continuity of existing user expectations,
        // prefer A2B0 (perceptual) over A2B1 (relative colormetric), and ignore A2B2 (saturation).
        let priorities = &[0, 1];
        Self::parse_with_a2b_priority(buf, priorities)
    }

    /// Utilities for programmatically constructing profiles
    fn init(&mut self) {
        *self = Self::default();
        self.data_color_space = Signature::RGB;
        self.pcs = Signature::XYZ;
    }

    fn set_transfer_function(&mut self, tf: &TransferFunction) {
        self.has_trc = true;
        for i in 0..3 {
            self.trc[i].table_entries = 0;
            self.trc[i].parametric = tf.clone();
        }
    }

    fn set_xyzd50(&mut self, matrix: Matrix3x3) {
        self.has_to_xyzd50 = true;
        self.to_xyzd50 = matrix;
    }
}

/// These are common ICC signature values
#[repr(u32)]
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum Signature {
    // data_color_space
    CMYK = 0x434D_594B,
    Gray = 0x4752_4159,
    RGB = 0x5247_4220,

    // pcs
    Lab = 0x4C61_6220,
    XYZ = 0x5859_5A20,
}

impl Default for Signature {
    fn default() -> Self {
        Self::RGB
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
#[allow(non_camel_case_types)]
pub enum PixelFormat {
    A_8,
    A_8_,
    G_8,
    G_8_,
    RGBA_8888_Palette8,
    BGRA_8888_Palette8,

    RGB_565,
    BGR_565,

    ABGR_4444,
    ARGB_4444,

    RGB_888,
    BGR_888,
    RGBA_8888,
    BGRA_8888,
    /// Automatic sRGB encoding / decoding.
    RGBA_8888_sRGB,
    /// (Generally used with linear transfer functions.)
    BGRA_8888_sRGB,

    RGBA_1010102,
    BGRA_1010102,

    /// Little-endian. Pointers must be 16-bit aligned.
    RGB_161616LE,
    BGR_161616LE,
    RGBA_16161616LE,
    BGRA_16161616LE,

    /// Big-endian. Pointers must be 16-bit aligned.
    RGB_161616BE,
    BGR_161616BE,
    RGBA_16161616BE,
    BGRA_16161616BE,

    /// 1-5-10 half-precision float in [0,1]
    RGB_hhh_Norm,
    /// Pointers must be 16-bit aligned.
    BGR_hhh_Norm,
    RGBA_hhhh_Norm,
    BGRA_hhhh_Norm,

    /// 1-5-10 half-precision float.
    RGB_hhh,
    /// Pointers must be 16-bit aligned.
    BGR_hhh,
    RGBA_hhhh,
    BGRA_hhhh,

    /// 1-8-23 single-precision float (the normal kind).
    RGB_fff,
    /// Pointers must be 32-bit aligned.
    BGR_fff,
    RGBA_ffff,
    BGRA_ffff,

    /// Note: This is located here to signal no clamping.
    RGB_101010x_XR,
    /// Compatible with MTLPixelFormatBGR10_XR.
    BGR_101010x_XR,
}

/// We always store any alpha channel linearly.  In the chart below, tf-1() is the inverse
/// transfer function for the given color profile (applying the transfer function linearizes).
///
/// We treat opaque as a strong requirement, not just a performance hint: we will ignore
/// any source alpha and treat it as 1.0, and will make sure that any destination alpha
/// channel is filled with the equivalent of 1.0.
///
/// We used to offer multiple types of premultiplication, but now just one, `PremulAsEncoded`.
/// This is the premul you're probably used to working with.
pub enum AlphaFormat {
    /// alpha is always opaque. tf-1(r),   tf-1(g),   tf-1(b),   1.0
    Opaque,

    /// alpha and color are unassociated. tf-1(r),   tf-1(g),   tf-1(b),   a
    Unpremul,

    // premultiplied while encoded. tf-1(r)*a, tf-1(g)*a, tf-1(b)*a, a
    PremulAsEncoded,
}

//
///// Convert npixels pixels from src format and color profile to dst format and color profile
///// and return true, otherwise return false.  It is safe to alias dst == src if dstFmt == srcFmt.
//pub fn transform(const void*             src,
//                               PixelFormat       srcFmt,
//                               AlphaFormat       srcAlpha,
//                               const ICCProfile* srcProfile,
//                               void*                   dst,
//                               PixelFormat       dstFmt,
//                               AlphaFormat       dstAlpha,
//                               const ICCProfile* dstProfile,
//                               size_t                  npixels) -> bool {
//    unimplemented!()
//}
//
///// As `transform()`, supporting srcFmts with a palette.
//pub fn transform_with_palette(const void*             src,
//                                          PixelFormat       srcFmt,
//                                          AlphaFormat       srcAlpha,
//                                          const ICCProfile* srcProfile,
//                                          void*                   dst,
//                                          PixelFormat       dstFmt,
//                                          AlphaFormat       dstAlpha,
//                                          const ICCProfile* dstProfile,
//                                          size_t                  npixels,
//                                          const void*             palette) -> bool {
//    unimplemented!()
//}
