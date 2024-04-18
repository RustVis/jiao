// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that can be found
// in the LICENSE file.

#![allow(clippy::module_name_repetitions)]
#![allow(clippy::verbose_bit_mask)]

pub trait Align: Copy {
    #[must_use]
    fn align2(self) -> Self;

    #[must_use]
    fn align4(self) -> Self;

    #[must_use]
    fn align8(self) -> Self;

    fn is_align2(self) -> bool;

    fn is_align4(self) -> bool;

    fn is_align8(self) -> bool;

    #[must_use]
    fn align_ptr(self) -> Self {
        if cfg!(target_pointer_width = "64") {
            self.align8()
        } else {
            self.align4()
        }
    }

    fn is_align_ptr(self) -> bool {
        if cfg!(target_pointer_width = "64") {
            self.is_align8()
        } else {
            self.is_align4()
        }
    }
}

macro_rules! align_impl {
    ($($t:ty)*) => {$(
        impl Align for $t {
            fn align2(self) -> Self {
                (self + 1) >> 1 << 1
            }

            fn align4(self) -> Self {
                (self + 3) >> 2 << 2
            }

            fn align8(self) -> Self {
                (self + 7) >> 3 << 3
            }

            fn is_align2(self) -> bool {
                (self & 1) == 0
            }

            fn is_align4(self) -> bool {
                (self & 3) == 0
            }

            fn is_align8(self) -> bool {
                (self & 7) == 0
            }
        }
    )*}
}
align_impl! { isize i32 i64 usize u32 u64 }

/// align up to a power of 2
#[must_use]
#[inline]
pub fn align_to(x: usize, alignment: usize) -> usize {
    // The same as alignment && is_pow2(value), w/o a dependency cycle.
    debug_assert!(alignment != 0 && (alignment & (alignment - 1)) == 0);
    (x + alignment - 1) & !(alignment - 1)
}
