use super::tree::NodeRef;

#[derive(Debug)]
pub enum Event {
    Click { x: i16, y: i16, ui_node: NodeRef },
}
