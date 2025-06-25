use std::cell::RefCell;
use std::rc::Rc;

mod node;

use node::internal::InternalNode;
use node::leaf::LeafNode;
use node::BTreeNode;

type NodeRef<T> = Rc<RefCell<BTreeNode<T>>>;

#[derive(PartialEq)]
pub struct DoesNotExist;

#[derive(PartialEq)]
pub struct AlreadyExists;

pub struct BPlusTree<T: Clone> {
    pub root_node: NodeRef<T>,
    pub order: usize,
}

impl<T: Clone> BPlusTree<T> {
    pub fn new(order: usize) -> BPlusTree<T> {
        BPlusTree::<T> {
            root_node: Rc::new(RefCell::new(BTreeNode::Leaf(LeafNode {
                keys: vec![],
                values: vec![],
                next: None,
            }))),
            order,
        }
    }

    pub fn insert(&mut self, key: i32, value: T) -> Result<(i32, T), AlreadyExists> {
        let split_result = self
            .root_node
            .borrow_mut()
            .insert(key, value.clone(), self.order);
        match split_result {
            Ok(Some(split_result)) => {
                let new_node = BTreeNode::Internal(InternalNode {
                    keys: vec![split_result.key],
                    children: vec![Rc::clone(&self.root_node), split_result.right],
                });
                self.root_node = Rc::new(RefCell::new(new_node));
            }
            Ok(None) => {}
            Err(err) => return Err(err),
        };
        Ok((key, value))
    }

    pub fn search(&self, key: i32) -> Option<T> {
        self.root_node.borrow_mut().search(key)
    }

    pub fn update(&self, key: i32, value: T) -> Result<(i32, T), DoesNotExist> {
        self.root_node.borrow_mut().update(key, value)
    }
}
