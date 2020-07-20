use super::color::Color;
use super::component::Component;
use super::geometry::Geometry;
use super::spacing::Spacing;

pub enum Node {
    Row {
        geometry: Geometry,
        spacing: Spacing,
    },

    Column {
        geometry: Geometry,
        spacing: Spacing,
    },

    Text {
        geometry: Geometry,
        spacing: Spacing,
        content: String,
    },

    Component(Box<dyn Component>),
}
