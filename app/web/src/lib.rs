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
use nexo::layout::{Align, Layout};
use nexo::length::Length;
use nexo::node::Node;
use nexo::spacing::{Margin, Padding};
use nexo::style::{Background, BorderRadius, Property, Style, StyleBuilder};
use nexo::tree::{NodeRef, Tree};

fn render_bla(tree: &mut Tree, parent: NodeRef) {
    let col = tree.create(
        parent,
        Node::Element {
            layout: Layout::Column {
                main_axis: Align::Stretch,
                cross_axis: Align::Start,
            },
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
            content: "Blabla".into(),
            style: StyleBuilder::new()
                .with_bg_color(Color::rgb(255, 0, 255))
                .with_margin(Margin::left(Length::Dots(10)))
                .build(),
        },
    );
}

fn panel(tree: &mut Tree, parent: NodeRef) {
    let row = tree.create(
        parent,
        Node::Element {
            layout: Layout::Row {
                main_axis: Align::End,
                cross_axis: Align::Start,
            },
            style: StyleBuilder::new()
                .with_bg_color(Color::rgb(120, 120, 0))
                .with_width(Length::Auto)
                .build(),
        },
    );

    tree.create(
        row,
        Node::Text {
            content: "Services".into(),
            style: StyleBuilder::new()
                .with_margin(Margin::horizontal(Length::Dots(5)))
                .build(),
        },
    );

    tree.create(
        row,
        Node::Text {
            content: "About Us".into(),
            style: StyleBuilder::new()
                .with_margin(Margin::horizontal(Length::Dots(5)))
                .build(),
        },
    );
}

#[wasm_bindgen]
pub fn greet() {
    use nexo::render::web::Renderer;

    let mut renderer = Renderer::new("nexo-root".into());

    let tree = renderer.tree_mut();
    let root = tree.root();

    let row = tree.create(
        root,
        Node::Element {
            layout: Layout::Row {
                main_axis: Align::Stretch,
                cross_axis: Align::Start,
            },
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

    render_bla(tree, row);
    panel(tree, root);

    renderer.render(root);
}
