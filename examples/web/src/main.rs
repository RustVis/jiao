// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

use jiao::base::RectF;
use jiao::kernel::PaintContextTrait;
use jiao::platforms::web::PaintContext;
use jiao::shapes::{LineShape, RectShape};
use web_sys::HtmlElement;
use yew::prelude::*;

#[derive(Debug)]
enum Msg {}

struct AppComponent {
    container_node: NodeRef,
    paint_ctx: Option<PaintContext>,
}

impl Component for AppComponent {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            container_node: NodeRef::default(),
            paint_ctx: None,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
        false
    }

    fn rendered(&mut self, _ctx: &Context<Self>, _first_render: bool) {
        if let Some(node) = self.container_node.cast::<HtmlElement>() {
            let mut paint_ctx = PaintContext::from_dom(&node);
            let shape_manager = paint_ctx.shape_manager();
            let line = LineShape::from_f64(0.0, 0.0, 50.0, 50.0);
            shape_manager.add(Box::new(line));
            let rect = RectShape::from_rect(RectF::from(10.0, 10.0, 25.0, 25.0));
            shape_manager.add(Box::new(rect));
            paint_ctx.update();
            self.paint_ctx = Some(paint_ctx);
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let _link = ctx.link();
        html! {
            <div class="container" ref={self.container_node.clone()}>
            </div>
        }
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<AppComponent>::new().render();
}
