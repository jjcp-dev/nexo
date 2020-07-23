use super::color::Color;
use super::length::Length;
use super::spacing::{Margin, Padding};

#[derive(Debug)]
pub enum Property<T> {
    Inherit,
    With(T),
}

#[derive(Debug)]
pub struct Background {
    pub color: Property<Color>,
}

#[derive(Debug)]
pub struct Style {
    pub background: Background,
    pub margin: Margin,
    pub padding: Padding,
    pub width: Length,
    pub height: Length,
}
