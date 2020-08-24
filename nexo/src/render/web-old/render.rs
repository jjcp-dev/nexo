use std::fmt::Write;

use crate::component::Component;
use crate::layout::Layout;
use crate::length::Length;
use crate::node::Node;
use crate::style::{Background, Property, Style};
use crate::tree::{NodeRef, Tree};

use super::css::ClassNames;

pub struct Renderer {
    tree: Tree,
    class_names: ClassNames,
    current_style: Style,
}

macro_rules! class_str {
    ($class_name:expr) => {
        concat!(r#"class=""#, $class_name, r#"""#)
    };

    ($first_class_name:expr, $($class_name:expr),*) => {
        concat!(r#"class=""#, $first_class_name, $(" ", $class_name),*, r#"""#)
    };
}

macro_rules! style_str {
    ($k:expr => $v:expr) => {
        concat!(r#"style=""#, concat!($k, ":", $v), r#"""#)
    };

    ($fk:expr => $fv:expr, $($k:expr => $v:expr),*) => {
        concat!(r#"style=""#, concat!($fk, ":", $fv), $(concat!(";", $k, ":", $v)),*, r#"""#)
    };
}

macro_rules! tag_open_str {
    ($tag:ident) => {
        concat!("<", stringify!($tag), ">")
    };

    ($tag:ident, class = { $($c:expr),* }) => {
        concat!(
            "<",
            stringify!($tag),
            " ",
            class_str!($($c),*),
            ">"
        )
    };

    ($tag:ident, style = { $($k:expr => $v:expr),* }) => {
        concat!(
            "<",
            stringify!($tag),
            " ",
            style_str!($($k => $v),*),
            ">"
        )
    };

    ($tag:ident, class = { $($c:expr),* }, style = { $($k:expr => $v:expr),* }) => {
        concat!(
            "<",
            stringify!($tag),
            " ",
            class_str!($($c),*),
            " ",
            style_str!($($k => $v),*),
            ">"
        )
    };
}

macro_rules! tag_close_str {
    ($tag:ident) => {
        concat!("</", stringify!($tag), ">")
    };
}

impl Renderer {
    pub fn new() -> Renderer {
        Renderer {
            tree: Tree::with_capacity(100),
            class_names: ClassNames::new(String::from("nexo")),
            current_style: Style::default(),
        }
    }

    fn render_children(&self, buffer: &mut String, root: NodeRef) {
        for (_, n) in self.tree.children(root) {
            self.render_node(buffer, n);
        }
    }

    fn write_style(&self, buffer: &mut String, style: &Style) {
        write!(
            buffer,
            concat!(
                "style=\"",
                "margin:{margin};",
                "padding:{padding};",
                "width:{width};",
                "height:{height};",
            ),
            margin = style.margin,
            padding = style.padding,
            width = style.width,
            height = style.height,
        );

        match style.background.color {
            Property::Inherit => {}
            Property::With(x) => {
                write!(buffer, "background-color:{};", x);
            }
        }

        write!(buffer, "\"");
    }

    fn render_node(&self, buffer: &mut String, root: NodeRef) {
        let node = self.tree.get(root);

        match node {
            Node::Text { content, style } => {
                write!(buffer, "<p ");
                self.write_style(buffer, &style);
                write!(buffer, ">");
                // FIXME: The text has to be HTML encoded!
                write!(buffer, "{}", content);
                write!(buffer, "</p>");
            }

            Node::Element { style, layout } => match layout {
                Layout::Row => {
                    write!(buffer, r#"<div class=""#);
                    write!(buffer, "{}\" ", self.class_names.flex_row());
                    self.write_style(buffer, &style);
                    write!(buffer, ">");
                    self.render_children(buffer, root);
                    write!(buffer, "</div>");
                }
                Layout::Column => {
                    write!(buffer, r#"<div class=""#);
                    write!(buffer, "{}\" ", self.class_names.flex_col());
                    self.write_style(buffer, &style);
                    write!(buffer, ">");
                    self.render_children(buffer, root);
                    write!(buffer, "</div>");
                }
            },
            _ => (),
        }
    }

    pub fn render<T: Component>(&mut self, component: T) -> String {
        let root = component.render(&mut self.tree, &[]);
        let mut output = String::with_capacity(1 * 1024 * 1024);

        self.current_style = Style::default();

        self.render_node(&mut output, root);
        output
    }
}
