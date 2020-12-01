use std::cell::RefCell;
use std::fmt;
use std::rc::Rc;

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{Document, HtmlElement};

use crate::core::event::Event;
use crate::core::layout::{Align, Layout};
use crate::core::length::Length;
use crate::core::node::Node;
use crate::core::style::{Color, Property, Style};
use crate::core::tree::{NodeRef, Tree};

use super::Closures;

pub struct WebRenderer {
    root_element_id: String,
}

impl WebRenderer {
    pub fn new() -> WebRenderer {
        WebRenderer {
            root_element_id: "nexo-root".into(),
        }
    }

    pub fn render(&self, tree: &Tree, events: Rc<Closures>) {
        let window = web_sys::window().expect("Missing `window` object");
        let document = window.document().expect("Missing `document` on `window`");

        let nexo_root: HtmlElement = document
            .get_element_by_id(&self.root_element_id)
            .unwrap()
            .dyn_into()
            .unwrap();

        self.render_node(tree, events, tree.root(), &document, &nexo_root);
    }

    pub fn render_node(
        &self,
        tree: &Tree,
        events: Rc<Closures>,
        node_ref: NodeRef,
        document: &Document,
        parent: &HtmlElement,
    ) {
        let node = tree.get(node_ref);
        let emitter_config = tree.get_emitter_config(node_ref);

        let tag = match node {
            Node::Text { content, style } => {
                let p: HtmlElement = document.create_element("p").unwrap().dyn_into().unwrap();

                self.set_style(&p, &style);
                self.bind_event_callbacks(&events, &p, &emitter_config);
                p.set_inner_text(content);

                p
            }
            Node::Element { layout, style } => {
                let classes = match layout {
                    Layout::Row {
                        main_axis,
                        cross_axis,
                    } => (
                        "nexo-flex-row",
                        self.justify_align(main_axis),
                        self.align_align(cross_axis),
                    ),
                    Layout::Column {
                        main_axis,
                        cross_axis,
                    } => (
                        "nexo-flex-col",
                        self.justify_align(main_axis),
                        self.align_align(cross_axis),
                    ),
                };

                let div: HtmlElement = document.create_element("div").unwrap().dyn_into().unwrap();

                div.set_class_name(&format!("{} {} {}", classes.0, classes.1, classes.2));

                self.set_style(&div, &style);
                self.bind_event_callbacks(&events, &div, &emitter_config);

                div
            }
        };

        self.render_children(tree, events, node_ref, document, &tag);
        parent.append_child(&tag).unwrap();
    }

    fn justify_align(&self, a: &Align) -> &str {
        match a {
            Align::Start => "nexo-justify-start",
            Align::Center => "nexo-justify-center",
            Align::End => "nexo-justify-end",
            Align::Stretch => "",
        }
    }

    fn align_align(&self, a: &Align) -> &str {
        match a {
            Align::Start => "nexo-align-start",
            Align::Center => "nexo-align-center",
            Align::End => "nexo-align-end",
            Align::Stretch => "",
        }
    }
    fn render_children(
        &self,
        tree: &Tree,
        events: Rc<Closures>,
        root: NodeRef,
        document: &Document,
        parent: &HtmlElement,
    ) {
        for (_, node_ref) in tree.children(root) {
            self.render_node(tree, events.clone(), node_ref, document, parent);
        }
    }

    fn set_style(&self, element: &HtmlElement, style: &Style) {
        let s = element.style();

        if style.margin.top != Length::Dots(0)
            || style.margin.right != Length::Dots(0)
            || style.margin.bottom != Length::Dots(0)
            || style.margin.left != Length::Dots(0)
        {
            s.set_property(
                "margin",
                &format!(
                    "{} {} {} {}",
                    style.margin.top, style.margin.right, style.margin.bottom, style.margin.left
                ),
            )
            .unwrap();
        }

        if style.padding.top != Length::Dots(0)
            || style.padding.right != Length::Dots(0)
            || style.padding.bottom != Length::Dots(0)
            || style.padding.left != Length::Dots(0)
        {
            s.set_property(
                "padding",
                &format!(
                    "{} {} {} {}",
                    style.padding.top,
                    style.padding.right,
                    style.padding.bottom,
                    style.padding.left
                ),
            )
            .unwrap();
        }

        if style.width != Length::Auto {
            s.set_property("width", &format!("{}", style.width))
                .unwrap();
        }

        if style.height != Length::Auto {
            s.set_property("height", &format!("{}", style.height))
                .unwrap();
        }

        match style.background.color {
            Property::Inherit => (),
            Property::With(x) => {
                s.set_property("background-color", &format!("{}", x))
                    .unwrap();
            }
        }

        if style.background.image != "" {
            s.set_property(
                "background-image",
                &format!("url({:?})", style.background.image),
            )
            .unwrap();
        }
    }

    #[inline]
    fn bind_event_callbacks(
        &self,
        events: &Rc<Closures>,
        tag: &HtmlElement,
        emitter_config: &crate::core::tree::EmitterConfig,
    ) {
        if emitter_config.click {
            tag.set_onmousedown(Some(events.on_click_handler.as_ref().unchecked_ref()));
        }
    }
}

impl fmt::Display for Length {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Length::Dots(x) => write!(f, "{}px", x),
            Length::Auto => write!(f, "auto"),
        }
    }
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "#{:08X}", self.value())
    }
}
