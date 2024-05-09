// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that can be found
// in the LICENSE file.

/// Return the integer square root of value, with a bias of `bit_bias`
///
/// See [`FixedSqrt`](www.worldserver.com/turk/computergraphics/FixedSqrt.pdf)
#[must_use]
#[allow(clippy::cast_possible_wrap)]
#[allow(clippy::cast_sign_loss)]
pub fn sqrt_bits(value: i32, mut bit_bias: i32) -> i32 {
    debug_assert!(value >= 0 && bit_bias > 0 && bit_bias <= 30);

    let mut root: u32 = 0;
    let mut rem_hi: u32 = 0;
    let mut rem_lo: u32 = value as u32;

    loop {
        root <<= 1;

        rem_hi = (rem_hi << 2) | (rem_lo >> 30);
        rem_lo <<= 2;

        let test_div: u32 = (root << 1) + 1;
        if rem_hi >= test_div {
            rem_hi -= test_div;
            root += 1;
        }

        bit_bias -= 1;
        if bit_bias < 0 {
            break;
        }
    }

    root as i32
}

/// Return the integer square root of n, treated as a Fixed (16.16)
#[must_use]
#[inline]
pub fn sqrt32(n: i32) -> i32 {
    sqrt_bits(n, 15)
}

/// Returns (value < 0 ? 0 : value) efficiently (i.e. no compares or branches)
pub const fn clamp_pos(value: i32) -> i32 {
    value & !(value >> 31)
}

/// Stores numer/denom and numer%denom into div and mod respectively.
#[inline]
pub fn div_mod(numer: i32, denom: i32, div: &mut i32, modulo: &mut i32) {
    *div = numer / denom;
    *modulo = numer % denom;
}

/// Returns -1 if n < 0, else returns 0
#[must_use]
#[inline]
pub const fn extract_sign(n: i32) -> i32 {
    n >> 31
}

/// If sign == -1, returns -n, else sign must be 0, and returns n.
///
/// Typically used in conjunction with `extract_sign()`.
#[must_use]
#[inline]
pub const fn apply_sign(n: i32, sign: i32) -> i32 {
    debug_assert!(sign == 0 || sign == -1);
    (n ^ sign) - sign
}

/// Return x with the sign of y
#[must_use]
#[inline]
pub const fn copy_sign32(x: i32, y: i32) -> i32 {
    apply_sign(x, extract_sign(x ^ y))
}

/// Given a positive value and a positive max, return the value pinned against max.
///
/// Note: only works as long as max - value doesn't wrap around
/// @return max if value >= max, else value
#[must_use]
#[inline]
pub const fn clamp_umax(value: u32, max: u32) -> u32 {
    if value > max {
        max
    } else {
        value
    }
}

/// If a signed int holds `min_int` (e.g. 0x80000000) it is undefined what happens when
/// we negate it (even though we *know* we're 2's complement and we'll get the same
/// value back).
///
/// So we create this helper function that casts to `size_t` (unsigned) first,
/// to avoid the complaint.
#[must_use]
#[inline]
#[allow(clippy::cast_sign_loss)]
pub const fn negate_to_usize(value: i32) -> usize {
    (-(value as isize)) as usize
}

/// Return a*b/255, truncating away any fractional bits.
///
/// Only valid if both a and b are 0..255
#[must_use]
#[inline]
#[allow(clippy::cast_possible_truncation)]
pub const fn mul_div_255_trunc(a: u8, b: u8) -> u8 {
    let prod: u32 = (a as u32) * (b as u32) + 1;
    ((prod + (prod >> 8)) >> 8) as u8
}

/// Return (a*b)/255, taking the ceiling of any fractional bits.
///
/// Only valid if both a and b are 0..255.
/// The expected result equals (a * b + 254) / 255.
#[must_use]
#[inline]
#[allow(clippy::cast_possible_truncation)]
pub const fn mul_div_255_ceiling(a: u8, b: u8) -> u8 {
    let prod: u32 = (a as u32) * (b as u32) + 255;
    ((prod + (prod >> 8)) >> 8) as u8
}

/// Just the rounding step in `div_255_round`: round(value / 255)
#[must_use]
#[inline]
pub const fn div_255_round(mut prod: u32) -> u32 {
    prod += 128;
    (prod + (prod >> 8)) >> 8
}

/// Swap byte order of a 4-byte value, e.g. 0xaarrggbb -> 0xbbggrraa.
#[must_use]
#[inline]
pub const fn b_swap32(v: u32) -> u32 {
    v.swap_bytes()
}

// Return the number of set bits (i.e., the population count) in the provided uint32_t.
//
// Replaced with `u32::count_ones()`.
// pub const fn pop_count(n: u32) -> u32;

// Returns the number of leading zero bits (0...32).
//
// replaced with `u32::leading_zeros()`.
// pub const fn clz(x: u32) -> i32;

// Returns the number of trailing zero bits (0...32).
//
// replaced with `u32::trailing_zeros()`.
// pub const fn ctz(x: u32) -> i32;

/// Return the 0-based index of the nth bit set in target.
///
/// Returns 32 if there is no nth bit set.
#[must_use]
pub const fn nth_set(mut target: u32, n: u32) -> u32 {
    // Here we strip off the unwanted bits and then return the number of trailing zero bits
    debug_assert!(n < target.count_ones());

    let mut i: u32 = 0;
    while i < n {
        // Remove the lowest bit in the integer.
        target &= target - 1;
        i += 1;
    }

    target.trailing_zeros()
}

/// Returns the log2 of the specified value, were that value to be rounded up
/// to the next power of 2.
///
/// It is undefined to pass 0. Examples:
///   `next_log2(1`) -> 0
///   `next_log2(2`) -> 1
///   `next_log2(3`) -> 2
///   `next_log2(4`) -> 2
///   `next_log2(5`) -> 3
#[must_use]
#[inline]
#[allow(clippy::cast_possible_wrap)]
pub const fn next_log2(value: u32) -> i32 {
    debug_assert!(value != 0);
    32 - (value - 1).leading_zeros() as i32
}

/// Returns the log2 of the specified value, were that value to be rounded down
/// to the previous power of 2.
///
/// It is undefined to pass 0. Examples:
///   `prev_log2(1`) -> 0
///   `prev_log2(2`) -> 1
///   `prev_log2(3`) -> 1
///   `prev_log2(4`) -> 2
///   `prev_log2(5`) -> 2
#[must_use]
#[inline]
#[allow(clippy::cast_possible_wrap)]
pub const fn prev_log2(value: u32) -> i32 {
    debug_assert!(value != 0);
    32 - (value >> 1).leading_zeros() as i32
}

/// Returns the smallest power-of-2 that is >= the specified value.
///
/// If value is already a power of 2, then it is returned unchanged.
/// It is undefined if value is <= 0.
#[must_use]
#[inline]
#[allow(clippy::cast_sign_loss)]
pub const fn next_pow2(value: i32) -> i32 {
    debug_assert!(value > 0);
    1 << next_log2(value as u32)
}

/// Returns the largest power-of-2 that is <= the specified value.
///
/// If value is already a power of 2, then it is returned unchanged.
/// It is undefined if value is <= 0.
#[must_use]
#[inline]
#[allow(clippy::cast_sign_loss)]
pub const fn prev_pow2(value: i32) -> i32 {
    debug_assert!(value > 0);
    1 << prev_log2(value as u32)
}

/// Return the smallest power-of-2 >= n.
#[must_use]
#[inline]
pub const fn gr_next_pow2(n: u32) -> u32 {
    if n == 0 {
        1 << (32 - (n - 1).leading_zeros())
    } else {
        1
    }
}

/// Returns the next power of 2 >= n or n if the next power of 2 can't be represented by `size_t`.
#[must_use]
#[inline]
pub const fn gr_next_size_pow2(mut n: usize) -> usize {
    const NUM_SIZE_BITS: usize = usize::BITS as usize;
    const HIGH_BITSET: usize = 1usize << (NUM_SIZE_BITS - 1);

    if n == 0 {
        return 1;
    } else if n >= HIGH_BITSET {
        return n;
    }

    n -= 1;
    let mut shift: usize = 1;
    while shift < NUM_SIZE_BITS {
        n |= n >> shift;
        shift <<= 1;
    }
    n + 1
}

/// conservative check.
///
/// will return false for very large values that "could" fit
#[must_use]
#[inline]
pub fn fits_in_fixed(x: f32) -> bool {
    x.abs() <= 32767.0_f32
}
