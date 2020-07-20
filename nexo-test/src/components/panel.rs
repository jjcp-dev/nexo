use nexo::component::Component;
use nexo::tree::{Node, NodeRef, Tree};

pub struct Panel;

impl Panel {
    pub fn new() -> Panel {
        Panel {}
    }
}

impl Component for Panel {
    fn render(&self, tree: &mut Tree, children: &[NodeRef]) -> NodeRef {
        let c = [tree.create(Node::Col, &[]), tree.create(Node::Col, &[])];
        tree.create(Node::Row, &c)
    }
}
