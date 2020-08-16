use nexo::color::Color;
use nexo::component::Component;
use nexo::layout::Layout;
use nexo::length::Length;
use nexo::node::Node;
use nexo::spacing::{Margin, Padding};
use nexo::style::{Background, Property, Style};
use nexo::tree::{NodeRef, Tree};

use super::nav::Nav;

pub struct App {
    name: &'static str,
}

impl App {
    pub fn new(name: &'static str) -> App {
        App { name: name }
    }
}

impl Component for App {
    fn render(&self, tree: &mut Tree, _children: &[NodeRef]) -> NodeRef {
        let children = &[tree.create(Node::Component(Box::new(Nav {})), &[])];

        tree.create(
            Node::Element {
                layout: Layout::Column,
                style: Style::default(),
            },
            children,
        )
    }
}
