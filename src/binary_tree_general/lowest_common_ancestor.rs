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
    pub fn lowest_common_ancestor(
        root: Option<Rc<RefCell<TreeNode>>>,
        p: Option<Rc<RefCell<TreeNode>>>,
        q: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        fn dfs(
            node: Option<Rc<RefCell<TreeNode>>>,
            p_val: i32,
            q_val: i32,
        ) -> Option<Rc<RefCell<TreeNode>>> {
            if let Some(n) = node {
                let n_ref = n.borrow();

                if n_ref.val == p_val || n_ref.val == q_val {
                    return Some(Rc::clone(&n));
                }

                let left = dfs(n_ref.left.clone(), p_val, q_val);
                let right = dfs(n_ref.right.clone(), p_val, q_val);

                if left.is_some() && right.is_some() {
                    return Some(Rc::clone(&n));
                }

                left.or(right)
            } else {
                None
            }
        }

        let p_val = p.as_ref().unwrap().borrow().val;
        let q_val = q.as_ref().unwrap().borrow().val;

        dfs(root, p_val, q_val)
    }
}

pub struct Solution;
