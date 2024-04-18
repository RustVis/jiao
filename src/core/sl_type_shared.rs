// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that can be found
// in the LICENSE file.

#![allow(clippy::match_like_matches_macro)]

/// Types of shader-language-specific boxed variables we can create.
#[repr(u8)]
#[derive(Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
pub enum SLType {
    Void,

    Bool,
    Bool2,
    Bool3,
    Bool4,

    Short,
    Short2,
    Short3,
    Short4,

    UShort,
    UShort2,
    UShort3,
    UShort4,

    Float,
    Float2,
    Float3,
    Float4,

    Float2x2,
    Float3x3,
    Float4x4,

    Half,
    Half2,
    Half3,
    Half4,

    Half2x2,
    Half3x3,
    Half4x4,

    Int,
    Int2,
    Int3,
    Int4,

    UInt,
    UInt2,
    UInt3,
    UInt4,

    Texture2DSampler,
    TextureExternalSampler,
    Texture2DRectSampler,
    Texture2D,
    Sampler,
    Input,
}

pub const SL_TYPE_COUNT: usize = SLType::Input as usize + 1;

impl SLType {
    /// Returns the `SkSL` typename for this type.
    #[must_use]
    pub const fn to_string(self) -> &'static str {
        match self {
            Self::Void => "void",
            Self::Bool => "bool",
            Self::Bool2 => "bool2",
            Self::Bool3 => "bool3",
            Self::Bool4 => "bool4",
            Self::Short => "short",
            Self::Short2 => "short2",
            Self::Short3 => "short3",
            Self::Short4 => "short4",
            Self::UShort => "ushort",
            Self::UShort2 => "ushort2",
            Self::UShort3 => "ushort3",
            Self::UShort4 => "ushort4",
            Self::Float => "float",
            Self::Float2 => "float2",
            Self::Float3 => "float3",
            Self::Float4 => "float4",
            Self::Float2x2 => "float2x2",
            Self::Float3x3 => "float3x3",
            Self::Float4x4 => "float4x4",
            Self::Half => "half",
            Self::Half2 => "half2",
            Self::Half3 => "half3",
            Self::Half4 => "half4",
            Self::Half2x2 => "half2x2",
            Self::Half3x3 => "half3x3",
            Self::Half4x4 => "half4x4",
            Self::Int => "int",
            Self::Int2 => "int2",
            Self::Int3 => "int3",
            Self::Int4 => "int4",
            Self::UInt => "uint",
            Self::UInt2 => "uint2",
            Self::UInt3 => "uint3",
            Self::UInt4 => "uint4",
            Self::Texture2DSampler => "sampler2D",
            Self::TextureExternalSampler => "samplerExternalOES",
            Self::Texture2DRectSampler => "sampler2DRect",
            Self::Texture2D => "texture2D",
            Self::Sampler => "sampler",
            Self::Input => "subpassInput",
        }
    }

    /// Is the shading language type float (including vectors/matrices)?
    #[must_use]
    #[inline]
    pub const fn is_float_type(self) -> bool {
        match self {
            Self::Float
            | Self::Float2
            | Self::Float3
            | Self::Float4
            | Self::Float2x2
            | Self::Float3x3
            | Self::Float4x4
            | Self::Half
            | Self::Half2
            | Self::Half3
            | Self::Half4
            | Self::Half2x2
            | Self::Half3x3
            | Self::Half4x4 => true,

            Self::Void
            | Self::Texture2DSampler
            | Self::TextureExternalSampler
            | Self::Texture2DRectSampler
            | Self::Bool
            | Self::Bool2
            | Self::Bool3
            | Self::Bool4
            | Self::Short
            | Self::Short2
            | Self::Short3
            | Self::Short4
            | Self::UShort
            | Self::UShort2
            | Self::UShort3
            | Self::UShort4
            | Self::Int
            | Self::Int2
            | Self::Int3
            | Self::Int4
            | Self::UInt
            | Self::UInt2
            | Self::UInt3
            | Self::UInt4
            | Self::Texture2D
            | Self::Sampler
            | Self::Input => false,
        }
    }

    /// Is the shading language type integral (including vectors)?
    #[must_use]
    #[inline]
    pub const fn is_integral_type(self) -> bool {
        match self {
            Self::Short
            | Self::Short2
            | Self::Short3
            | Self::Short4
            | Self::UShort
            | Self::UShort2
            | Self::UShort3
            | Self::UShort4
            | Self::Int
            | Self::Int2
            | Self::Int3
            | Self::Int4
            | Self::UInt
            | Self::UInt2
            | Self::UInt3
            | Self::UInt4 => true,

            Self::Float
            | Self::Float2
            | Self::Float3
            | Self::Float4
            | Self::Float2x2
            | Self::Float3x3
            | Self::Float4x4
            | Self::Half
            | Self::Half2
            | Self::Half3
            | Self::Half4
            | Self::Half2x2
            | Self::Half3x3
            | Self::Half4x4
            | Self::Void
            | Self::Texture2DSampler
            | Self::TextureExternalSampler
            | Self::Texture2DRectSampler
            | Self::Bool
            | Self::Bool2
            | Self::Bool3
            | Self::Bool4
            | Self::Texture2D
            | Self::Sampler
            | Self::Input => false,
        }
    }

    /// If the type represents a single value or vector return the vector length; otherwise, -1.
    #[must_use]
    #[inline]
    pub const fn vec_length(self) -> i32 {
        match self {
            Self::Float
            | Self::Half
            | Self::Bool
            | Self::Short
            | Self::UShort
            | Self::Int
            | Self::UInt => 1,

            Self::Float2
            | Self::Half2
            | Self::Bool2
            | Self::Short2
            | Self::UShort2
            | Self::Int2
            | Self::UInt2 => 2,

            Self::Float3
            | Self::Half3
            | Self::Bool3
            | Self::Short3
            | Self::UShort3
            | Self::Int3
            | Self::UInt3 => 3,

            Self::Float4
            | Self::Half4
            | Self::Bool4
            | Self::Short4
            | Self::UShort4
            | Self::Int4
            | Self::UInt4 => 4,

            Self::Float2x2
            | Self::Float3x3
            | Self::Float4x4
            | Self::Half2x2
            | Self::Half3x3
            | Self::Half4x4
            | Self::Void
            | Self::Texture2DSampler
            | Self::TextureExternalSampler
            | Self::Texture2DRectSampler
            | Self::Texture2D
            | Self::Sampler
            | Self::Input => -1,
        }
    }

    /// Is the shading language type supported as a uniform (ie, does it have a corresponding set
    /// function on `GrGLSLProgramDataManager`)?
    #[must_use]
    #[inline]
    pub const fn can_be_uniform_value(self) -> bool {
        // This is almost "IsFloatType || IsIntegralType" but excludes non-full precision int types.
        match self {
            Self::Float
            | Self::Float2
            | Self::Float3
            | Self::Float4
            | Self::Float2x2
            | Self::Float3x3
            | Self::Float4x4
            | Self::Half
            | Self::Half2
            | Self::Half3
            | Self::Half4
            | Self::Half2x2
            | Self::Half3x3
            | Self::Half4x4
            | Self::Int
            | Self::Int2
            | Self::Int3
            | Self::Int4
            | Self::UInt
            | Self::UInt2
            | Self::UInt3
            | Self::UInt4 => true,
            _ => false,
        }
    }

    /// Is the shading language type full precision?
    #[must_use]
    pub const fn is_full_precision_numeric_type(self) -> bool {
        match self {
        // Half-precision types:
        Self::Short |
        Self::Short2 |
        Self::Short3 |
        Self::Short4 |
        Self::UShort |
        Self::UShort2 |
        Self::UShort3 |
        Self::UShort4 |
        Self::Half |
        Self::Half2 |
        Self::Half3 |
        Self::Half4 |
        Self::Half2x2 |
        Self::Half3x3 |
        Self::Half4x4 |
        // Non-numeric types |
        Self::Void |
        Self::Texture2DSampler |
        Self::TextureExternalSampler |
        Self::Texture2DRectSampler |
        Self::Texture2D |
        Self::Sampler |
        Self::Input |
        Self::Bool |
        Self::Bool2 |
        Self::Bool3 |
        Self::Bool4 => false,

        // Full-precision numeric types |
        Self::Int |
        Self::Int2 |
        Self::Int3 |
        Self::Int4 |
        Self::UInt |
        Self::UInt2 |
        Self::UInt3 |
        Self::UInt4 |
        Self::Float |
        Self::Float2 |
        Self::Float3 |
        Self::Float4 |
        Self::Float2x2 |
        Self::Float3x3 |
        Self::Float4x4 => true,
        }
    }

    /// If the type represents a square matrix, return its size; otherwise, -1.
    #[must_use]
    #[inline]
    pub const fn matrix_size(self) -> i32 {
        match self {
            Self::Float2x2 | Self::Half2x2 => 2,

            Self::Float3x3 | Self::Half3x3 => 3,

            Self::Float4x4 | Self::Half4x4 => 4,

            Self::Float
            | Self::Half
            | Self::Bool
            | Self::Short
            | Self::UShort
            | Self::Int
            | Self::UInt
            | Self::Float2
            | Self::Half2
            | Self::Bool2
            | Self::Short2
            | Self::UShort2
            | Self::Int2
            | Self::UInt2
            | Self::Float3
            | Self::Half3
            | Self::Bool3
            | Self::Short3
            | Self::UShort3
            | Self::Int3
            | Self::UInt3
            | Self::Float4
            | Self::Half4
            | Self::Bool4
            | Self::Short4
            | Self::UShort4
            | Self::Int4
            | Self::UInt4
            | Self::Void
            | Self::Texture2DSampler
            | Self::TextureExternalSampler
            | Self::Texture2DRectSampler
            | Self::Texture2D
            | Self::Sampler
            | Self::Input => -1,
        }
    }

    /// If the type represents a square matrix, return its size; otherwise, -1.
    #[must_use]
    #[inline]
    pub const fn is_combined_sampler_type(self) -> bool {
        match self {
            Self::Texture2DRectSampler | Self::Texture2DSampler | Self::TextureExternalSampler => {
                true
            }

            Self::Void
            | Self::Float
            | Self::Float2
            | Self::Float3
            | Self::Float4
            | Self::Float2x2
            | Self::Float3x3
            | Self::Float4x4
            | Self::Half
            | Self::Half2
            | Self::Half3
            | Self::Half4
            | Self::Half2x2
            | Self::Half3x3
            | Self::Half4x4
            | Self::Int
            | Self::Int2
            | Self::Int3
            | Self::Int4
            | Self::UInt
            | Self::UInt2
            | Self::UInt3
            | Self::UInt4
            | Self::Bool
            | Self::Bool2
            | Self::Bool3
            | Self::Bool4
            | Self::Short
            | Self::Short2
            | Self::Short3
            | Self::Short4
            | Self::UShort
            | Self::UShort2
            | Self::UShort3
            | Self::UShort4
            | Self::Texture2D
            | Self::Sampler
            | Self::Input => false,
        }
    }
}
