use super::background::Background;
use super::component::Component;
use super::geometry::Geometry;
use super::justify::Justify;
use super::spacing::Spacing;

pub enum Node {
    Row {
        geometry: Geometry,
        spacing: Spacing,
        background: Background,
        justify: Justify,
    },

    Column {
        geometry: Geometry,
        spacing: Spacing,
        background: Background,
        justify: Justify,
    },

    Text {
        geometry: Geometry,
        spacing: Spacing,
        background: Background,
        content: String,
    },

    Component(Box<dyn Component>),
}
