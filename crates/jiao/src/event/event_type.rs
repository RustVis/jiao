// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

#![allow(clippy::module_name_repetitions)]

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EventType {
    None = 0,

    MouseButtonPress = 2,
    MouseButtonRelease = 3,
    MouseButtonDoubleClick = 4,
    MouseMove = 5,
    KeyPress = 6,
    KeyRelease = 7,
    FocusIn = 8,
    FocusOut = 9,
    Enter = 10,
    Leave = 11,
    /// Mouse wheel rolled
    Wheel = 31,

    /// Beginning of a sequence of touch-screen or track-pad events
    TouchBegin = 194,

    /// Touch-screen event
    TouchUpdate = 195,

    /// End of touch-event sequence
    TouchEnd = 196,

    /// Cancellation of touch-event sequence
    TouchCancel = 209,
}
