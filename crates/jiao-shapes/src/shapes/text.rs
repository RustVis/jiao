// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that can be found
// in the LICENSE file.

use jiao::base::RectF;
use jiao::kernel::PainterTrait;

use crate::ShapeTrait;

#[derive(Debug, Clone)]
pub struct TextShape {
    text: String,
    container_rect: RectF,
    path_is_dirty: bool,
}

impl TextShape {
    #[must_use]
    pub const fn new(text: String, container_rect: RectF) -> Self {
        Self {
            text,
            container_rect,
            path_is_dirty: true,
        }
    }

    /// Get text content.
    #[must_use]
    pub fn text(&self) -> &str {
        &self.text
    }

    /// Update text content.
    pub fn set_text(&mut self, text: String) {
        if self.text != text {
            self.text = text;
            self.path_is_dirty = true;
        }
    }

    /// Get container rectangle.
    #[must_use]
    pub const fn container_rect(&self) -> &RectF {
        &self.container_rect
    }

    /// Update bounding rectangle.
    pub fn set_container_rect(&mut self, container_rect: RectF) {
        if self.container_rect != container_rect {
            self.container_rect = container_rect;
            self.path_is_dirty = true;
        }
    }
}

impl ShapeTrait for TextShape {
    fn bounding_rect(&self) -> RectF {
        todo!()
    }

    fn repaint(&mut self, _painter: &mut dyn PainterTrait) {}
}
