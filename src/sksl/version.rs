// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that can be found
// in the LICENSE file.

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum Version {
    /// Desktop GLSL 1.10, GLSL ES 1.00, WebGL 1.0
    V100,

    /// Desktop GLSL 3.30, GLSL ES 3.00, WebGL 2.0
    V300,
}
