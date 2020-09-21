use std::fmt::Write;

use crate::component::Component;
use crate::layout::Layout;
use crate::length::Length;
use crate::node::Node;
use crate::style::{Background, Property, Style};
use crate::tree::{NodeRef, Tree};

use web_sys::{HtmlElement, Element, Document};

use super::css::ClassNames;

pub struct Renderer {
    tree: Tree,
    class_names: ClassNames,
    current_style: Style,
}

impl Renderer {
    pub fn new() -> Renderer {
        Renderer {
            tree: Tree::with_capacity(100),
            class_names: ClassNames::new(String::from("nexo")),
            current_style: Style::default(),
        }
    }

    fn render_children(&self, document: &mut Document, parent: &mut Element, root: NodeRef) {
        for (_, n) in self.tree.children(root) {
            self.render_node(document, parent, n);
        }
    }

    // fn write_style(&self, buffer: &mut String, style: &Style) {
    //     write!(
    //         buffer,
    //         concat!(
    //             "style=\"",
    //             "margin:{margin};",
    //             "padding:{padding};",
    //             "width:{width};",
    //             "height:{height};",
    //         ),
    //         margin = style.margin,
    //         padding = style.padding,
    //         width = style.width,
    //         height = style.height,
    //     );

    //     match style.background.color {
    //         Property::Inherit => {}
    //         Property::With(x) => {
    //             write!(buffer, "background-color:{};", x);
    //         }
    //     }

    //     write!(buffer, "\"");
    // }

    fn render_node(&self, document: &mut Document, parent: &mut Element, root: NodeRef) {
        let node = self.tree.get(root);

        match node {
            Node::Text { content, style } => {
                let mut p = document.create_element("p").unwrap();
                p.set_inner_html(content);
                parent.append_child(&p).unwrap();
            }

            Node::Element { style, layout } => match layout {
                Layout::Row => {
                    let mut div = document.create_element("div").unwrap();
                    self.render_children(document, &mut div, root);
                    parent.append_child(&div).unwrap();
                }
                Layout::Column => {
                    let mut div = document.create_element("div").unwrap();
                    self.render_children(document, &mut div, root);
                    parent.append_child(&div).unwrap();
                }
            },
            _ => (),
        }
    }

    pub fn render<T: Component>(&mut self, component: T) {
        let root = component.render(&mut self.tree, &[]);

        let window = web_sys::window().expect("no global `window` exists");
        let mut document = window.document().expect("should have a document on window");
        let body = document.body().expect("document should have a body");

        let mut div = document.create_element("div").unwrap();
        self.render_node(&mut document, &mut div, root);
        body.append_child(&div).unwrap();
    }
}
