mod utils;

use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

use nexo::color::Color;
use nexo::component::Component;
use nexo::layout::Layout;
use nexo::length::Length;
use nexo::node::Node;
use nexo::spacing::{Margin, Padding};
use nexo::style::{Background, BorderRadius, Property, Style, StyleBuilder};
use nexo::tree::{NodeRef, Tree};

#[wasm_bindgen]
pub fn greet() {
    use nexo::render::web::Renderer;

    let mut renderer = Renderer::new("nexo-root".into());

    let tree = renderer.tree_mut();
    let root = tree.root();

    let row = tree.create(
        root,
        Node::Element {
            layout: Layout::Row,
            style: Style::new(),
        },
    );

    tree.create(
        row,
        Node::Text {
            content: "Hola".into(),
            style: Style::new(),
        },
    );

    tree.create(
        row,
        Node::Text {
            content: "Pepe".into(),
            style: Style::new(),
        },
    );
    tree.create(
        row,
        Node::Text {
            content: "Hola".into(),
            style: Style::new(),
        },
    );

    let col = tree.create(
        row,
        Node::Element {
            layout: Layout::Column,
            style: Style::new(),
        },
    );
    tree.create(
        col,
        Node::Text {
            content: "Hola".into(),
            style: Style::new(),
        },
    );
    tree.create(
        col,
        Node::Text {
            content: "Hola".into(),
            style: Style::new(),
        },
    );
    tree.create(
        col,
        Node::Text {
            content: "Hola".into(),
            style: StyleBuilder::new()
                .with_bg_color(Color::rgb(255, 0, 255))
                .with_margin(Margin::left(Length::Dots(10)))
                .build(),
        },
    );
    renderer.render(root);
}
