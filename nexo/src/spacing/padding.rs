use crate::length::Length;

#[derive(Debug, Clone)]
pub struct Padding {
    pub top: Length,
    pub right: Length,
    pub bottom: Length,
    pub left: Length,
}
