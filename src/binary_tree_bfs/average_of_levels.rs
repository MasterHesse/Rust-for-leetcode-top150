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
    pub fn average_of_levels(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<f64> {
        let mut result: Vec<f64> = Vec::new();
        let mut queue = VecDeque::new();

        if root.is_none() {
            return result;
        }

        queue.push_back(root.unwrap());

        while !queue.is_empty() {
            let levelsize = queue.len();
            let mut _levelsum: f64 = 0.0;

            for _ in 0..levelsize {
                let node = queue.pop_front().unwrap();
                let node_borrow = node.borrow();
                _levelsum += node_borrow.val as f64;

                // 将左子节点加入队列
                if let Some(left) = &node_borrow.left {
                    queue.push_back(left.clone());
                }

                // 将右子节点加入队列
                if let Some(right) = &node_borrow.right {
                    queue.push_back(right.clone());
                }
            }

            result.push(_levelsum / (levelsize as f64));
        }
        result
    }
}

pub struct Solution;
