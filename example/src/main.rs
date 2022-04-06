// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

use jiao::kernel::PaintContext;
use jiao::shapes::LineShape;
use web_sys::HtmlElement;
use yew::{html, Component, Context, Html, NodeRef};

#[derive(Debug)]
enum Msg {
    AddOne,
}

struct Model {
    value: i64,
    container_node: NodeRef,
    paint_ctx: Option<PaintContext>,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            value: 1,
            container_node: NodeRef::default(),
            paint_ctx: None,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::AddOne => {
                self.value += 1;
                true
            }
        }
    }

    fn rendered(&mut self, _ctx: &Context<Self>, _first_render: bool) {
        if let Some(node) = self.container_node.cast::<HtmlElement>() {
            let mut paint_ctx = PaintContext::from_dom(node);
            paint_ctx.start();
            let mut shape_manager = paint_ctx.shape_manager();
            let line = LineShape::from_f64(0.0, 0.0, 50.0, 50.0);
            shape_manager.add(Box::new(line));
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
    yew::start_app::<Model>();
}
