use super::Align;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Layout {
    Row { main_axis: Align, cross_axis: Align },
    Column { main_axis: Align, cross_axis: Align },
}
