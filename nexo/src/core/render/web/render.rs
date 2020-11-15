use std::cell::RefCell;
use std::fmt;
use std::rc::Rc;

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{Document, HtmlElement};

use crate::core::length::Length;
use crate::core::node::Node;
use crate::core::style::Style;
use crate::core::tree::{NodeRef, Tree};

use super::Events;

pub struct WebRenderer {
    root_element_id: String,
}

impl WebRenderer {
    pub fn new() -> WebRenderer {
        WebRenderer {
            root_element_id: "nexo-root".into(),
        }
    }

    pub fn render(&self, tree: &Tree, events: Rc<RefCell<Events>>) {
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
        events: Rc<RefCell<Events>>,
        node_ref: NodeRef,
        document: &Document,
        parent: &HtmlElement,
    ) {
        let node = tree.get(node_ref);
        let listen = tree.get_listen(node_ref);

        match node {
            Node::Text { content, style } => {
                let p: HtmlElement = document.create_element("p").unwrap().dyn_into().unwrap();

                self.set_style(&p, &style);
                p.set_inner_text(content);

                if listen.click {
                    {
                        let k = events.clone();
                        let event_listener = Box::new(move |event: web_sys::MouseEvent| {
                            k.borrow_mut().events.push(100);
                            let size = k.borrow().events.len();
                            web_sys::console::log(&js_sys::Array::of3(
                                &wasm_bindgen::JsValue::from_str("Size:"),
                                &wasm_bindgen::JsValue::from_f64(size as f64),
                                &wasm_bindgen::JsValue::from_f64(event.buttons() as f64),
                            ));
                            for i in k.borrow().events.iter() {
                                web_sys::console::log(&js_sys::Array::of1(
                                    &wasm_bindgen::JsValue::from_f64(*i as f64),
                                ));
                            }
                        });
                        let c =
                            Closure::wrap(event_listener as Box<dyn FnMut(web_sys::MouseEvent)>);
                        p.set_onmousedown(Some(c.as_ref().unchecked_ref()));
                        c.forget();
                    }
                    p.set_id(&node_ref.value().to_string());
                }

                parent.append_child(&p).unwrap();

                // self.set_style(&style, &p);
                // parent.append_child(&p).unwrap();
                // if listen.click {
                //     let cb = Closure::wrap(Box::new(|| {
                //         log("interval elapsed!");
                //     }) as Box<dyn FnMut()>);
                //     p.set_onclick(Some(cb.as_ref().unchecked_ref()));
                //     // p.set_onclick(Some(&js_sys::Function::new_no_args(
                //     //     format!("do_something({});", root.value()).as_str(),
                //     // )));
                // }
                // web_sys::console::log(&js_sys::Array::of2(
                //     &wasm_bindgen::JsValue::from_str("C"),
                //     &wasm_bindgen::JsValue::from_bool(listen.click),
                // ));
            }
            _ => (self.render_children(tree, events, node_ref, document, parent)),
        }
    }

    fn render_children(
        &self,
        tree: &Tree,
        events: Rc<RefCell<Events>>,
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

        // match style.background.color {
        //     Property::Inherit => (),
        //     Property::With(x) => {
        //         s.set_property("background-color", &format!("{}", x))
        //             .unwrap();
        //     }
        // }
    }
}

// use std::fmt::Write;

// use crate::component::Component;
// use crate::layout::{Align, Layout};
// use crate::length::Length;
// use crate::node::Node;
// use crate::style::{Background, Property, Style};
// use crate::tree::{NodeRef, Tree};

// use wasm_bindgen::prelude::*;
// use wasm_bindgen::JsCast;
// use web_sys::{Document, Element, HtmlElement};

// use super::class_names::ClassNames;
// use std::vec::Vec;

// pub struct Renderer {
//     tree: Tree,
//     class_names: ClassNames,
//     current_style: Style,
//     root_element_id: String,
//     events: Vec<i32>,
// }

// impl Renderer {
//     pub fn new(root_id: String) -> Renderer {
//         Renderer::with_prefix(root_id, "nexo".into())
//     }

//     pub fn with_prefix(root_id: String, css_prefix: String) -> Renderer {
//         Renderer {
//             tree: Tree::with_capacity(100),
//             class_names: ClassNames::new(css_prefix),
//             current_style: Style::new(),
//             root_element_id: root_id,
//             events: Vec::new(),
//         }
//     }

//     fn render_children(&self, document: &Document, parent: &HtmlElement, root: NodeRef) {
//         for (_, n) in self.tree.children(root) {
//             self.render_node(document, parent, n);
//         }
//     }

//     pub fn tree_mut(&mut self) -> &mut Tree {
//         &mut self.tree
//     }

//     pub fn render(&mut self, root: NodeRef) {
//         // let root = component.render(&mut self.tree, &[]);
//         let window = web_sys::window().expect("no global `window` exists");
//         let document = window.document().expect("should have a document on window");

//         let html_root: HtmlElement = document
//             .get_element_by_id(&self.root_element_id)
//             .unwrap()
//             .dyn_into()
//             .unwrap();

//         self.render_node(&document, &html_root, root);
//     }

//     fn set_style(&self, style: &Style, element: &HtmlElement) {
//         let s = element.style();
//         match style.background.color {
//             Property::Inherit => (),
//             Property::With(x) => {
//                 s.set_property("background-color", &format!("{}", x))
//                     .unwrap();
//             }
//         }

//         s.set_property("margin", &format!("{}", style.margin))
//             .unwrap();
//         s.set_property("padding", &format!("{}", style.padding))
//             .unwrap();
//         s.set_property("width", &format!("{}", style.width))
//             .unwrap();
//         s.set_property("height", &format!("{}", style.height))
//             .unwrap();
//         s.set_property("border-radius", &format!("{}", style.radius))
//             .unwrap();
//     }

//     fn justify_align(&self, a: &Align) -> &str {
//         match a {
//             Align::Start => "nexo-justify-start",
//             Align::Center => "nexo-justify-center",
//             Align::End => "nexo-justify-end",
//             Align::Stretch => "",
//         }
//     }

//     fn align_align(&self, a: &Align) -> &str {
//         match a {
//             Align::Start => "nexo-align-start",
//             Align::Center => "nexo-align-center",
//             Align::End => "nexo-align-end",
//             Align::Stretch => "",
//         }
//     }

//     fn render_node(&self, document: &Document, parent: &HtmlElement, root: NodeRef) {
//         let node = self.tree.get(root);
//         let listen = self.tree.get_listen(root);

//         match node {
//             Node::Text { content, style } => {
//                 let p: HtmlElement = document.create_element("p").unwrap().dyn_into().unwrap();
//                 p.set_inner_html(content);
//                 self.set_style(&style, &p);
//                 parent.append_child(&p).unwrap();
//                 if listen.click {
//                     let cb = Closure::wrap(Box::new(|| {
//                         log("interval elapsed!");
//                     }) as Box<dyn FnMut()>);
//                     p.set_onclick(Some(cb.as_ref().unchecked_ref()));
//                     // p.set_onclick(Some(&js_sys::Function::new_no_args(
//                     //     format!("do_something({});", root.value()).as_str(),
//                     // )));
//                 }
//                 web_sys::console::log(&js_sys::Array::of2(
//                     &wasm_bindgen::JsValue::from_str("C"),
//                     &wasm_bindgen::JsValue::from_bool(listen.click),
//                 ));
//             }

//             Node::Element { style, layout } => {
//                 let classes = match layout {
//                     Layout::Row {
//                         main_axis,
//                         cross_axis,
//                     } => (
//                         "nexo-flex-row",
//                         self.justify_align(main_axis),
//                         self.align_align(cross_axis),
//                     ),
//                     Layout::Column {
//                         main_axis,
//                         cross_axis,
//                     } => (
//                         "nexo-flex-col",
//                         self.justify_align(main_axis),
//                         self.align_align(cross_axis),
//                     ),
//                 };

//                 let mut div: HtmlElement =
//                     document.create_element("div").unwrap().dyn_into().unwrap();
//                 div.set_class_name(format!("{} {} {}", classes.0, classes.1, classes.2).as_str());
//                 self.set_style(&style, &div);
//                 self.render_children(document, &mut div, root);
//                 if listen.click {
//                     let cb = Closure::wrap(Box::new(|| {
//                         log("interval elapsed!");
//                     }) as Box<dyn FnMut()>);
//                     div.set_onclick(Some(cb.as_ref().unchecked_ref()));
//                 }
//                 web_sys::console::log(&js_sys::Array::of2(
//                     &wasm_bindgen::JsValue::from_str("C"),
//                     &wasm_bindgen::JsValue::from_bool(listen.click),
//                 ));
//                 parent.append_child(&div).unwrap();
//             }
//         }
//     }
// }

impl fmt::Display for Length {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Length::Dots(x) => write!(f, "{}px", x),
            Length::Auto => write!(f, "auto"),
        }
    }
}
