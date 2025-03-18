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
use std::iter;
use std::rc::Rc;
impl Solution {
    pub fn build_tree(inorder: Vec<i32>, postorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        Self::build_tree_helper(&inorder, &postorder)
    }
    fn build_tree_helper(inorder: &[i32], postorder: &[i32]) -> Option<Rc<RefCell<TreeNode>>> {
        if inorder.is_empty() {
            return None;
        }

        let root_val = postorder[postorder.len() - 1];
        let root = Rc::new(RefCell::new(TreeNode::new(root_val)));

        let mid = inorder.iter().position(|&x| x == root_val).unwrap();

        root.borrow_mut().left = Self::build_tree_helper(&inorder[..mid], &postorder[..mid]);
        root.borrow_mut().right =
            Self::build_tree_helper(&inorder[mid + 1..], &postorder[mid..postorder.len() - 1]);

        Some(root)
    }
}
pub struct Solution;
