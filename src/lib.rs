use std::cell::RefCell;
use std::error;
use std::rc::Rc;

mod node;

use node::internal::InternalNode;
use node::leaf::LeafNode;
use node::BTreeNode;

type NodeRef<T> = Rc<RefCell<BTreeNode<T>>>;

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

    pub fn insert_one(&mut self, key: i32, value: T) -> Result<(i32, T), Box<dyn error::Error>> {
        let split_result = self
            .root_node
            .borrow_mut()
            .insert(key, value.clone(), self.order);
        if let Some(split_result) = split_result {
            let new_node = BTreeNode::Internal(InternalNode {
                keys: vec![split_result.key],
                children: vec![Rc::clone(&self.root_node), split_result.right],
            });
            self.root_node = Rc::new(RefCell::new(new_node));
        };
        Ok((key, value))
    }

    pub fn search(&self, key: i32) -> Option<T> {
        self.root_node.borrow_mut().search(key)
    }

    pub fn update(&self, key: i32, value: T) -> Result<(i32, T), ()> {
        self.root_node.borrow_mut().update(key, value)
    }
}
