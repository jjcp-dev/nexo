use super::{Color, Property};

#[derive(Debug, Clone, PartialEq)]
pub struct Background {
    pub color: Property<Color>,
    // FIXME: Implement a new type for this, or use crates.io/crates/url
    pub image: String,
}
