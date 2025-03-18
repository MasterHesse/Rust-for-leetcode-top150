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
    pub fn get_minimum_difference(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut min_diff = i32::MAX;
        let mut prev: Option<i32> = None;

        fn dfs(node: Option<Rc<RefCell<TreeNode>>>, prev: &mut Option<i32>, min_diff: &mut i32) {
            if let Some(n) = node {
                let n = n.borrow();
                dfs(n.left.clone(), prev, min_diff);

                if let Some(p) = prev {
                    *min_diff = (*min_diff).min(n.val - *p);
                }
                *prev = Some(n.val);

                dfs(n.right.clone(), prev, min_diff);
            }
        }

        dfs(root, &mut prev, &mut min_diff);
        min_diff
    }
}

pub struct Solution;
