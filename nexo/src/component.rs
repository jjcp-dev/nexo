use super::event::Event;
use super::tree::{NodeRef, Tree};

pub trait Component {
    fn event(&mut self, event: Event) {}
    fn render(&self, tree: &mut Tree, children: &[NodeRef]) -> NodeRef;
}
