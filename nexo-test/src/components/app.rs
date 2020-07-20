use nexo::background::Background;
use nexo::color::Color;
use nexo::component::Component;
use nexo::geometry::Geometry;
use nexo::justify::Justify;
use nexo::length::Length;
use nexo::node::Node;
use nexo::spacing::{Margin, Padding, Spacing};
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
        // let children = &[
        //     tree.create(Node::Text(self.name.to_string()), &[]),
        //     tree.create(Node::Text("Hola".to_string()), &[]),
        //     tree.create(Node::Text("Chau".to_string()), &[]),
        // ];

        // let rec = &[tree.create(
        //     Node::Rectangle {
        //         geometry: Geometry {
        //             x: Length::Points(0),
        //             y: Length::Points(0),
        //             width: Length::Percentage(100),
        //             height: Length::Points(100),
        //         },
        //         spacing: Spacing {
        //             margin: Margin::Dots(100),
        //             padding: Padding::Dots(0),
        //         },
        //         color: Color::rgb(255, 0, 0),
        //     },
        //     children,
        // )];

        // tree.create(Node::Row, rec)

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
                    content: String::from("Pimbalabs"),
                },
                &[],
            ),
            // tree.create(
            //     Node::Text {
            //         geometry: Geometry {
            //             x: Length::Dots(0),
            //             y: Length::Dots(0),
            //             width: Length::Auto,
            //             height: Length::Auto,
            //             max_width: Length::Auto,
            //         },
            //         spacing: Spacing {
            //             margin: Margin {
            //                 top: Length::Dots(0),
            //                 right: Length::Dots(0),
            //                 bottom: Length::Dots(0),
            //                 left: Length::Dots(0),
            //             },
            //             padding: Padding {
            //                 top: Length::Dots(10),
            //                 right: Length::Dots(10),
            //                 bottom: Length::Dots(10),
            //                 left: Length::Dots(10),
            //             },
            //         },
            //         background: Background {
            //             color: Color::rgb(0, 255, 0),
            //         },
            //         content: String::from("Chau"),
            //     },
            //     &[],
            // ),
            // tree.create(
            //     Node::Text {
            //         geometry: Geometry {
            //             x: Length::Dots(0),
            //             y: Length::Dots(0),
            //             width: Length::Auto,
            //             height: Length::Auto,
            //             max_width: Length::Auto,
            //         },
            //         spacing: Spacing {
            //             margin: Margin {
            //                 top: Length::Dots(0),
            //                 right: Length::Dots(0),
            //                 bottom: Length::Dots(0),
            //                 left: Length::Dots(0),
            //             },
            //             padding: Padding {
            //                 top: Length::Dots(10),
            //                 right: Length::Dots(10),
            //                 bottom: Length::Dots(10),
            //                 left: Length::Dots(10),
            //             },
            //         },
            //         background: Background {
            //             color: Color::rgb(0, 0, 255),
            //         },
            //         content: String::from("Hola"),
            //     },
            //     &[],
            // ),
            // tree.create(
            //     Node::Text {
            //         geometry: Geometry {
            //             x: Length::Dots(0),
            //             y: Length::Dots(0),
            //             width: Length::Auto,
            //             height: Length::Auto,
            //             max_width: Length::Auto,
            //         },
            //         spacing: Spacing {
            //             margin: Margin {
            //                 top: Length::Dots(0),
            //                 right: Length::Dots(0),
            //                 bottom: Length::Dots(0),
            //                 left: Length::Dots(0),
            //             },
            //             padding: Padding {
            //                 top: Length::Dots(10),
            //                 right: Length::Dots(10),
            //                 bottom: Length::Dots(10),
            //                 left: Length::Dots(10),
            //             },
            //         },
            //         background: Background {
            //             color: Color::rgb(0, 255, 0),
            //         },
            //         content: String::from("åëúóüñ"),
            //     },
            //     &[],
            // ),
            tree.create(Node::Component(Box::new(Nav {})), &[]),
        ];

        let c = &[tree.create(
            Node::Row {
                geometry: Geometry {
                    x: Length::Dots(0),
                    y: Length::Dots(0),
                    width: Length::Auto,
                    height: Length::Auto,
                    max_width: Length::Dots(1200),
                },
                spacing: Spacing {
                    margin: Margin {
                        top: Length::Dots(10),
                        right: Length::Dots(10),
                        bottom: Length::Dots(10),
                        left: Length::Dots(10),
                    },
                    padding: Padding {
                        top: Length::Dots(0),
                        right: Length::Dots(0),
                        bottom: Length::Dots(0),
                        left: Length::Dots(0),
                    },
                },
                justify: Justify::Between,
                background: Background {
                    color: Color::rgb(255, 0, 0),
                },
            },
            children,
        )];

        tree.create(
            Node::Row {
                geometry: Geometry {
                    x: Length::Auto,
                    y: Length::Auto,
                    width: Length::Auto,
                    height: Length::Auto,
                    max_width: Length::Auto,
                },
                spacing: Spacing {
                    margin: Margin {
                        top: Length::Dots(10),
                        right: Length::Dots(10),
                        bottom: Length::Dots(10),
                        left: Length::Dots(10),
                    },
                    padding: Padding {
                        top: Length::Dots(0),
                        right: Length::Dots(0),
                        bottom: Length::Dots(0),
                        left: Length::Dots(0),
                    },
                },
                justify: Justify::Center,
                background: Background {
                    color: Color::white(),
                },
            },
            c,
        )
    }
}
