// use std::fmt::Write;

use crate::component::Component;
// use crate::layout::Layout;
// use crate::length::Length;
// use crate::node::Node;
// use crate::style::{Background, Property, Style};
// use crate::tree::{NodeRef, Tree};

// use super::css::ClassNames;

pub struct Renderer {
    // tree: Tree,
    // class_names: ClassNames,
    // current_style: Style,
}

impl Renderer {
    pub fn render<T: Component>(&mut self, component: T) -> String {
        // Use `web_sys`'s global `window` function to get a handle on the global
        // window object.
        let window = web_sys::window().expect("no global `window` exists");
        let document = window.document().expect("should have a document on window");
        let body = document.body().expect("document should have a body");

        // Manufacture the element we're gonna append
        let val = document.create_element("p").unwrap();
        val.set_inner_html("Hello from Rust!");
        "".to_string()
    }
}
