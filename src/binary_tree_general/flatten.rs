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
    pub fn flatten(root: &mut Option<Rc<RefCell<TreeNode>>>) {
        if root.is_none() {
            return;
        }

        // 递归展开左子树和右子树
        let mut node = root.clone();
        while let Some(current) = node {
            let mut current_borrow = current.borrow_mut();

            // 如果左子树不为空，将其插入到当前节点和右子树之间
            if let Some(left) = current_borrow.left.take() {
                // 找到左子树的最后一个节点
                let mut left_tail = left.clone();
                loop {
                    let next = left_tail.borrow().right.clone();
                    if let Some(right) = next {
                        left_tail = right;
                    } else {
                        break;
                    }
                }

                // 将左子树的最后一个节点的 right 指向当前节点的右子树
                left_tail.borrow_mut().right = current_borrow.right.take();
                // 将当前节点的 right 指向左子树
                current_borrow.right = Some(left);
            }

            // 移动到右子树
            node = current_borrow.right.clone();
        }
    }
}

pub struct Solution;
