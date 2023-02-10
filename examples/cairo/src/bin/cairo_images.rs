// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

use jiao::error::Error;
use jiao::kernel::PaintContextTrait;
use jiao::platforms::cairo::{
    ImagePaintDevice, PaintContext, PaintDevice, PdfPaintDevice, SvgPaintDevice,
};
use paint_shapes::paint_shapes;
use std::fs::File;

fn draw_png() -> Result<(), Error> {
    let mut paint_device = ImagePaintDevice::new(
        cairo::Format::ARgb32,
        paint_shapes::CANVAS_WIDTH,
        paint_shapes::CANVAS_HEIGHT,
    )?;
    let mut paint_ctx = PaintContext::new(PaintDevice::Image(paint_device.clone()));
    let mut shape_manager = paint_ctx.shape_manager();
    paint_shapes(&mut shape_manager);
    paint_ctx.update();
    let mut fd = File::create("out-cairo.png")?;
    paint_device.surface().write_to_png(&mut fd)?;
    Ok(())
}

fn draw_pdf() -> Result<(), Error> {
    let mut paint_device = PdfPaintDevice::new(
        paint_shapes::CANVAS_WIDTH as f64,
        paint_shapes::CANVAS_HEIGHT as f64,
        "out-cairo.pdf",
    )?;
    let mut paint_ctx = PaintContext::new(PaintDevice::Pdf(paint_device.clone()));
    let mut shape_manager = paint_ctx.shape_manager();
    paint_shapes(&mut shape_manager);
    paint_ctx.update();
    paint_device.surface().finish();
    Ok(())
}

fn draw_svg() -> Result<(), Error> {
    let mut paint_device = SvgPaintDevice::new(
        paint_shapes::CANVAS_WIDTH as f64,
        paint_shapes::CANVAS_HEIGHT as f64,
        "out-cairo.svg",
    )?;
    let mut paint_ctx = PaintContext::new(PaintDevice::Svg(paint_device.clone()));
    let mut shape_manager = paint_ctx.shape_manager();
    paint_shapes(&mut shape_manager);
    paint_ctx.update();
    paint_device.surface().finish();
    Ok(())
}

fn main() -> Result<(), Error> {
    draw_png()?;
    draw_pdf()?;
    draw_svg()?;
    Ok(())
}
