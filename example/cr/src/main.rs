// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

use jiao::kernel::PaintContextTrait;
use jiao::platforms::cr::paint_device::ImagePaintDevice;
use jiao::platforms::{PaintContext, PaintDevice};
use jiao::shapes::LineShape;
use std::fs::File;

fn main() {
    let paint_device = ImagePaintDevice::new(cairo::Format::ARgb32, 300, 150);
    let mut paint_ctx = PaintContext::new(PaintDevice::Image(paint_device.clone()));
    let shape_manager = paint_ctx.shape_manager();
    let line = LineShape::from_f64(0.0, 0.0, 50.0, 50.0);
    shape_manager.add(Box::new(line));
    paint_ctx.update();

    let mut fd = File::create("out.png").unwrap();
    paint_device.surface().write_to_png(&mut fd).unwrap();
}
