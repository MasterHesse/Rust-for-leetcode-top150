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
    pub fn is_symmetric(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        if root.is_none() {
            return true;
        }
        let mut queue: VecDeque<(Option<Rc<RefCell<TreeNode>>>, Option<Rc<RefCell<TreeNode>>>)> =
            VecDeque::new();
        if let Some(node) = root {
            let node_ref = node.borrow();
            let left = node_ref.left.clone();
            let right = node_ref.right.clone();
            queue.push_back((left, right));

            while let Some((node1, node2)) = queue.pop_front() {
                match (node1, node2) {
                    (None, None) => continue,

                    (None, Some(_)) | (Some(_), None) => return false,

                    (Some(n1), Some(n2)) => {
                        let n1 = n1.borrow();
                        let n2 = n2.borrow();

                        if n1.val != n2.val {
                            return false;
                        }

                        queue.push_back((n1.left.clone(), n2.right.clone()));
                        queue.push_back((n1.right.clone(), n2.left.clone()));
                    }
                }
            }
        }
        true
    }
}

pub struct Solution;
