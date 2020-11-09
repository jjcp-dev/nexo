#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Length {
    Auto,
    Dots(i32),
    // Percent(f32),
}
