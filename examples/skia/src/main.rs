// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

use jiao::error::{Error, ErrorKind};
use jiao::kernel::PaintContextTrait;
use jiao::platforms::skia::{ImagePaintDevice, PaintContext, PaintDevice};
use paint_shapes::paint_shapes;
use skia_safe::EncodedImageFormat;
use std::fs::File;
use std::io::Write;

fn draw_png() -> Result<(), Error> {
    let paint_device = PaintDevice::Image(ImagePaintDevice::new(
        paint_shapes::CANVAS_WIDTH,
        paint_shapes::CANVAS_HEIGHT,
    ));
    let mut paint_ctx = PaintContext::new(paint_device);
    let mut shape_manager = paint_ctx.shape_manager();
    paint_shapes(&mut shape_manager);
    paint_ctx.update();

    let paint_device = paint_ctx.paint_device();
    if let PaintDevice::Image(img_paint_device) = paint_device {
        let data = img_paint_device
            .encode(EncodedImageFormat::PNG)
            .ok_or(Error::new(ErrorKind::SkiaError, "No data in image surface"))?;
        let mut file = File::create("out-skia.png")?;
        let bytes = data.as_bytes();
        file.write_all(bytes).map(drop).map_err(Into::into)
    } else {
        Err(Error::new(
            ErrorKind::SkiaError,
            "Invalid image paint device",
        ))
    }
}

fn draw_jpg() -> Result<(), Error> {
    let paint_device = PaintDevice::Image(ImagePaintDevice::new(300, 150));
    let mut paint_ctx = PaintContext::new(paint_device);
    let mut shape_manager = paint_ctx.shape_manager();
    paint_shapes(&mut shape_manager);
    paint_ctx.update();

    let paint_device = paint_ctx.paint_device();
    if let PaintDevice::Image(img_paint_device) = paint_device {
        let data = img_paint_device
            .encode(EncodedImageFormat::JPEG)
            .ok_or(Error::new(ErrorKind::SkiaError, "No data in image surface"))?;
        let mut file = File::create("out-skia.jpg")?;
        let bytes = data.as_bytes();
        file.write_all(bytes)?;
        Ok(())
    } else {
        Err(Error::new(
            ErrorKind::SkiaError,
            "Invalid image paint device",
        ))
    }
}

fn main() -> Result<(), Error> {
    draw_png()?;
    draw_jpg()?;
    Ok(())
}
