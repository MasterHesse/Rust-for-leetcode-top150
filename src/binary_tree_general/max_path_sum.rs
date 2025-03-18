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
use std::i32;
use std::rc::Rc;
impl Solution {
    pub fn max_path_sum(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut max_sum = i32::MIN;

        fn dfs(node: &Option<Rc<RefCell<TreeNode>>>, max_sum: &mut i32) -> i32 {
            match node {
                None => 0,
                Some(n) => {
                    let node = n.borrow();

                    let left_gain = dfs(&node.left, max_sum).max(0);
                    let right_gain = dfs(&node.right, max_sum).max(0);

                    let current_sum = node.val + left_gain + right_gain;
                    *max_sum = (*max_sum).max(current_sum);

                    node.val + left_gain.max(right_gain)
                }
            }
        }
        dfs(&root, &mut max_sum);
        max_sum
    }
}
pub struct Solution;
