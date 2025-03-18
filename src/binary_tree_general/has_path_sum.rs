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
use std::rc::Rc;
impl Solution {
    pub fn has_path_sum(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> bool {
        match root {
            Some(node) => {
                let node_borrow = node.borrow();
                let new_target = target_sum - node_borrow.val;

                if node_borrow.left.is_none() && node_borrow.right.is_none() {
                    return new_target == 0;
                }

                Self::has_path_sum(node_borrow.left.clone(), new_target)
                    || Self::has_path_sum(node_borrow.right.clone(), new_target)
            }
            None => false,
        }
    }
}

pub struct Solution;
