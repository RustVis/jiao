// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

use gtk::prelude::*;
use jiao::kernel::PaintContextTrait;
use jiao::platforms::cairo::DirectPaintContext;
use paint_shapes::paint_shapes;

fn main() {
    let application =
        gtk::Application::new(Some("org.biofan.jiao.cairo-example"), Default::default());

    application.connect_activate(build_ui);

    application.run();
}

fn build_ui(app: &gtk::Application) {
    let window = gtk::ApplicationWindow::new(app);
    set_visual(&window, None);

    window.connect_screen_changed(set_visual);
    window.connect_draw(do_draw);

    window.set_position(gtk::WindowPosition::Center);
    window.set_title("Draw cairo shapes");
    window.set_default_size(paint_shapes::CANVAS_WIDTH, paint_shapes::CANVAS_HEIGHT);
    // Make window transparent
    window.set_app_paintable(true);

    window.show_all();
}

fn set_visual(window: &gtk::ApplicationWindow, _screen: Option<&gdk::Screen>) {
    if let Some(screen) = GtkWindowExt::screen(window) {
        if let Some(ref visual) = screen.rgba_visual() {
            window.set_visual(Some(visual));
        }
    }
}

fn do_draw(_window: &gtk::ApplicationWindow, ctx: &cairo::Context) -> gtk::Inhibit {
    // ctx.set_source_rgba(1.0, 1.0, 1.0, 0.7);
    // ctx.set_operator(cairo::Operator::Screen);
    // ctx.paint().expect("Invalid cairo surface state");

    ctx.set_source_rgba(0.0, 0.0, 0.0, 1.0);

    let mut paint_ctx = DirectPaintContext::new();
    let mut shape_manager = paint_ctx.shape_manager();
    paint_shapes(&mut shape_manager);
    paint_ctx.set_cairo_context(ctx);
    paint_ctx.update();

    gtk::Inhibit(false)
}
