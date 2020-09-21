mod utils;

use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern {
    fn alert(s: &str);
}


use nexo::component::Component;
use nexo::tree::{Tree, NodeRef};
use nexo::node::Node;
use nexo::style::Style;
use nexo::layout::Layout;

struct App;

impl Component for App {
    fn render(&self, tree: &mut Tree, _children: &[NodeRef]) -> NodeRef {
        let c = [
            tree.create(Node::Text{content: "Hola".to_string(), style: Style::default()}, &[]),
            tree.create(Node::Text{content: "Mundo".to_string(), style: Style::default()}, &[]),
            tree.create(Node::Text{content: "Oi oi oi".to_string(), style: Style::default()}, &[]),
        ];
        tree.create(Node::Element{layout: Layout::Column, style: Style::default()}, &c)
    }
}

use nexo::render::web::render::Renderer;

#[wasm_bindgen]
pub fn greet() {
    let mut r = Renderer::new();

    r.render(App{});
}
