use std::fmt::Write;

use crate::component::Component;
use crate::length::Length;
use crate::node::Node;
use crate::tree::{NodeRef, Tree};

use super::css::ClassNames;

pub struct Renderer {
    tree: Tree,
    class_names: ClassNames,
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
        }
    }

    fn render_children(&self, buffer: &mut String, root: NodeRef) {
        for (_, n) in self.tree.children(root) {
            self.render_node(buffer, n);
        }
    }

    fn render_node(&self, buffer: &mut String, root: NodeRef) {
        let node = self.tree.get(root);

        match node {
            Node::Row {
                geometry,
                spacing,
                background,
            } => {
                write!(
                    buffer,
                    tag_open_str!(
                        div,
                        class = { "{flex_class}" },
                        style = {
                            "margin" => "{margin}",
                            "padding" => "{padding}",
                            "background-color" => "{bg_color}"
                        }
                    ),
                    flex_class = self.class_names.flex_row(),
                    margin = spacing.margin,
                    padding = spacing.padding,
                    bg_color = background.color
                );

                self.render_children(buffer, root);

                write!(buffer, tag_close_str!(div));
            }

            Node::Column {
                geometry,
                spacing,
                background,
            } => {
                write!(
                    buffer,
                    tag_open_str!(
                        div,
                        class = { "{flex_class}" },
                        style = {
                            "margin" => "{margin}",
                            "padding" => "{padding}",
                            "background-color" => "{bg_color}"
                        }
                    ),
                    flex_class = self.class_names.flex_col(),
                    margin = spacing.margin,
                    padding = spacing.padding,
                    bg_color = background.color
                );

                self.render_children(buffer, root);

                write!(buffer, tag_close_str!(div));
            }

            Node::Text {
                geometry,
                spacing,
                content,
                background,
            } => {
                write!(
                    buffer,
                    tag_open_str!(
                        p,
                        style = {
                            "display" => "inline-block",
                            "margin" => "{margin}",
                            "padding" => "{padding}",
                            "background-color" => "{bg_color}"
                        }
                    ),
                    margin = spacing.margin,
                    padding = spacing.padding,
                    bg_color = background.color
                );

                write!(buffer, "{}", content);

                self.render_children(buffer, root);

                write!(buffer, tag_close_str!(p));
            }
            _ => (),
        }
    }

    pub fn render<T: Component>(&mut self, component: T) -> String {
        let root = component.render(&mut self.tree, &[]);
        let mut output = String::with_capacity(1 * 1024 * 1024);
        self.render_node(&mut output, root);
        output
    }

    #[inline]
    fn render_rectangle_open(&self, buffer: &mut String, width: Length, height: Length) {
        write!(
            buffer,
            tag_open_str!(
                div,
                class = {
                    "{rectangle}"
                },
                style = {
                    "width" => "{width}",
                    "height" => "{height}"
                }
            ),
            rectangle = self.class_names.rectangle(),
            width = width,
            height = height
        );
    }
}
