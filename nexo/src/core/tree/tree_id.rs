use std::sync::atomic::{AtomicU32, Ordering};

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct TreeId {
    id: u32,
}

static GLOBAL_TREE_ID: AtomicU32 = AtomicU32::new(1);

impl TreeId {
    pub fn new() -> Option<TreeId> {
        let id = GLOBAL_TREE_ID.fetch_add(1, Ordering::SeqCst);

        if id != 0 {
            Some(TreeId { id })
        } else {
            None
        }
    }
}
