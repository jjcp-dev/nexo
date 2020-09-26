use std::fmt::Write;

use crate::component::Component;
use crate::layout::Layout;
use crate::length::Length;
use crate::node::Node;
use crate::style::{Background, Property, Style};
use crate::tree::{NodeRef, Tree};

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{Document, Element, HtmlElement};

use super::css::ClassNames;

pub struct Renderer {
    tree: Tree,
    class_names: ClassNames,
    current_style: Style,
    root_element_id: String,
}

impl Renderer {
    pub fn new(root_id: String) -> Renderer {
        Renderer {
            tree: Tree::with_capacity(100),
            class_names: ClassNames::new(String::from("nexo")),
            current_style: Style::default(),
            root_element_id: root_id,
        }
    }

    fn render_children(&self, document: &Document, parent: &HtmlElement, root: NodeRef) {
        for (_, n) in self.tree.children(root) {
            self.render_node(document, parent, n);
        }
    }

    pub fn render<T: Component>(&mut self, component: T) {
        let root = component.render(&mut self.tree, &[]);
        let window = web_sys::window().expect("no global `window` exists");
        let document = window.document().expect("should have a document on window");

        let html_root: HtmlElement = document
            .get_element_by_id(&self.root_element_id)
            .unwrap()
            .dyn_into()
            .unwrap();

        self.render_node(&document, &html_root, root);
    }

    fn render_node(&self, document: &Document, parent: &HtmlElement, root: NodeRef) {
        let node = self.tree.get(root);

        match node {
            Node::Text { content, style } => {
                let p: HtmlElement = document.create_element("p").unwrap().dyn_into().unwrap();
                p.set_inner_html(content);

                let s = p.style();
                match style.background.color {
                    Property::Inherit => (),
                    Property::With(x) => {
                        s.set_property("background-color", &format!("{}", x))
                            .unwrap();
                    }
                }

                parent.append_child(&p).unwrap();
            }

            Node::Element { style, layout } => match layout {
                Layout::Row => {
                    let mut div: HtmlElement =
                        document.create_element("div").unwrap().dyn_into().unwrap();
                    div.set_class_name("nexo-flex-row");

                    self.render_children(document, &mut div, root);
                    parent.append_child(&div).unwrap();
                }
                Layout::Column => {
                    let mut div: HtmlElement =
                        document.create_element("div").unwrap().dyn_into().unwrap();
                    self.render_children(document, &mut div, root);
                    parent.append_child(&div).unwrap();
                }
            },
            _ => (),
        }
    }
}
