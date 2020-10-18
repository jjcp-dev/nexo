use std::vec::Vec;

use super::index::Index;
use super::node_ref::NodeRef;
use super::tree_id::TreeId;

use crate::layout::Layout;
use crate::node::Node;
use crate::style::Style;

pub struct Tree {
    id: TreeId,
    nodes: Vec<Node>,
    links: Vec<Link>,
}

struct Link {
    parent: Index,
    first_child: Index,
    last_child: Index,
    previous_sibling: Index,
    next_sibling: Index,
}

impl Tree {
    pub fn new() -> Tree {
        let root = Node::Element {
            layout: Layout::Column,
            style: Style::default(),
        };

        Tree {
            id: TreeId::new().unwrap(),
            nodes: vec![root],
            links: vec![Link {
                parent: Index::null(),
                first_child: Index::null(),
                last_child: Index::null(),
                previous_sibling: Index::null(),
                next_sibling: Index::null(),
            }],
        }
    }

    pub fn with_capacity(capacity: usize) -> Tree {
        let mut nodes: Vec<Node> = Vec::with_capacity(capacity);
        let mut links: Vec<Link> = Vec::with_capacity(capacity);

        let root = Node::Element {
            layout: Layout::Column,
            style: Style::default(),
        };

        nodes.push(root);
        links.push(Link {
            parent: Index::null(),
            first_child: Index::null(),
            last_child: Index::null(),
            previous_sibling: Index::null(),
            next_sibling: Index::null(),
        });

        Tree {
            id: TreeId::new().unwrap(),
            nodes: nodes,
            links: links,
        }
    }

    #[inline]
    pub fn root(&self) -> NodeRef {
        NodeRef::new(Index::zero(), self.id)
    }

    pub fn create(&mut self, parent: NodeRef, node: Node) -> NodeRef {
        // FIXME: Add error messages.
        assert!(parent.tree_id() == self.id);
        assert!(!parent.index().is_null());

        // FIXME: This function should look for empty slots in the
        //        nodes and links vectors.

        let node_index = Index::new(self.nodes.len()).unwrap();

        let last_child = self.links[parent.value()].last_child;

        self.links[parent.value()].last_child = node_index;

        if last_child.is_null() {
            self.links[parent.value()].first_child = node_index;
        } else {
            self.links[last_child.value()].next_sibling = node_index;
        }

        self.nodes.push(node);
        self.links.push(Link {
            parent: parent.index(),
            first_child: Index::null(),
            last_child: Index::null(),
            previous_sibling: last_child,
            next_sibling: Index::null(),
        });

        NodeRef::new(node_index, self.id)
    }

    #[inline]
    pub fn children(&self, parent: NodeRef) -> ChildrenIter {
        assert!(self.id == parent.tree_id());

        ChildrenIter {
            tree: self,
            node: self.links[parent.value()].first_child,
        }
    }

    #[inline]
    pub fn get(&self, node: NodeRef) -> &Node {
        assert!(self.id == node.tree_id());

        &self.nodes[node.value()]
    }
}

pub struct ChildrenIter<'a> {
    tree: &'a Tree,
    node: Index,
}

impl<'a> Iterator for ChildrenIter<'a> {
    type Item = (&'a Node, NodeRef);

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if !self.node.is_null() {
            let result = Some((
                &self.tree.nodes[self.node.value()],
                NodeRef::new(self.node, self.tree.id),
            ));
            self.node = self.tree.links[self.node.value()].next_sibling;
            result
        } else {
            None
        }
    }
}
