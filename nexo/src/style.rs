use super::color::Color;
use super::length::Length;
use super::spacing::{Margin, Padding};

#[derive(Debug, Eq, PartialEq)]
pub enum Property<T> {
    Inherit,
    With(T),
}

#[derive(Debug, Eq, PartialEq)]
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

impl Style {
    pub fn default() -> Style {
        Style {
            background: Background {
                color: Property::With(Color::white()),
            },
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
            width: Length::Auto,
            height: Length::Auto,
        }
    }
}
