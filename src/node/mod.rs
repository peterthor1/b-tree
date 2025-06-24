use crate::NodeRef;

pub mod internal;
pub mod leaf;

use internal::InternalNode;
use leaf::LeafNode;

pub enum BTreeNode<T> {
    Internal(InternalNode<T>),
    Leaf(LeafNode<T>),
}

impl<T> BTreeNode<T> {
    pub fn insert(&mut self, key: i32, value: T, order: usize) -> Option<SplitResult<T>> {
        match self {
            BTreeNode::Internal(node) => node.insert(key, value, order),
            BTreeNode::Leaf(node) => node.insert(key, value, order),
        }
    }
}

pub struct SplitResult<T> {
    pub key: i32,
    pub right: NodeRef<T>,
}
