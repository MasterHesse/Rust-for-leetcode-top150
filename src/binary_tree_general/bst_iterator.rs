use std::cell::RefCell;
use std::rc::Rc;

use crate::stack;

// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}
struct BSTIterator {
    stack: Vec<Rc<RefCell<TreeNode>>>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl BSTIterator {
    fn new(root: Option<Rc<RefCell<TreeNode>>>) -> Self {
        let mut stack = Vec::new();
        let mut current = root;

        while let Some(node) = current {
            stack.push(Rc::clone(&node));
            current = node.borrow().left.clone();
        }

        BSTIterator { stack }
    }

    fn next(&mut self) -> i32 {
        let node = self.stack.pop().unwrap();
        let val = node.borrow().val;

        let mut current = node.borrow().right.clone();
        while let Some(right_node) = current {
            self.stack.push(Rc::clone(&right_node));
            current = right_node.borrow().left.clone();
        }

        val
    }

    fn has_next(&self) -> bool {
        !self.stack.is_empty()
    }
}
