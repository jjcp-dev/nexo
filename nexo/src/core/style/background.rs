use super::{Color, Property};

#[derive(Debug, Clone, PartialEq)]
pub struct Background {
    pub color: Property<Color>,
    // FIXME: This should be a list of some kind of `Image` type.
    pub image: String,
}

// enum Image {
//     Url(String),
//     LinearGradient {
//         angle: Angle,
//         stops: Vec<GradientStop>,
//     },
// }

impl Background {
    pub fn new() -> Background {
        Background {
            color: Property::Inherit,
            image: "".into(),
        }
    }
}
