// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that can be found
// in the LICENSE file.

//! Macros and helper functions for handling 16 and 32 bit values in
//! big and little endian formats.

// When a bytestream is embedded in a 32-bit word, how far we need to
// shift the word to extract each byte from the low 8 bits by anding with 0xff.
#[cfg(target_endian = "little")]
mod inner {
    /*
    #define SkEndian_SwapBE16(n)    SkEndianSwap16(n)
    #define SkEndian_SwapBE32(n)    SkEndianSwap32(n)
    #define SkEndian_SwapBE64(n)    SkEndianSwap64(n)
    #define SkEndian_SwapLE16(n)    static_cast<uint16_t>(n)
    #define SkEndian_SwapLE32(n)    static_cast<uint32_t>(n)
    #define SkEndian_SwapLE64(n)    static_cast<uint64_t>(n)

    #define SkTEndian_SwapBE16(n)    SkTEndianSwap16<n>::value
    #define SkTEndian_SwapBE32(n)    SkTEndianSwap32<n>::value
    #define SkTEndian_SwapBE64(n)    SkTEndianSwap64<n>::value
    #define SkTEndian_SwapLE16(n)    (n)
    #define SkTEndian_SwapLE32(n)    (n)
    #define SkTEndian_SwapLE64(n)    (n)
    */

    pub const BYTE0_SHIFT: u8 = 0;
    pub const BYTE1_SHIFT: u8 = 8;
    pub const BYTE2_SHIFT: u8 = 16;
    pub const BYTE3_SHIFT: u8 = 24;
}

#[cfg(target_endian = "big")]
mod inner {
    /*
    #define SkEndian_SwapBE16(n)    static_cast<uint16_t>(n)
    #define SkEndian_SwapBE32(n)    static_cast<uint32_t>(n)
    #define SkEndian_SwapBE64(n)    static_cast<uint64_t>(n)
    #define SkEndian_SwapLE16(n)    SkEndianSwap16(n)
    #define SkEndian_SwapLE32(n)    SkEndianSwap32(n)
    #define SkEndian_SwapLE64(n)    SkEndianSwap64(n)

    #define SkTEndian_SwapBE16(n)    (n)
    #define SkTEndian_SwapBE32(n)    (n)
    #define SkTEndian_SwapBE64(n)    (n)
    #define SkTEndian_SwapLE16(n)    SkTEndianSwap16<n>::value
    #define SkTEndian_SwapLE32(n)    SkTEndianSwap32<n>::value
    #define SkTEndian_SwapLE64(n)    SkTEndianSwap64<n>::value
    */

    pub const BYTE0_SHIFT: u8 = 24;
    pub const BYTE1_SHIFT: u8 = 16;
    pub const BYTE2_SHIFT: u8 = 8;
    pub const BYTE3_SHIFT: u8 = 0;
}

#[allow(unused_imports)]
pub use inner::*;
