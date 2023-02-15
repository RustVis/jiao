// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

#[allow(clippy::module_name_repetitions)]
pub trait PaintContextTrait {
    /// Repaint immediately.
    fn repaint(&mut self);

    /// Schedule a repaint operation.
    fn update(&mut self);
}
