use std::cell::RefCell;
use std::rc::Rc;

use super::BTreeNode;
use super::SplitResult;
use crate::NodeRef;

pub struct LeafNode<T: Clone> {
    pub keys: Vec<i32>,
    pub values: Vec<T>,
    pub next: Option<NodeRef<T>>,
}

impl<T: Clone> LeafNode<T> {
    fn split(&mut self) -> Option<SplitResult<T>> {
        let split_index = self.keys.len() / 2;
        let right_keys = self.keys.split_off(split_index);
        let right_values = self.values.split_off(split_index);

        let new_node = Rc::new(RefCell::new(BTreeNode::Leaf(LeafNode {
            keys: right_keys.clone(),
            values: right_values,
            next: self.next.clone(),
        })));

        self.next = Some(Rc::clone(&new_node));
        Some(SplitResult {
            key: right_keys[0],
            right: new_node,
        })
    }

    pub fn insert(&mut self, key: i32, value: T, order: usize) -> Option<SplitResult<T>> {
        match self.keys.binary_search(&key) {
            Ok(_) => {}
            Err(pos) => {
                self.keys.insert(pos, key);
                self.values.insert(pos, value);
            }
        };
        if self.keys.len() > order - 1 {
            self.split()
        } else {
            None
        }
    }

    pub fn search(&self, key: i32) -> Option<T> {
        match self.keys.binary_search(&key) {
            Ok(pos) => Some(self.values[pos].clone()),
            Err(_) => None,
        }
    }
}
