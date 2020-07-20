use nexo::background::Background;
use nexo::color::Color;
use nexo::component::Component;
use nexo::geometry::Geometry;
use nexo::justify::Justify;
use nexo::length::Length;
use nexo::node::Node;
use nexo::spacing::{Margin, Padding, Spacing};
use nexo::tree::{NodeRef, Tree};

pub struct Nav {}

impl Component for Nav {
    fn render(&self, tree: &mut Tree, _children: &[NodeRef]) -> NodeRef {
        let children = &[
            tree.create(
                Node::Text {
                    geometry: Geometry {
                        x: Length::Dots(0),
                        y: Length::Dots(0),
                        width: Length::Auto,
                        height: Length::Auto,
                        max_width: Length::Auto,
                    },
                    spacing: Spacing {
                        margin: Margin {
                            top: Length::Dots(0),
                            right: Length::Dots(0),
                            bottom: Length::Dots(0),
                            left: Length::Dots(0),
                        },
                        padding: Padding {
                            top: Length::Dots(10),
                            right: Length::Dots(10),
                            bottom: Length::Dots(10),
                            left: Length::Dots(10),
                        },
                    },
                    background: Background {
                        color: Color::white(),
                    },
                    content: String::from("HOW WE WORK"),
                },
                &[],
            ),
            tree.create(
                Node::Text {
                    geometry: Geometry {
                        x: Length::Dots(0),
                        y: Length::Dots(0),
                        width: Length::Auto,
                        height: Length::Auto,
                        max_width: Length::Auto,
                    },
                    spacing: Spacing {
                        margin: Margin {
                            top: Length::Dots(0),
                            right: Length::Dots(0),
                            bottom: Length::Dots(0),
                            left: Length::Dots(0),
                        },
                        padding: Padding {
                            top: Length::Dots(10),
                            right: Length::Dots(10),
                            bottom: Length::Dots(10),
                            left: Length::Dots(10),
                        },
                    },
                    background: Background {
                        color: Color::white(),
                    },
                    content: String::from("PORTFOLIO"),
                },
                &[],
            ),
            tree.create(
                Node::Text {
                    geometry: Geometry {
                        x: Length::Dots(0),
                        y: Length::Dots(0),
                        width: Length::Auto,
                        height: Length::Auto,
                        max_width: Length::Auto,
                    },
                    spacing: Spacing {
                        margin: Margin {
                            top: Length::Dots(0),
                            right: Length::Dots(0),
                            bottom: Length::Dots(0),
                            left: Length::Dots(0),
                        },
                        padding: Padding {
                            top: Length::Dots(10),
                            right: Length::Dots(10),
                            bottom: Length::Dots(10),
                            left: Length::Dots(10),
                        },
                    },
                    background: Background {
                        color: Color::white(),
                    },
                    content: String::from("SERVICES"),
                },
                &[],
            ),
            tree.create(
                Node::Text {
                    geometry: Geometry {
                        x: Length::Dots(0),
                        y: Length::Dots(0),
                        width: Length::Auto,
                        height: Length::Auto,
                        max_width: Length::Auto,
                    },
                    spacing: Spacing {
                        margin: Margin {
                            top: Length::Dots(0),
                            right: Length::Dots(0),
                            bottom: Length::Dots(0),
                            left: Length::Dots(0),
                        },
                        padding: Padding {
                            top: Length::Dots(10),
                            right: Length::Dots(10),
                            bottom: Length::Dots(10),
                            left: Length::Dots(10),
                        },
                    },
                    background: Background {
                        color: Color::white(),
                    },
                    content: String::from("FAQ"),
                },
                &[],
            ),
            tree.create(
                Node::Text {
                    geometry: Geometry {
                        x: Length::Dots(0),
                        y: Length::Dots(0),
                        width: Length::Auto,
                        height: Length::Auto,
                        max_width: Length::Auto,
                    },
                    spacing: Spacing {
                        margin: Margin {
                            top: Length::Dots(0),
                            right: Length::Dots(0),
                            bottom: Length::Dots(0),
                            left: Length::Dots(0),
                        },
                        padding: Padding {
                            top: Length::Dots(10),
                            right: Length::Dots(10),
                            bottom: Length::Dots(10),
                            left: Length::Dots(10),
                        },
                    },
                    background: Background {
                        color: Color::white(),
                    },
                    content: String::from("ABOUT"),
                },
                &[],
            ),
            tree.create(
                Node::Text {
                    geometry: Geometry {
                        x: Length::Dots(0),
                        y: Length::Dots(0),
                        width: Length::Auto,
                        height: Length::Auto,
                        max_width: Length::Auto,
                    },
                    spacing: Spacing {
                        margin: Margin {
                            top: Length::Dots(0),
                            right: Length::Dots(0),
                            bottom: Length::Dots(0),
                            left: Length::Dots(0),
                        },
                        padding: Padding {
                            top: Length::Dots(10),
                            right: Length::Dots(10),
                            bottom: Length::Dots(10),
                            left: Length::Dots(10),
                        },
                    },
                    background: Background {
                        color: Color::white(),
                    },
                    content: String::from("CONTACT"),
                },
                &[],
            ),
        ];

        tree.create(
            Node::Row {
                geometry: Geometry {
                    x: Length::Dots(0),
                    y: Length::Dots(0),
                    width: Length::Dots(200),
                    height: Length::Dots(150),
                    max_width: Length::Auto,
                },
                spacing: Spacing {
                    margin: Margin {
                        top: Length::Dots(0),
                        right: Length::Dots(0),
                        bottom: Length::Dots(0),
                        left: Length::Dots(0),
                    },
                    padding: Padding {
                        top: Length::Dots(0),
                        right: Length::Dots(0),
                        bottom: Length::Dots(0),
                        left: Length::Dots(0),
                    },
                },
                justify: Justify::Start,
                background: Background {
                    color: Color::white(),
                },
            },
            children,
        )
    }
}
