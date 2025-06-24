use crate::NodeRef;

pub mod internal;
pub mod leaf;

use internal::InternalNode;
use leaf::LeafNode;

pub enum BTreeNode<T: Clone> {
    Internal(InternalNode<T>),
    Leaf(LeafNode<T>),
}

impl<T: Clone> BTreeNode<T> {
    pub fn insert(&mut self, key: i32, value: T, order: usize) -> Option<SplitResult<T>> {
        match self {
            BTreeNode::Internal(node) => node.insert(key, value, order),
            BTreeNode::Leaf(node) => node.insert(key, value, order),
        }
    }

    pub fn search(&mut self, key: i32) -> Option<T> {
        match self {
            BTreeNode::Internal(node) => node.search(key),
            BTreeNode::Leaf(node) => node.search(key),
        }
    }

    pub fn update(&mut self, key: i32, value: T) -> Result<(i32, T), ()> {
        match self {
            BTreeNode::Internal(node) => node.update(key, value),
            BTreeNode::Leaf(node) => node.update(key, value),
        }
    }
}

pub struct SplitResult<T: Clone> {
    pub key: i32,
    pub right: NodeRef<T>,
}
