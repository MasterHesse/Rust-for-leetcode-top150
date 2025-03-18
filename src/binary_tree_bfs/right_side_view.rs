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
    pub fn right_side_view(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut result = Vec::new();

        if root.is_none() {
            return result;
        }

        let mut queue = VecDeque::new();
        queue.push_back(root.unwrap());

        while !queue.is_empty() {
            let level_size = queue.len();
            for i in 0..level_size {
                let node = queue.pop_front().unwrap();
                let node_borrow = node.borrow();

                // 如果是当前层的最后一个节点，将其值加入结果
                if i == level_size - 1 {
                    result.push(node_borrow.val);
                }

                // 将左子节点加入队列
                if let Some(left) = &node_borrow.left {
                    queue.push_back(left.clone());
                }

                // 将右子节点加入队列
                if let Some(right) = &node_borrow.right {
                    queue.push_back(right.clone());
                }
            }
        }

        result
    }
}
pub struct Solution;
