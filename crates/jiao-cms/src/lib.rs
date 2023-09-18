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
    pub fn eval(&self, _val: f32) -> f32 {
        unimplemented!()
    }

    #[must_use]
    pub fn invert(&self) -> Option<Self> {
        unimplemented!()
    }

    /// Identify which kind of transfer function is encoded in an `TransferFunction`
    #[must_use]
    pub fn get_type(&self) -> TFType {
        unimplemented!()
    }

    /// We can jam a couple alternate transfer function forms into `TransferFunction`,
    /// including those matching the general forms of the SMPTE ST 2084 PQ function or HLG.
    ///
    /// `PQish`:
    /// ```txt
    ///   max(A + B|encoded|^C, 0)
    ///   linear = sign(encoded) * (------------------------) ^ F
    /// ```
    pub fn make_pqish(&mut self, _a: f32, _b: f32, _c: f32, _d: f32, _e: f32, _f: f32) -> bool {
        unimplemented!()
    }

    /// `HLGish`:
    /// ```txt
    ///   { K * sign(encoded) * ( (R|encoded|)^G )          when 0   <= |encoded| <= 1/R
    ///   linear = { K * sign(encoded) * ( e^(a(|encoded|-c)) + b )  when 1/R <  |encoded|
    /// ```
    pub fn make_scaled_hlgish(
        &mut self,
        _k: f32,
        _r: f32,
        _g: f32,
        _a: f32,
        _b: f32,
        _c: f32,
    ) -> bool {
        unimplemented!()
    }

    /// Compatibility shim with K=1 for old callers.
    #[allow(clippy::many_single_char_names)]
    pub fn make_hlgish(&mut self, r: f32, g: f32, a: f32, b: f32, c: f32) -> bool {
        self.make_scaled_hlgish(1.0, r, g, a, b, c)
    }

    /// PQ mapping encoded [0,1] to linear [0,1].
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
        unimplemented!()
    }

    #[must_use]
    pub fn is_pqish(&self) -> bool {
        unimplemented!()
    }

    #[must_use]
    pub fn is_hlgish(&self) -> bool {
        unimplemented!()
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
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TFType {
    Invalid,
    SRGBish,
    PQish,
    HLGish,
    HLGinvish,
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
        //memset(p, 0, sizeof(*p));
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
