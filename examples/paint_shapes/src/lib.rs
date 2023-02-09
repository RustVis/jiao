// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

use jiao::base::{PointF, RectF};
use jiao::kernel::ShapeManager;
use jiao::shapes::{CircleShape, GridShape, LineShape, RoundedRectShape};

pub const CANVAS_WIDTH: i32 = 800;
pub const CANVAS_HEIGHT: i32 = 600;

pub fn paint_shapes(shape_manager: &mut ShapeManager) {
    for p in [
        (10.0, 10.0, 50.0, 10.0),
        (10.0, 10.0, 10.0, 50.0),
        (10.0, 50.0, 50.0, 50.0),
        (50.0, 10.0, 50.0, 50.0),
        (30.0, 10.0, 30.0, 50.0),
        (10.0, 30.0, 50.0, 30.0),
    ]
    .iter()
    {
        let line = LineShape::from_f64(p.0, p.1, p.2, p.3);
        shape_manager.add(Box::new(line));
    }

    let circle = CircleShape::new(PointF::from(75.0, 50.0), 15.0);
    shape_manager.add(Box::new(circle));

    let rect = RoundedRectShape::new(RectF::from(110.0, 60.0, 50.0, 20.0), 5.0);
    shape_manager.add(Box::new(rect));

    let grid = GridShape::new(5.0, 5.0);
    shape_manager.add(Box::new(grid));
}
