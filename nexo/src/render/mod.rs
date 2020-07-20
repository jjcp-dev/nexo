pub mod web;

// use crate::component::Component;
// use crate::tree::{Node, NodeRef, Tree};
//
// pub struct WebRender {
//     tree: Tree,
// }
//
// impl WebRender {
//     pub fn new() -> WebRender {
//         WebRender {
//             tree: Tree::with_capacity(100),
//         }
//     }
//
//     pub fn render_node(&self, node: NodeRef, depth: usize) {
//         let depth = depth * 2;
//         match self.tree.get_node(node) {
//             Node::Row => {
//                 println!("{:depth$}<div class={:?}>", "", "flex-row", depth = depth);
//                 for (_, r) in self.tree.children(node) {
//                     self.render_node(r, depth + 1);
//                 }
//                 println!("{:depth$}</div>", "", depth = depth);
//             }
//
//             Node::Col => {
//                 println!("{:depth$}<div class={:?}>", "", "flex-col", depth = depth);
//                 for (_, r) in self.tree.children(node) {
//                     self.render_node(r, depth + 1);
//                 }
//                 println!("{:depth$}</div>", "", depth = depth);
//             }
//         }
//     }
//
//     pub fn render<T: Component>(&mut self, component: T) {
//         let root = component.render(&mut self.tree, None);
//
//         self.render_node(root, 0);
//     }
// }
