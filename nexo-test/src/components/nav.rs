use nexo::color::Color;
use nexo::component::Component;
use nexo::layout::Layout;
use nexo::length::Length;
use nexo::node::Node;
use nexo::spacing::{Margin, Padding, Spacing};
use nexo::style::{Background, Property, Style};
use nexo::tree::{NodeRef, Tree};

pub struct Nav {}

impl Component for Nav {
    fn render(&self, tree: &mut Tree, _children: &[NodeRef]) -> NodeRef {
        let children = &[
            tree.create(Node::Text(String::from("HOW WE WORK")), &[]),
            tree.create(Node::Text(String::from("PORTFOLIO")), &[]),
            tree.create(Node::Text(String::from("SERVICES")), &[]),
            tree.create(Node::Text(String::from("FAQ")), &[]),
            tree.create(Node::Text(String::from("ABOUT")), &[]),
            tree.create(Node::Text(String::from("CONTACT")), &[]),
        ];

        tree.create(
            Node::Element {
                layout: Layout::Row,
                style: Style::default(),
            },
            children,
        )
    }
}
