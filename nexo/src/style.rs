use super::color::Color;
use super::length::Length;
use super::spacing::{Margin, Padding};

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Property<T> {
    Inherit,
    With(T),
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Background {
    pub color: Property<Color>,
}

#[derive(Debug, Clone)]
pub struct BorderRadius {
    pub top_left: Length,
    pub top_right: Length,
    pub bottom_left: Length,
    pub bottom_right: Length,
}

#[derive(Debug, Clone)]
pub struct Style {
    pub background: Background,
    pub margin: Margin,
    pub padding: Padding,
    pub radius: BorderRadius,
    pub width: Length,
    pub height: Length,
}

pub struct StyleBuilder {
    style: Style,
}

impl StyleBuilder {
    #[inline]
    pub fn new() -> StyleBuilder {
        StyleBuilder {
            style: Style::new(),
        }
    }

    #[inline]
    pub fn with_bg_color(&mut self, color: Color) -> &mut Self {
        self.style.background.color = Property::With(color);
        self
    }

    #[inline]
    pub fn with_margin(&mut self, margin: Margin) -> &mut Self {
        self.style.margin = margin;
        self
    }

    #[inline]
    pub fn build(&self) -> Style {
        self.style.clone()
    }
}

impl Style {
    pub fn new() -> Style {
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
            radius: BorderRadius {
                top_left: Length::Dots(0),
                top_right: Length::Dots(0),
                bottom_left: Length::Dots(0),
                bottom_right: Length::Dots(0),
            },
            width: Length::Auto,
            height: Length::Auto,
        }
    }
}
