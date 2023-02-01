// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

use jiao::kernel::PaintContextTrait;
use jiao::platforms::{
    ImagePaintDevice, PaintContext, PaintDevice, PdfPaintDevice, SvgPaintDevice,
};
use jiao::shapes::LineShape;
use std::fs::File;

fn draw_png() {
    let mut paint_device = ImagePaintDevice::new(cairo::Format::ARgb32, 300, 150);
    let mut paint_ctx = PaintContext::new(PaintDevice::Image(paint_device.clone()));
    let shape_manager = paint_ctx.shape_manager();
    let line = LineShape::from_f64(0.0, 0.0, 50.0, 50.0);
    shape_manager.add(Box::new(line));
    paint_ctx.update();

    let mut fd = File::create("out.png").unwrap();
    paint_device.surface().write_to_png(&mut fd).unwrap();
}

fn draw_pdf() {
    let mut paint_device = PdfPaintDevice::new(300.0, 150.0, "out.pdf");
    let mut paint_ctx = PaintContext::new(PaintDevice::Pdf(paint_device.clone()));
    let shape_manager = paint_ctx.shape_manager();
    let line = LineShape::from_f64(0.0, 0.0, 50.0, 50.0);
    shape_manager.add(Box::new(line));
    paint_ctx.update();

    paint_device.surface().finish();
}

fn draw_svg() {
    let mut paint_device = SvgPaintDevice::new(300.0, 150.0, "out.svg");
    let mut paint_ctx = PaintContext::new(PaintDevice::Svg(paint_device.clone()));
    let shape_manager = paint_ctx.shape_manager();
    let line = LineShape::from_f64(0.0, 0.0, 50.0, 50.0);
    shape_manager.add(Box::new(line));
    paint_ctx.update();

    paint_device.surface().finish();
}

fn main() {
    draw_png();
    draw_pdf();
    draw_svg();
}
