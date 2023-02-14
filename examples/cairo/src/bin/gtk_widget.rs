// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

use gtk::prelude::*;
use jiao::kernel::ShapeManager;
use jiao::platforms::cairo::DirectPainter;
use paint_shapes::paint_shapes;
use std::cell::RefCell;

thread_local! {
    static SHAPE_MANAGER: RefCell<ShapeManager> = RefCell::new(ShapeManager::new());
}

fn main() {
    let application =
        gtk::Application::new(Some("org.biofan.jiao.cairo-example"), Default::default());

    SHAPE_MANAGER.with(move |shape_manager| {
        paint_shapes(&mut shape_manager.borrow_mut());
    });
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
    ctx.set_source_rgba(0.0, 0.0, 0.0, 1.0);

    SHAPE_MANAGER.with(move |shape_manager| {
        let mut painter = DirectPainter::new(ctx);
        shape_manager.borrow_mut().update(&mut painter);
    });

    gtk::Inhibit(false)
}
