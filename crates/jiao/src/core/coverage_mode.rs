// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

/// `CoverageMode` describes geometric operations (ala `Region::Op`) that can be applied
/// to coverage bytes.
///
/// These can be thought of as variants of porter-duff (`BlendMode`) modes,
/// but only applied to the alpha channel.
///
/// See `MaskFilter` for ways to use these when combining two different masks.
#[repr(u8)]
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum SkCoverageMode {
    /// A ∪ B    A+B-A*B
    Union,

    /// A ∩ B    A*B
    Intersect,

    /// A - B    A*(1-B)
    Difference,

    /// B - A    B*(1-A)
    ReverseDifference,

    /// A ⊕ B    A+B-2*A*B
    Xor,
}
