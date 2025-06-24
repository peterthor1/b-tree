use std::cell::RefCell;
use std::rc::Rc;

use super::BTreeNode;
use super::SplitResult;
use crate::NodeRef;

pub struct InternalNode<T: Clone> {
    pub keys: Vec<i32>,
    pub children: Vec<NodeRef<T>>,
}

impl<T: Clone> InternalNode<T> {
    fn split(&mut self) -> Option<SplitResult<T>> {
        let split_index = self.keys.len() / 2;
        let right_keys = self.keys.split_off(split_index + 1);
        let right_children = self.children.split_off(split_index + 1);

        let promoted_key = self.keys.pop();

        let new_node = Rc::new(RefCell::new(BTreeNode::Internal(InternalNode {
            keys: right_keys.clone(),
            children: right_children,
        })));

        Some(SplitResult {
            key: promoted_key?,
            right: new_node,
        })
    }

    pub fn insert(&mut self, key: i32, value: T, order: usize) -> Option<SplitResult<T>> {
        let split_result = match self.keys.binary_search(&key) {
            Ok(pos) => self.children[pos].borrow_mut().insert(key, value, order),
            Err(pos) => self.children[pos].borrow_mut().insert(key, value, order),
        };
        if let Some(split_result) = split_result {
            if let Err(pos) = self.keys.binary_search(&split_result.key) {
                self.keys.insert(pos, split_result.key);
                self.children.insert(pos + 1, split_result.right);
            };
        };
        if self.keys.len() > order - 1 {
            self.split()
        } else {
            None
        }
    }

    pub fn search(&self, key: i32) -> Option<T> {
        match self.keys.binary_search(&key) {
            Ok(pos) => self.children[pos + 1].borrow_mut().search(key),
            Err(pos) => self.children[pos].borrow_mut().search(key),
        }
    }
}
