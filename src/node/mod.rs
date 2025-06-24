use crate::NodeRef;

pub mod internal;
pub mod leaf;

use internal::InternalNode;
use leaf::LeafNode;

pub enum BTreeNode {
    Internal(InternalNode),
    Leaf(LeafNode),
}

impl BTreeNode {
    pub fn insert(&mut self, key: i32, value: &str, order: usize) -> Option<SplitResult> {
        match self {
            BTreeNode::Internal(node) => node.insert(key, value, order),
            BTreeNode::Leaf(node) => node.insert(key, value, order),
        }
    }
}

pub struct SplitResult {
    pub key: i32,
    pub right: NodeRef,
}
