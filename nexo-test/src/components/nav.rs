use nexo::color::Color;
use nexo::component::Component;
use nexo::layout::Layout;
use nexo::length::Length;
use nexo::node::Node;
use nexo::spacing::{Margin, Padding};
use nexo::style::{Background, Property, Style};
use nexo::tree::{NodeRef, Tree};

pub struct Nav {}

impl Component for Nav {
    fn render(&self, tree: &mut Tree, _children: &[NodeRef]) -> NodeRef {
        let children = &[
            tree.create(
                Node::Text {
                    content: String::from("HOW WE WORK"),
                    style: Style::default(),
                },
                &[],
            ),
            tree.create(
                Node::Text {
                    content: String::from("PORTFOLIO"),
                    style: Style::default(),
                },
                &[],
            ),
            tree.create(
                Node::Text {
                    content: String::from("SERVICES"),
                    style: Style::default(),
                },
                &[],
            ),
            tree.create(
                Node::Text {
                    content: String::from("FAQ"),
                    style: Style::default(),
                },
                &[],
            ),
            tree.create(
                Node::Text {
                    content: String::from("ABOUT"),
                    style: Style::default(),
                },
                &[],
            ),
            tree.create(
                Node::Text {
                    content: String::from("CONTACT"),
                    style: Style::default(),
                },
                &[],
            ),
        ];

        //tree.add_on_click_listener(children[0], Box::new(|| println!("Hola")));

        tree.create(
            Node::Element {
                layout: Layout::Column,
                style: Style::default(),
            },
            children,
        )
    }
}
