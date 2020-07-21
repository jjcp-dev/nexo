use super::color::Color;

pub enum Property<T> {
    Inherit,
    With(T),
}

pub struct Background {
    pub color: Property<Color>,
}

pub struct Style {
    background: Background,
}

pub struct StyleBuilder {
    style: Style,
}

impl StyleBuilder {
    pub fn new() -> StyleBuilder {
        StyleBuilder {
            style: Style {
                background: Background {
                    color: Property::Inherit,
                },
            },
        }
    }

    pub fn with_background_color(&mut self, color: Color) -> &mut self {
        self.background.color = color;
        self
    }
}
