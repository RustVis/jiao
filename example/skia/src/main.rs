// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

use jiao::kernel::PaintContextTrait;
use jiao::platforms::PaintContext;
use jiao::shapes::LineShape;
use skia_safe::EncodedImageFormat;
use std::fs::File;
use std::io::Write;

fn main() {
    let mut paint_ctx = PaintContext::new(300, 150);
    let shape_manager = paint_ctx.shape_manager();
    let line = LineShape::from_f64(0.0, 0.0, 50.0, 50.0);
    shape_manager.add(Box::new(line));
    paint_ctx.update();

    let paint_device = paint_ctx.paint_device();

    // Save to png file
    {
        let data = paint_device.data(EncodedImageFormat::PNG);
        let mut file = File::create("out.png").unwrap();
        let bytes = data.as_bytes();
        file.write_all(bytes).unwrap();
    }

    // Save to jpg file
    {
        let data = paint_device.data(EncodedImageFormat::JPEG);
        let mut file = File::create("out.jpg").unwrap();
        let bytes = data.as_bytes();
        file.write_all(bytes).unwrap();
    }
}
