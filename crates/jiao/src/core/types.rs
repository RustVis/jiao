// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

pub const G32_SHIFT: usize = 8;
pub const A32_SHIFT: usize = 24;

// NOTE(Shaohua): Use byte order cfg instead.
// SK_PMCOLOR_BYTE_ORDER can be used to query the byte order of SkPMColor at compile time.
//#ifdef SK_CPU_BENDIAN
//#  define SK_PMCOLOR_BYTE_ORDER(C0, C1, C2, C3)     \
//        (SK_ ## C3 ## 32_SHIFT == 0  &&             \
//         SK_ ## C2 ## 32_SHIFT == 8  &&             \
//         SK_ ## C1 ## 32_SHIFT == 16 &&             \
//         SK_ ## C0 ## 32_SHIFT == 24)
//#else
//#  define SK_PMCOLOR_BYTE_ORDER(C0, C1, C2, C3)     \
//        (SK_ ## C0 ## 32_SHIFT == 0  &&             \
//         SK_ ## C1 ## 32_SHIFT == 8  &&             \
//         SK_ ## C2 ## 32_SHIFT == 16 &&             \
//         SK_ ## C3 ## 32_SHIFT == 24)
//#endif

/// 32 bit integer to hold a unicode value
pub type Unichar = i32;

/// 16 bit unsigned integer to hold a glyph index
pub type GlyphId = u16;

/// 32 bit value to hold a millisecond duration.
///
/// Note that `MSecMax` is about 25 days.
pub type MilliSec = u32;

/// Maximum representable milliseconds; 24d 20h 31m 23.647s.
pub const MILLI_SEC_MAX: MilliSec = u32::MAX;

/// The generation IDs reserve 0 has an invalid marker.
pub const INVALID_GEN_ID: u32 = 0;

/// The unique IDs reserve 0 has an invalid marker.
pub const INVALID_UNIQUE_ID: u32 = 0;
