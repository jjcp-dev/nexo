use super::background::Background;
use super::component::Component;
use super::geometry::Geometry;
use super::justify::Justify;
use super::spacing::Spacing;

use super::layout::Layout;
use super::style::Style;

pub enum Node {
    Text(String),
    Element { layout: Layout, style: Style },
    Component(Box<dyn Component>),
}
