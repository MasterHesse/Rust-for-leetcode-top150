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
    pub fn reverse_between(
        head: Option<Box<ListNode>>,
        left: i32,
        right: i32,
    ) -> Option<Box<ListNode>> {
        // 1. 创建哑节点 dummy
        let mut dummy = Box::new(ListNode::new(0));
        dummy.next = head;
        let mut prev = &mut dummy;

        // 2. 移动 prev 到 left 前一个位置
        for _ in 0..(left - 1) {
            prev = prev.next.as_mut().unwrap();
        }

        // 3. 反转 [left, right] 区间
        // curr 指向反转区间的第一个节点，reversed 保存反转后的链表头
        let mut curr = prev.next.take();
        let mut reversed: Option<Box<ListNode>> = None;
        for _ in 0..(right - left + 1) {
            // 取出当前节点
            let mut temp = curr.unwrap();
            curr = temp.next.take(); // temp 与原链表断开
                                     // 头插法，将 temp 插入到 reversed 的头部
            temp.next = reversed;
            reversed = Some(temp);
        }

        // 4. 连接反转后的部分与后续部分
        // 找到反转后子链表的尾节点（即原 left 节点，现在在链表末尾）
        {
            let mut tail = &mut reversed;
            while tail.as_mut().unwrap().next.is_some() {
                tail = &mut tail.as_mut().unwrap().next;
            }
            // 连接尾节点和 curr（right+1 后续部分）
            tail.as_mut().unwrap().next = curr;
        }
        // 连接 prev 与反转后的子链表
        prev.next = reversed;

        dummy.next
    }
}

pub struct Solution;
