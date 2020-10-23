use super::background::Background;
use super::component::Component;
use super::justify::Justify;

use super::layout::Layout;
use super::style::Style;

#[derive(Debug)]
pub enum Node {
    Text { content: String, style: Style },
    Element { layout: Layout, style: Style },
}
