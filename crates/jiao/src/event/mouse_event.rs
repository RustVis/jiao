// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

#![allow(clippy::struct_excessive_bools)]

/// The `MouseEvent` interface represents events that occur due to the user
/// interacting with a pointing device (such as a mouse).
///
/// Common events using this interface include click, dblclick, mouseup, mousedown.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MouseEvent {
    client_x: i32,
    client_y: i32,
    page_x: i32,
    page_y: i32,
    screen_x: i32,
    screen_y: i32,

    /// Indicates which button was pressed on the mouse to trigger the event.
    button: Button,

    alt_key: bool,
    ctrl_key: bool,
    meta_key: bool,
    shift_key: bool,
}

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Button {
    /// The left button is pressed, or an event refers to the left button.
    ///
    /// The left button may be the right button on left-handed mice.
    Left = 0x0000_0000,

    /// The middle button.
    Middle = 0x0000_0001,

    /// The right button.
    Right = 0x0000_0002,

    /// The browse-back button.
    Back = 0x0000_0003,

    /// The browse-forward button.
    Forward = 0x0000_0004,
}

impl MouseEvent {
    /// Returns true if the alt key was down when the mouse event was fired.
    #[inline]
    #[must_use]
    pub const fn alt_key(&self) -> bool {
        self.alt_key
    }

    /// Indicates whether the ctrl key was pressed or not when a given mouse event occurs.
    #[inline]
    #[must_use]
    pub const fn ctrl_key(&self) -> bool {
        self.ctrl_key
    }

    /// Indicates whether the meta key was pressed or not when a given mouse event occurs.
    #[inline]
    #[must_use]
    pub const fn meta_key(&self) -> bool {
        self.meta_key
    }

    /// Indicates whether the shift key was pressed or not when a given mouse event occurs.
    #[inline]
    #[must_use]
    pub const fn shift_key(&self) -> bool {
        self.shift_key
    }

    /// The button number that was pressed (if applicable) when the mouse event was fired.
    #[inline]
    #[must_use]
    pub const fn button(&self) -> Button {
        self.button
    }

    /// The X coordinate of the mouse pointer in local (DOM content or widget) coordinates.
    #[inline]
    #[must_use]
    pub const fn client_x(&self) -> i32 {
        self.client_x
    }

    /// The X coordinate of the mouse pointer in local (DOM content or widget) coordinates.
    #[inline]
    #[must_use]
    pub const fn x(&self) -> i32 {
        self.client_x
    }

    /// The Y coordinate of the mouse pointer in local (DOM content or widget) coordinates.
    #[inline]
    #[must_use]
    pub const fn client_y(&self) -> i32 {
        self.client_y
    }

    /// The Y coordinate of the mouse pointer in local (DOM content or widget) coordinates.
    #[inline]
    #[must_use]
    pub const fn y(&self) -> i32 {
        self.client_y
    }

    /// The X (horizontal) coordinate at which the mouse was clicked,
    /// relative to the left edge of the entire document or top window.
    #[inline]
    #[must_use]
    pub const fn page_x(&self) -> i32 {
        self.page_x
    }

    /// The Y (vertical) coordinate in pixels of the event relative to
    /// the whole document or top window.
    #[inline]
    #[must_use]
    pub const fn page_y(&self) -> i32 {
        self.page_y
    }

    /// The horizontal coordinate (offset) of the mouse pointer in global (screen) coordinates.
    #[inline]
    #[must_use]
    pub const fn screen_x(&self) -> i32 {
        self.screen_x
    }

    /// The vertical coordinate (offset) of the mouse pointer in global (screen) coordinates.
    #[inline]
    #[must_use]
    pub const fn screen_y(&self) -> i32 {
        self.screen_y
    }
}
