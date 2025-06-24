use std::cell::RefCell;
use std::error;
use std::rc::Rc;

mod node;

use node::internal::InternalNode;
use node::leaf::LeafNode;
use node::BTreeNode;

type NodeRef = Rc<RefCell<BTreeNode>>;

pub struct BPlusTree {
    pub root_node: NodeRef,
    pub order: usize,
}

impl BPlusTree {
    pub fn new(order: usize) -> BPlusTree {
        BPlusTree {
            root_node: Rc::new(RefCell::new(BTreeNode::Leaf(LeafNode {
                keys: vec![],
                values: vec![],
                next: None,
            }))),
            order,
        }
    }

    pub fn insert_one<'a>(
        &mut self,
        key: i32,
        value: &'a str,
    ) -> Result<(i32, &'a str), Box<dyn error::Error>> {
        let split_result = self.root_node.borrow_mut().insert(key, value, self.order);
        if let Some(split_result) = split_result {
            let new_node = BTreeNode::Internal(InternalNode {
                keys: vec![split_result.key],
                children: vec![Rc::clone(&self.root_node), split_result.right],
            });
            self.root_node = Rc::new(RefCell::new(new_node));
        };
        Ok((key, value))
    }
}
