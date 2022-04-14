// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

use jiao::kernel::PaintContextTrait;
use jiao::platforms::PaintContext;
use jiao::shapes::LineShape;
use qt_core::QString;
use qt_gui::q_image::Format;
use qt_gui::{QGuiApplication, QImage};

fn draw_png() {
    let image = unsafe { QImage::from_2_int_format(300, 150, Format::FormatRGBA8888) };
    let mut paint_ctx = PaintContext::new();
    let shape_manager = paint_ctx.shape_manager();
    let line = LineShape::from_f64(0.0, 0.0, 50.0, 50.0);
    shape_manager.add(Box::new(line));
    paint_ctx.start(&image);

    let ok = unsafe { image.save_q_string(&QString::from_std_str("out.png")) };
    assert!(ok);
}

fn main() {
    QGuiApplication::init(|_app| unsafe {
        println!("Init qt gui");
        draw_png();
        QGuiApplication::exec()
    });
}
