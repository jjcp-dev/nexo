use crate::length::Length;

#[derive(Debug)]
pub struct Padding {
    pub top: Length,
    pub right: Length,
    pub bottom: Length,
    pub left: Length,
}
