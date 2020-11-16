#[derive(Debug)]
pub enum Event {
    Click { x: i16, y: i16 },
    MouseDown { x: i16, y: i16 },
}
