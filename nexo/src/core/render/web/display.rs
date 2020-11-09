use std::fmt;

use crate::core::style::Color;
use crate::justify::Justify;
use crate::length::Length;
use crate::spacing::{Margin, Padding};
use crate::style::BorderRadius;

impl fmt::Display for Length {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Length::Dots(x) => write!(f, "{}px", x),
            Length::Auto => write!(f, "auto"),
        }
    }
}

impl fmt::Display for Margin {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} {} {} {}",
            self.top, self.right, self.bottom, self.left
        )
    }
}

impl fmt::Display for Padding {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} {} {} {}",
            self.top, self.right, self.bottom, self.left
        )
    }
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "#{:08X}", self.value())
    }
}

impl fmt::Display for Justify {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Justify::Start => write!(f, "{}", "flex-start"),
            Justify::End => write!(f, "{}", "flex-end"),
            Justify::Center => write!(f, "{}", "center"),
            Justify::Between => write!(f, "{}", "space-between"),
        }
    }
}

impl fmt::Display for BorderRadius {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} {} {} {}",
            self.top_left, self.top_right, self.bottom_right, self.bottom_left
        )
    }
}
