// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that can be found
// in the LICENSE file.

/// ```txt
///  Jiao               | GL_COMPRESSED_*     | MTLPixelFormat*      | VK_FORMAT_*_BLOCK
/// --------------------------------------------------------------------------------------
///  Etc2Rgb8Unorm    | ETC1_RGB8           | ETC2_RGB8 (iOS-only) | ETC2_R8G8B8_UNORM
///                     | RGB8_ETC2           |                      |
/// --------------------------------------------------------------------------------------
///  Bc1RGB8Unorm     | RGB_S3TC_DXT1_EXT   | N/A                  | BC1_RGB_UNORM
/// --------------------------------------------------------------------------------------
///  Bc1Rgba8Unorm    | RGBA_S3TC_DXT1_EXT  | BC1_RGBA (macOS-only)| BC1_RGBA_UNORM
/// ```
#[repr(u8)]
#[derive(Debug, Default, Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum TextureCompressionType {
    #[default]
    None = 0,
    Etc2Rgb8Unorm,
    Bc1Rgb8Unorm,
    Bc1Rgba8Unorm,
}

pub const ETC1_RGB8: TextureCompressionType = TextureCompressionType::Etc2Rgb8Unorm;
