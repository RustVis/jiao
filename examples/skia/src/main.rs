// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

use jiao::kernel::{PaintContextTrait, ShapeManager};
use jiao::platforms::skia::{ImagePaintDevice, PaintContext, PaintDevice};
use jiao::shapes::LineShape;
use skia_safe::EncodedImageFormat;
use std::fs::File;
use std::io::Write;

fn do_paint(shape_manager: &mut ShapeManager) {
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
}

fn main() {
    let paint_device = PaintDevice::Image(ImagePaintDevice::new(300, 150));
    let mut paint_ctx = PaintContext::new(paint_device);
    let mut shape_manager = paint_ctx.shape_manager();
    do_paint(&mut shape_manager);
    paint_ctx.update();

    let paint_device = paint_ctx.paint_device();
    if let PaintDevice::Image(img_paint_device) = paint_device {
        // Save to png file
        {
            let data = img_paint_device.encode(EncodedImageFormat::PNG);
            let mut file = File::create("out.png").unwrap();
            let bytes = data.as_bytes();
            file.write_all(bytes).unwrap();
        }

        // Save to jpg file
        {
            let data = img_paint_device.encode(EncodedImageFormat::JPEG);
            let mut file = File::create("out.jpg").unwrap();
            let bytes = data.as_bytes();
            file.write_all(bytes).unwrap();
        }
    }
}
