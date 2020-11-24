use super::color::Color;
use crate::core::length::Length;

use super::{Background, Margin, Padding, Property};

#[derive(Debug, Clone, PartialEq)]
pub struct Style {
    pub background: Background,
    pub margin: Margin,
    pub padding: Padding,
    pub width: Length,
    pub height: Length,
}

impl Style {
    pub fn new() -> Style {
        Style {
            background: Background::new(),
            margin: Margin::new(),
            padding: Padding::new(),
            width: Length::Auto,
            height: Length::Auto,
            // border: Border {},
            // clip_path: ClipPath
        }
    }
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
    pub fn with_bg_image(&mut self, src: String) -> &mut Self {
        self.style.background.image = src;
        self
    }

    #[inline]
    pub fn with_margin(&mut self, margin: Margin) -> &mut Self {
        self.style.margin = margin;
        self
    }

    #[inline]
    pub fn with_padding(&mut self, padding: Padding) -> &mut Self {
        self.style.padding = padding;
        self
    }

    #[inline]
    pub fn with_width(&mut self, width: Length) -> &mut Self {
        self.style.width = width;
        self
    }

    #[inline]
    pub fn with_height(&mut self, height: Length) -> &mut Self {
        self.style.height = height;
        self
    }

    #[inline]
    pub fn build(&self) -> Style {
        self.style.clone()
    }
}
