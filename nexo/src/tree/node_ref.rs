use std::fmt;

use super::index::Index;
use super::tree_id::TreeId;

#[derive(Clone, Copy, Eq, PartialEq)]
pub struct NodeRef {
    index: Index,
    tree_id: TreeId,
}

impl NodeRef {
    #[inline]
    pub(crate) fn new(index: Index, tree_id: TreeId) -> NodeRef {
        NodeRef { index, tree_id }
    }

    #[inline]
    pub(crate) fn index(&self) -> Index {
        self.index
    }

    #[inline]
    pub(crate) fn tree_id(&self) -> TreeId {
        self.tree_id
    }

    #[inline]
    pub fn value(&self) -> usize {
        self.index.value()
    }
}

impl fmt::Debug for NodeRef {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if !self.index.is_null() {
            f.debug_tuple("NodeRef").field(&self.index.value()).finish()
        } else {
            f.debug_tuple("NodeRef").field(&(-1)).finish()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_returns_a_node_ref() {
        let a = NodeRef::new(Index::new(10).unwrap(), TreeId::new().unwrap());

        assert_eq!(a.value(), 10);
    }

    #[test]
    fn value_given_a_node_ref_returns_the_internal_numeric_index() {
        let a = NodeRef::new(Index::new(5).unwrap(), TreeId::new().unwrap());

        assert_eq!(a.value(), 5);
    }

    #[test]
    fn index_given_a_node_ref_returns_the_internal_index() {
        let a = NodeRef::new(Index::new(5).unwrap(), TreeId::new().unwrap());

        assert_eq!(a.index(), Index::new(5).unwrap());
    }

    #[test]
    fn debug_fmt() {
        assert_eq!(
            format!(
                "{:?}",
                NodeRef::new(Index::new(5).unwrap(), TreeId::new().unwrap())
            ),
            "NodeRef(5)"
        );
        assert_eq!(
            format!("{:?}", NodeRef::new(Index::null(), TreeId::new().unwrap())),
            "NodeRef(-1)"
        );
    }
}
