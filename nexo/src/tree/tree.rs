use std::vec::Vec;

use super::super::listeners::Listeners;
use super::index::Index;
use super::node_ref::NodeRef;
use super::tree_id::TreeId;

use crate::node::Node;

pub struct Tree {
    id: TreeId,
    nodes: Vec<Node>,
    links: Vec<Link>,
    listeners: Vec<Listeners>,
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
        Tree {
            id: TreeId::new().unwrap(),
            nodes: Vec::new(),
            links: Vec::new(),
            listeners: Vec::new(),
        }
    }

    pub fn with_capacity(capacity: usize) -> Tree {
        Tree {
            id: TreeId::new().unwrap(),
            nodes: Vec::with_capacity(capacity),
            links: Vec::with_capacity(capacity),
            listeners: Vec::with_capacity(capacity),
        }
    }

    pub fn create(&mut self, node: Node, children: &[NodeRef]) -> NodeRef {
        // match node {
        //     _ => {
        // TODO: Look for an empty slot to put the new node in.
        self.nodes.push(node);
        self.links.push(Link {
            parent: Index::null(),
            first_child: Index::null(),
            last_child: Index::null(),
            previous_sibling: Index::null(),
            next_sibling: Index::null(),
        });

        let index = Index::new(self.nodes.len() - 1).unwrap();

        self.connect_children(index, children);

        NodeRef::new(index, self.id)
        // }
        // }
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

    #[inline]
    fn connect_children(&mut self, parent: Index, children: &[NodeRef]) {
        debug_assert!(!parent.is_null());

        if !children.is_empty() {
            // TODO: Verify the tree_id of the children

            {
                let last_child = self.links[parent.value()].last_child;

                self.links[children[0].value()].previous_sibling = last_child;
                if !last_child.is_null() {
                    self.links[last_child.value()].next_sibling = children[0].index();
                }

                let parent_link = &mut self.links[parent.value()];

                if parent_link.first_child.is_null() {
                    parent_link.first_child = children[0].index();
                }

                parent_link.last_child = children.last().unwrap().index();
            }

            for window in children.windows(2) {
                let first = window[0];
                let second = window[1];

                self.links[first.value()].parent = parent;
                self.links[first.value()].next_sibling = second.index();
                self.links[second.value()].previous_sibling = first.index();
            }
        }
    }
}

pub struct ChildrenIter<'a> {
    tree: &'a Tree,
    node: Index,
}

impl<'a> Iterator for ChildrenIter<'a> {
    type Item = (&'a Node, NodeRef);

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
