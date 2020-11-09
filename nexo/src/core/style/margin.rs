use crate::core::length::Length;

#[derive(Debug, Clone, PartialEq)]
pub struct Margin {
    pub top: Length,
    pub right: Length,
    pub bottom: Length,
    pub left: Length,
}

impl Margin {
    pub fn new() -> Margin {
        Margin::all(Length::Dots(0))
    }

    pub fn all(len: Length) -> Margin {
        Margin {
            top: len,
            right: len,
            bottom: len,
            left: len,
        }
    }

    #[inline]
    pub fn top(len: Length) -> Margin {
        Margin {
            top: len,
            right: Length::Dots(0),
            bottom: Length::Dots(0),
            left: Length::Dots(0),
        }
    }

    #[inline]
    pub fn right(len: Length) -> Margin {
        Margin {
            top: Length::Dots(0),
            right: len,
            bottom: Length::Dots(0),
            left: Length::Dots(0),
        }
    }

    #[inline]
    pub fn bottom(len: Length) -> Margin {
        Margin {
            top: Length::Dots(0),
            right: Length::Dots(0),
            bottom: len,
            left: Length::Dots(0),
        }
    }

    #[inline]
    pub fn left(len: Length) -> Margin {
        Margin {
            top: Length::Dots(0),
            right: Length::Dots(0),
            bottom: Length::Dots(0),
            left: len,
        }
    }

    #[inline]
    pub fn horizontal(len: Length) -> Margin {
        Margin {
            top: Length::Dots(0),
            right: len,
            bottom: Length::Dots(0),
            left: len,
        }
    }

    #[inline]
    pub fn vertical(len: Length) -> Margin {
        Margin {
            top: len,
            right: Length::Dots(0),
            bottom: len,
            left: Length::Dots(0),
        }
    }
}
