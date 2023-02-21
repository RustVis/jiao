// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

#![allow(clippy::module_name_repetitions)]

use serde::{Deserialize, Serialize};

/// Icon of mouse cursor.
///
/// See <https://developer.mozilla.org/en-US/docs/Web/CSS/cursor>
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CursorIcon {
    /// The platform-dependent default cursor.
    ///
    /// Typically an arrow.
    Default,

    /// No cursor is rendered.
    None,

    /// A context menu is available.
    ContextMneu,

    /// Help information is available.
    Help,

    /// The cursor is a pointer that indicates a link.
    ///
    /// Typically an image of a pointing hand.
    Pointer,

    /// wide arrow and hour glass.
    ///
    /// The program is busy in the background, but the user can still interact with
    /// the interface (in contrast to wait).
    Progress,

    /// The program is busy, and the user can't interact with the interface (in contrast to progress).
    ///
    /// Sometimes an image of an hourglass or a watch.
    Wait,

    /// The table cell or set of cells can be selected.
    Cell,

    /// Cross cursor, often used to indicate selection in a bitmap.
    CrossHair,

    /// The text can be selected.
    ///
    /// Typically the shape of an I-beam.
    Text,

    /// The vertical text can be selected.
    ///
    /// Typically the shape of a sideways I-beam.
    VerticalText,

    /// An alias or shortcut is to be created.
    Alias,

    /// Something is to be copied.
    Copy,

    /// Something is to be moved.
    Move,

    /// An item may not be dropped at the current location.
    NoDrop,

    /// The requested action will not be carried out.
    NotAllowed,

    /// Something can be grabbed (dragged to be moved).
    Grab,

    /// Something is being grabbed (dragged to be moved).
    Grabbing,

    /// Something can be scrolled in any direction (panned).
    AllScroll,

    /// The item/column can be resized horizontally.
    ///
    /// Often rendered as arrows pointing left and right with a vertical bar separating them.
    ColResize,

    /// The item/row can be resized vertically.
    ///
    /// Often rendered as arrows pointing up and down with a horizontal bar separating them.
    RowResize,

    /// Some edge is to be moved.
    ///
    /// For example, the se-resize cursor is used when the movement starts from
    /// the south-east corner of the box.
    /// In some environments, an equivalent bidirectional resize cursor is shown.
    /// For example, n-resize and s-resize are the same as ns-resize.
    NResize,
    EResize,
    SResize,
    WResize,
    NEResize,
    NWResize,
    SEResize,
    SWResize,

    /// Bidirectional resize cursor.
    EWResize,
    NSResize,
    NESWResize,
    NWSEResize,

    /// Something can be zoomed (magnified) in or out.
    ZoomIn,
    ZoomOut,
}

impl Default for CursorIcon {
    fn default() -> Self {
        Self::Default
    }
}

impl CursorIcon {
    #[allow(dead_code)]
    pub const ALL: [Self; 35] = [
        Self::Default,
        Self::None,
        Self::ContextMneu,
        Self::Help,
        Self::Pointer,
        Self::Progress,
        Self::Wait,
        Self::Cell,
        Self::CrossHair,
        Self::Text,
        Self::VerticalText,
        Self::Alias,
        Self::Copy,
        Self::Move,
        Self::NoDrop,
        Self::NotAllowed,
        Self::Grab,
        Self::Grabbing,
        Self::AllScroll,
        Self::ColResize,
        Self::RowResize,
        Self::NResize,
        Self::EResize,
        Self::SResize,
        Self::WResize,
        Self::NEResize,
        Self::NWResize,
        Self::SEResize,
        Self::SWResize,
        Self::EWResize,
        Self::NSResize,
        Self::NESWResize,
        Self::NWSEResize,
        Self::ZoomIn,
        Self::ZoomOut,
    ];
}

#[cfg(test)]
mod tests {
    use super::CursorIcon;

    #[test]
    fn test_print_all_icons() {
        for icon in CursorIcon::ALL {
            println!("icon: {icon:?}");
        }
    }
}
