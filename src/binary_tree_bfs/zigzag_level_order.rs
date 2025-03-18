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
use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;
impl Solution {
    pub fn zigzag_level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        let mut result: Vec<Vec<i32>> = Vec::new();
        if root.is_none() {
            return result;
        }

        let mut is_forward = true;
        let mut queue = VecDeque::new();
        queue.push_back(root.unwrap());

        while !queue.is_empty() {
            let levelsize = queue.len();
            let mut val_queue: Vec<i32> = Vec::new();
            for _ in 0..levelsize {
                let node = queue.pop_front().unwrap();
                let node_borrow = node.borrow();
                val_queue.push(node_borrow.val);

                if let Some(left) = &node_borrow.left {
                    queue.push_back(left.clone());
                }
                if let Some(right) = &node_borrow.right {
                    queue.push_back(right.clone());
                }
            }

            if is_forward {
                result.push(val_queue);
                is_forward = false;
            } else {
                val_queue.reverse();
                result.push(val_queue);
                is_forward = true;
            }
        }
        result
    }
}

pub struct Solution;
