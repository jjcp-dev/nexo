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
use nexo::core::render::web::Closures;
use nexo::core::render::web::WebRenderer;
use nexo::core::style::*;
use nexo::core::tree::{EmitterConfig, Tree};

use std::rc::Rc;
#[wasm_bindgen]
pub struct NexoApp {
    tree: Tree,
    renderer: WebRenderer,
    closures: Rc<Closures>,
}

#[wasm_bindgen]
impl NexoApp {
    pub fn new() -> NexoApp {
        NexoApp {
            tree: Tree::new(),
            renderer: WebRenderer::new(),
            closures: Rc::new(Closures::new(std::rc::Rc::new(
                |event: nexo::core::event::Event| {
                    web_sys::console::log(&js_sys::Array::of1(&wasm_bindgen::JsValue::from_str(
                        &format!("{:?}", event),
                    )));
                },
            ))),
        }
    }

    pub fn greet(&mut self) {
        let root = self.tree.root();

        self.tree.create(
        root,
        Node::Text {
            content: "Hola<b>HOLA</b>".into(),
            style: StyleBuilder::new()
                .with_margin(Margin::all(Length::Dots(30)))
                .with_padding(Padding::all(Length::Dots(200)))
                .with_bg_image("https://encrypted-tbn0.gstatic.com/images?q=tbn:ANd9GcQmKpHnYnWqm9eSqOmfvXU-foLEig2lsw7l3w&usqp=CAU".into())
                .build(),
        },
        EmitterConfig { click: true },
    );
        self.tree.create(
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

        self.renderer.render(&self.tree, self.closures.clone());
    }
}

/*
   struct MyApp;
   impl CoreApp for MyApp {
       fn startup(&mut self) -> Result<(), Self::Error>;

       fn on_event(&mut self, tree: &mut Tree, event: Event);
   }

   fn start() {
       let app = CoreLoop::new(MyApp {});
       app.run();
   }
*/
