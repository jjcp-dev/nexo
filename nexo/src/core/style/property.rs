#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Property<T> {
    Inherit,
    With(T),
}
