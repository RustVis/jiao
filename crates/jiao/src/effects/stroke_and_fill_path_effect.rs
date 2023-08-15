// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

use crate::core::path::Path;
use crate::core::scalar::Scalar;
use crate::effects::d1_path_effect::{D1PathEffect, Style};

/// If the paint is set to stroke, this will add the stroke and fill geometries
/// together (hoping that the winding-direction works out).
///
/// If the paint is set to fill, this effect is ignored.
///
/// Note that if the paint is set to stroke and the stroke-width is 0, then
/// this will turn the geometry into just a fill.
#[must_use]
pub fn make(_path: &Path, _advance: Scalar, _phase: Scalar, _style: Style) -> Box<D1PathEffect> {
    unimplemented!()
}
