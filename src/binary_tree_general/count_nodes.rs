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
    pub fn count_nodes(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if root.is_none() {
            return 0;
        }

        let mut count = 0;
        let mut queue = VecDeque::new();
        queue.push_back(root);

        while !queue.is_empty() {
            let levelsize = queue.len();
            for _ in 0..levelsize {
                if let Some(Some(node)) = queue.pop_front() {
                    count += 1;
                    let n = node.borrow();

                    if n.left.is_some() {
                        queue.push_back(n.left.clone());
                    }
                    if n.right.is_some() {
                        queue.push_back(n.right.clone());
                    }
                }
            }
        }

        count
    }
}

pub struct Solution;
