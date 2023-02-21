// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

#![allow(clippy::module_name_repetitions)]

mod event_type;
mod keyboard_event;
pub mod mouse_event;
mod touch_event;
mod wheel_event;

pub use event_type::EventType;
pub use keyboard_event::KeyboardEvent;
pub use mouse_event::{Button, MouseEvent};
pub use touch_event::TouchEvent;
pub use wheel_event::WheelEvent;
