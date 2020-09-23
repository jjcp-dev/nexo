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
use nexo::style::{Background, Property, Style};
use nexo::tree::{NodeRef, Tree};

struct App;

impl Component for App {
    fn render(&self, tree: &mut Tree, _children: &[NodeRef]) -> NodeRef {
        let c = [
            tree.create(
                Node::Text {
                    content: "Hola".to_string(),
                    style: Style::default(),
                },
                &[],
            ),
            tree.create(
                Node::Text {
                    content: "Mundo".to_string(),
                    style: Style {
                        background: Background {
                            color: Property::With(Color::rgb(255, 0, 255)),
                        },
                        margin: Margin {
                            top: Length::Dots(0),
                            right: Length::Dots(0),
                            bottom: Length::Dots(0),
                            left: Length::Dots(0),
                        },
                        padding: Padding {
                            top: Length::Dots(0),
                            right: Length::Dots(0),
                            bottom: Length::Dots(0),
                            left: Length::Dots(0),
                        },
                        width: Length::Auto,
                        height: Length::Auto,
                    },
                },
                &[],
            ),
            tree.create(
                Node::Text {
                    content: "Oi oi oi".to_string(),
                    style: Style::default(),
                },
                &[],
            ),
        ];
        tree.create(
            Node::Element {
                layout: Layout::Column,
                style: Style::default(),
            },
            &c,
        )
    }
}

use nexo::render::web::Renderer;

#[wasm_bindgen]
pub fn greet() {
    Renderer::new("nexo-root".to_string()).render(App {});
}
