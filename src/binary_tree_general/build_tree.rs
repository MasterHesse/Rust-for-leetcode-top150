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
use std::collections::HashMap;
use std::rc::Rc;

// 定义树的类型
type Tree = Option<Rc<RefCell<TreeNode>>>;

impl Solution {
    pub fn build_tree(pre: Vec<i32>, ino: Vec<i32>) -> Tree {
        let mut mp = HashMap::new();

        for (i, &val) in ino.iter().enumerate() {
            mp.insert(val, i);
        }

        let n = pre.len();
        return Self::build(&pre, &ino, &mp, 0, n, 0, n);
    }

    fn build(
        pre: &Vec<i32>,
        ino: &Vec<i32>,
        mp: &HashMap<i32, usize>,
        pre_start: usize,
        pre_end: usize,
        ino_start: usize,
        ino_end: usize,
    ) -> Tree {
        if pre_start >= pre_end {
            return None;
        }

        let val = pre[pre_start]; // 取出当前子树的根节点
        let mut root = TreeNode::new(val);

        let idx = mp[&val]; // 获取根节点在 inorder 中的位置
        let left = idx - ino_start; // 左子树的长度

        // 递归构建左子树
        root.left = Self::build(
            pre,
            ino,
            mp,
            pre_start + 1,
            pre_start + left + 1,
            ino_start,
            idx,
        );
        // 递归构建右子树
        root.right = Self::build(
            pre,
            ino,
            mp,
            pre_start + left + 1,
            pre_end,
            idx + 1,
            ino_end,
        );

        Some(Rc::new(RefCell::new(root)))
    }
}

pub struct Solution;
