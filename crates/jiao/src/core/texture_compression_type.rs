// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by LGPL-3.0 License that can be found
// in the LICENSE file.

/// ```txt
///  Jiao               | GL_COMPRESSED_*     | MTLPixelFormat*      | VK_FORMAT_*_BLOCK
/// --------------------------------------------------------------------------------------
///  ETC2_RGB8_UNORM    | ETC1_RGB8           | ETC2_RGB8 (iOS-only) | ETC2_R8G8B8_UNORM
///                     | RGB8_ETC2           |                      |
/// --------------------------------------------------------------------------------------
///  BC1_RGB8_UNORM     | RGB_S3TC_DXT1_EXT   | N/A                  | BC1_RGB_UNORM
/// --------------------------------------------------------------------------------------
///  BC1_RGBA8_UNORM    | RGBA_S3TC_DXT1_EXT  | BC1_RGBA (macOS-only)| BC1_RGBA_UNORM
/// ```
#[repr(u8)]
pub enum TextureCompressionType {
    None = 0,
    Etc2Rgb8Unorm,
    Bc1Rgb8Unorm,
    Bc1Rgba8Unorm,
}

pub const ETC1_RGB8: TextureCompressionType = TextureCompressionType::Etc2Rgb8Unorm;
