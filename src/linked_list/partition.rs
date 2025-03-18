// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

impl Solution {
    pub fn partition(head: Option<Box<ListNode>>, x: i32) -> Option<Box<ListNode>> {
        let mut left_dummy = Box::new(ListNode::new(0)); // 左分区哑节点
        let mut right_dummy = Box::new(ListNode::new(0)); // 右分区哑节点
        let mut left_tail = &mut left_dummy; // 左分区尾节点
        let mut right_tail = &mut right_dummy; // 右分区尾节点
        let mut cursor = head; // 遍历链表的游标

        // 遍历链表
        while let Some(mut node) = cursor {
            cursor = node.next.take(); // 断开当前节点与后续节点的连接
            if node.val < x {
                // 如果节点值小于 x，放入左分区
                left_tail.next = Some(node);
                left_tail = left_tail.next.as_mut().unwrap();
            } else {
                // 否则，放入右分区
                right_tail.next = Some(node);
                right_tail = right_tail.next.as_mut().unwrap();
            }
        }

        // 将左分区的尾节点连接到右分区的头节点
        left_tail.next = right_dummy.next;
        // 返回左分区的头节点（跳过哑节点）
        left_dummy.next
    }
}

pub struct Solution;
