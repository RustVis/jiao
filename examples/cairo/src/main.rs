// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

use jiao::error::Error;
use jiao::kernel::{PaintContextTrait, ShapeManager};
use jiao::platforms::cairo::{
    ImagePaintDevice, PaintContext, PaintDevice, PdfPaintDevice, SvgPaintDevice,
};
use jiao::shapes::LineShape;
use std::fs::File;

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

fn draw_png() -> Result<(), Error> {
    let mut paint_device = ImagePaintDevice::new(cairo::Format::ARgb32, 300, 150)?;
    let mut paint_ctx = PaintContext::new(PaintDevice::Image(paint_device.clone()));
    let mut shape_manager = paint_ctx.shape_manager();
    do_paint(&mut shape_manager);
    paint_ctx.update();
    let mut fd = File::create("out.png")?;
    paint_device.surface().write_to_png(&mut fd)?;
    Ok(())
}

fn draw_pdf() -> Result<(), Error> {
    let mut paint_device = PdfPaintDevice::new(300.0, 150.0, "out.pdf")?;
    let mut paint_ctx = PaintContext::new(PaintDevice::Pdf(paint_device.clone()));
    let mut shape_manager = paint_ctx.shape_manager();
    do_paint(&mut shape_manager);
    paint_ctx.update();
    paint_device.surface().finish();
    Ok(())
}

fn draw_svg() -> Result<(), Error> {
    let mut paint_device = SvgPaintDevice::new(300.0, 150.0, "out.svg")?;
    let mut paint_ctx = PaintContext::new(PaintDevice::Svg(paint_device.clone()));
    let mut shape_manager = paint_ctx.shape_manager();
    do_paint(&mut shape_manager);
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
