// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that can be found
// in the LICENSE file.

#[derive(Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
pub enum AlphaOption {
    Ignore,
    BlendOnBlack,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
pub enum Downsample {
    /// Reduction by a factor of two in both the horizontal and vertical directions.
    K420,

    /// Reduction by a factor of two in the horizontal direction.
    K422,

    /// No downsampling.
    K444,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Options {
    /// quality must be in `[0, 100]` where 0 corresponds to the lowest quality.
    pub quality: i32,

    /// Choose the downsampling factor for the U and V components.
    ///
    /// This is only meaningful if the `src` is not Gray, since Gray will not be encoded as YUV.
    ///
    /// This is ignored in favor of src's subsampling when src is an `YUVAPixmaps`.
    pub downsample: Downsample,

    /// Jpegs must be opaque.
    ///
    /// This instructs the encoder on how to handle input images with alpha.
    /// The default is to ignore the alpha channel and treat the image as opaque.
    /// Another option is to blend the pixels onto a black background before encoding.
    /// In the second case, the encoder supports linear or legacy blending.
    pub alpha_option: AlphaOption,

    /// Optional XMP metadata.
    pub xmp_metadata: Vec<u8>,

    ///  An optional ICC profile to override the default behavior.
    ///
    /// The default behavior is to generate an ICC profile using a primary matrix and
    /// analytic transfer function. If the color space of src cannot be represented
    /// in this way (e.g, it is HLG or PQ), then no profile will be embedded.
    //icc_profile: Option<IccProfile>,
    icc_profile_description: String,
}

impl Default for Options {
    fn default() -> Self {
        Self {
            quality: 100,
            downsample: Downsample::K420,
            alpha_option: AlphaOption::Ignore,
            xmp_metadata: Vec::new(),
            //icc_profile: None,
            icc_profile_description: String::new(),
        }
    }
}
