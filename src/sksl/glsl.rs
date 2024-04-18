// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that can be found
// in the LICENSE file.

/// Limited set of GLSL versions we build shaders for.
///
/// Caller should round down the GLSL version to one of these enums.
#[derive(Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
pub enum GLSLGeneration {
    /// Desktop GLSL 1.10 and ES2 shading language (based on desktop GLSL 1.20)
    V110,
    //V100ES,
    /// Desktop GLSL 1.30
    V130,

    /// Desktop GLSL 1.40
    V140,

    /// Desktop GLSL 1.50
    V150,

    /// Desktop GLSL 3.30, and ES GLSL 3.00
    V330,
    //V300ES,
    /// Desktop GLSL 4.00
    V400,

    /// Desktop GLSL 4.20
    V420,

    /// ES GLSL 3.10 only
    V310ES,

    /// ES GLSL 3.20
    V320ES,
}
