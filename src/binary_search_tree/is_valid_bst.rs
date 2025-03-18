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
    pub fn is_valid_bst(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        fn dfs(node: &Option<Rc<RefCell<TreeNode>>>, min: Option<i32>, max: Option<i32>) -> bool {
            match node {
                Some(n) => {
                    let n_borrow = n.borrow();
                    // 检查当前节点是否在合法范围内
                    if min.is_some() && n_borrow.val <= min.unwrap() {
                        return false;
                    }
                    if max.is_some() && n_borrow.val >= max.unwrap() {
                        return false;
                    }
                    // 递归检查左子树和右子树
                    dfs(&n_borrow.left, min, Some(n_borrow.val))
                        && dfs(&n_borrow.right, Some(n_borrow.val), max)
                }
                None => true, // 空节点是合法的
            }
        }
        dfs(&root, None, None)
    }
}
pub struct Solution;
