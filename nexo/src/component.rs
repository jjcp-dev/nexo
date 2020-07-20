use super::tree::{NodeRef, Tree};

pub trait Component {
    fn render(&self, tree: &mut Tree, children: &[NodeRef]) -> NodeRef;
}
