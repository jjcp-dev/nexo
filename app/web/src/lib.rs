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
use nexo::core::tree::{EmitterConfig, Tree};

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
                .with_padding(Padding::all(Length::Dots(200)))
                .with_bg_image("https://image.shutterstock.com/image-vector/abstract-futuristic-landscape-1980s-style-260nw-1139046833.jpg".into())
                .build(),
        },
        EmitterConfig { click: true },
    );
    tree.create(
        root,
        Node::Text {
            content: "Hola<b>HOLA 2</b>".into(),
            style: StyleBuilder::new()
                .with_bg_color(Color::rgb(255, 0, 255))
                // .with_margin(Margin::all(Length::Dots(30)))
                // .with_padding(Padding::all(Length::Dots(10)))
                .build(),
        },
        EmitterConfig { click: false },
    );

    renderer.render(&tree, std::rc::Rc::new(std::cell::RefCell::new(events)));
}

/*

struct MyApp {

}
impl Application for MyApp    {
    fn on_event(&mut self, ) {

    }
}

  let app = BaseApplication::new();

  app.get_tree()


 */
