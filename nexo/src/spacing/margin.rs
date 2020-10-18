use crate::length::Length;

#[derive(Debug, Clone)]
pub struct Margin {
    pub top: Length,
    pub right: Length,
    pub bottom: Length,
    pub left: Length,
}

impl Margin {
    pub fn new(len: Length) -> Margin {
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
}
