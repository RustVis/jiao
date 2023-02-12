// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

use jiao::base::{PointF, RectF, SizeF};
use jiao::kernel::ShapeManager;
use jiao::shapes::{
    CircleShape, DonutShape, DropletShape, GridShape, LineShape, PolygonShape, RectShape,
    RoundRectShape,
};

pub const CANVAS_WIDTH: i32 = 800;
pub const CANVAS_HEIGHT: i32 = 600;

pub fn paint_shapes(shape_manager: &mut ShapeManager) {
    let lines = [
        (10.0, 10.0, 50.0, 10.0),
        (10.0, 10.0, 10.0, 50.0),
        (10.0, 50.0, 50.0, 50.0),
        (50.0, 10.0, 50.0, 50.0),
        (30.0, 10.0, 30.0, 50.0),
        (10.0, 30.0, 50.0, 30.0),
    ];
    // Line 1
    for p in lines.iter() {
        let line = LineShape::from_f64(p.0, p.1, p.2, p.3);
        shape_manager.add(Box::new(line));
    }

    let circle = CircleShape::new(PointF::from(100.0, 30.0), 20.0);
    shape_manager.add(Box::new(circle));
    let rect = RectF::from_circle(PointF::from(100.0, 30.0), 20.0);
    shape_manager.add(Box::new(RectShape::from_rect(rect)));

    let rect = RectF::from(150.0, 15.0, 50.0, 30.0);
    let round_rect = RoundRectShape::new(rect.clone(), 15.0);
    shape_manager.add(Box::new(round_rect));
    shape_manager.add(Box::new(RectShape::from_rect(rect)));

    // Line 2
    let grid = GridShape::with_viewport(RectF::from(0.0, 80.0, 50.0, 50.0), 5.0, 5.0);
    shape_manager.add(Box::new(grid));

    let droplet = DropletShape::new(PointF::from(100.0, 100.0), SizeF::from(20.0, 20.0));
    shape_manager.add(Box::new(droplet));

    let points = [
        (170.0, 80.0),
        (150.0, 100.0),
        (170.0, 120.0),
        (190.0, 100.0),
        (170.0, 80.0),
        (160.0, 100.0),
        (170.0, 120.0),
        (180.0, 100.0),
        (170.0, 80.0),
        (150.0, 100.0),
        (190.0, 100.0),
    ];
    let points: Vec<PointF> = points.iter().map(|(x, y)| PointF::from(*x, *y)).collect();
    let polyline = PolygonShape::from_points(&points, false);
    shape_manager.add(Box::new(polyline));

    // Line 3
    let donut = DonutShape::new(PointF::from(160.0, 260.0), 120.0);
    shape_manager.add(Box::new(donut));
}
