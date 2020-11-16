mod utils;

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

use nexo::core::length::Length;
use nexo::core::node::Node;
use nexo::core::render::web::WebRenderer;
use nexo::core::style::*;
use nexo::core::tree::{ListenTo, Tree};

#[wasm_bindgen]
pub struct NexoApp {
    tree: Tree,
    renderer: WebRenderer,
    c: JsValue,
}

#[wasm_bindgen]
impl NexoApp {
    pub fn new() -> NexoApp {
        NexoApp {
            tree: Tree::new(),
            renderer: WebRenderer::new(),
            c: JsValue::null(),
        }
    }

    pub fn hola(&self) {
        // web_sys::console::log(&js_sys::Array::of1(&wasm_bindgen::JsValue::from_str(
        //     "Hola!",
        // )));
    }

    pub fn set(&mut self, cb: JsValue) {
        // self.c = cb;
        cb.dyn_ref::<js_sys::Function>()
            .unwrap()
            .call0(&JsValue::null())
            .unwrap();
    }
}

#[wasm_bindgen]
pub fn greet() {
    let mut tree = Tree::new();
    let mut events = nexo::core::render::web::Events {
        events: std::vec::Vec::new(),
    };
    let renderer = WebRenderer::new();
    let root = tree.root();

    tree.create(
        root,
        Node::Text {
            content: "Hola<b>HOLA</b>".into(),
            style: StyleBuilder::new()
                .with_margin(Margin::all(Length::Dots(30)))
                .with_padding(Padding::all(Length::Dots(10)))
                .build(),
        },
        ListenTo { click: true },
    );
    tree.create(
        root,
        Node::Text {
            content: "Hola<b>HOLA 2</b>".into(),
            style: StyleBuilder::new()
                // .with_margin(Margin::all(Length::Dots(30)))
                // .with_padding(Padding::all(Length::Dots(10)))
                .build(),
        },
        ListenTo { click: false },
    );

    renderer.render(&tree, std::rc::Rc::new(std::cell::RefCell::new(events)));
}

// use nexo::color::Color;
// use nexo::component::Component;
// use nexo::layout::{Align, Layout};
// use nexo::length::Length;
// use nexo::node::Node;
// use nexo::spacing::{Margin, Padding};
// use nexo::style::{Background, BorderRadius, Property, Style, StyleBuilder};
// use nexo::tree::{ListenTo, NodeRef, Tree};

// fn render_bla(tree: &mut Tree, parent: NodeRef) {
//     let col = tree.create(
//         parent,
//         Node::Element {
//             layout: Layout::Column {
//                 main_axis: Align::Stretch,
//                 cross_axis: Align::Start,
//             },
//             style: Style::new(),
//         },
//         ListenTo { click: true },
//     );
//     tree.create(
//         col,
//         Node::Text {
//             content: "Hola".into(),
//             style: Style::new(),
//         },
//         ListenTo::new(),
//     );
//     tree.create(
//         col,
//         Node::Text {
//             content: "Hola".into(),
//             style: Style::new(),
//         },
//         ListenTo::new(),
//     );
//     tree.create(
//         col,
//         Node::Text {
//             content: "Blabla".into(),
//             style: StyleBuilder::new()
//                 .with_bg_color(Color::rgb(255, 0, 255))
//                 .with_margin(Margin::left(Length::Dots(10)))
//                 .build(),
//         },
//         ListenTo::new(),
//     );
// }

// fn panel(tree: &mut Tree, parent: NodeRef) {
//     let row = tree.create(
//         parent,
//         Node::Element {
//             layout: Layout::Row {
//                 main_axis: Align::Start,
//                 cross_axis: Align::Center,
//             },
//             style: StyleBuilder::new()
//                 .with_bg_color(Color::rgb(120, 120, 0))
//                 .with_height(Length::Dots(50))
//                 .build(),
//         },
//         ListenTo { click: true },
//     );

//     tree.create(
//         row,
//         Node::Text {
//             content: "Services".into(),
//             style: StyleBuilder::new()
//                 .with_margin(Margin::horizontal(Length::Dots(5)))
//                 .build(),
//         },
//         ListenTo::new(),
//     );

//     tree.create(
//         row,
//         Node::Text {
//             content: "About Us".into(),
//             style: StyleBuilder::new()
//                 .with_margin(Margin::horizontal(Length::Dots(5)))
//                 .build(),
//         },
//         ListenTo::new(),
//     );
// }

// use nexo::core::render::WebRenderer;

// #[wasm_bindgen]
// pub struct Hola {
//     renderer: WebRenderer,
// }

// #[wasm_bindgen]
// impl Hola {
//     pub fn new(root_element_id: &str) -> Hola {
//         Hola {
//             renderer: WebRenderer::new(),
//         }
//     }
// }

// #[wasm_bindgen]
// pub fn greet() {
//     let mut renderer = WebRenderer::new();

//     let tree = renderer.tree_mut();
//     let root = tree.root();

//     let row = tree.create(
//         root,
//         Node::Element {
//             layout: Layout::Row {
//                 main_axis: Align::Stretch,
//                 cross_axis: Align::Start,
//             },
//             style: Style::new(),
//         },
//         ListenTo::new(),
//     );

//     tree.create(
//         row,
//         Node::Text {
//             content: "Hola".into(),
//             style: Style::new(),
//         },
//         ListenTo::new(),
//     );

//     tree.create(
//         row,
//         Node::Text {
//             content: "Pepe".into(),
//             style: Style::new(),
//         },
//         ListenTo::new(),
//     );
//     tree.create(
//         row,
//         Node::Text {
//             content: "Hola".into(),
//             style: Style::new(),
//         },
//         ListenTo::new(),
//     );

//     render_bla(tree, row);
//     panel(tree, root);

//     renderer.render(root);
// }
