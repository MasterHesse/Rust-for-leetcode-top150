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
    pub fn kth_smallest(root: Option<Rc<RefCell<TreeNode>>>, k: i32) -> i32 {
        let mut result = 0;
        let mut count = 0;
        fn inorder(node: Option<Rc<RefCell<TreeNode>>>, k: i32, count: &mut i32, result: &mut i32) {
            if let Some(n) = node {
                let n = n.borrow();
                inorder(n.left.clone(), k, count, result);

                *count += 1;
                if *count == k {
                    *result = n.val;
                    return;
                }

                inorder(n.right.clone(), k, count, result);
            }
        }

        inorder(root, k, &mut count, &mut result);
        result
    }
}

pub struct Solution;
