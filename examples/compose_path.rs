// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that can be found
// in the LICENSE file.

//! Based on https://fiddle.skia.org/c/@compose_path

use jiao::core::canvas::Canvas;
use jiao::core::color::colors;
use jiao::core::paint::Paint;
use jiao::core::paint_types::PaintStyle;
use jiao::core::path::Path;
use jiao::core::path_builder::PathBuilder;
use jiao::core::path_effect::PathEffect;
use jiao::core::scalar::Scalar;

fn star() -> Path {
    const RADIUS: Scalar = 115.2;
    const CENTER: Scalar = 128.0;

    let mut pb = PathBuilder::new();
    pb.move_to(CENTER + RADIUS, CENTER);
    for i in 1..8 {
        let a: Scalar = 2.6927937 * i as Scalar;
        pb.line_to(CENTER + RADIUS * a.cos(), CENTER + RADIUS * a.sin());
    }

    pb.finish().unwrap()
}

fn main() {
    let canvas = Canvas::new();

    let mut paint = Paint::new();
    let intervals = [10.0, 5.0, 2.0, 5.0];
    //    paint.set_path_effect(PathEffect::compose(
    //        DashPathEffect::new(&intervals, 0.0),
    //        DiscretePathEffect::new(10.0, 4.0),
    //    ));
    paint.set_style(PaintStyle::Stroke);
    paint.set_stroke_width(2.0);
    paint.set_anti_alias(true);
    paint.set_color(0xff4285F4.into());

    canvas.clear(colors::WHITE);
    let path: Path = star();
    canvas.draw_path(path, paint);
}
