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
    pub fn sum_numbers(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        fn dfs(node: &Option<Rc<RefCell<TreeNode>>>, current_sum: i32) -> i32 {
            match node {
                Some(n) => {
                    let n_borrow = n.borrow();
                    let current_sum = current_sum * 10 + n_borrow.val;

                    if n_borrow.left.is_none() && n_borrow.right.is_none() {
                        return current_sum;
                    }

                    dfs(&n_borrow.left, current_sum) + dfs(&n_borrow.right, current_sum)
                }
                None => 0,
            }
        }
        dfs(&root, 0)
    }
}

pub struct Solution;
