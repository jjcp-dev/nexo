#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Align {
    Start,
    Center,
    End,
    Stretch,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Layout {
    Row { main_axis: Align, cross_axis: Align },
    Column { main_axis: Align, cross_axis: Align },
}
