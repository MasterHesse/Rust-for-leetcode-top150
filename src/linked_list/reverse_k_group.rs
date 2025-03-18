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
    pub fn reverse_k_group(head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
        let mut dummy = Box::new(ListNode::new(0));
        dummy.next = head;
        let mut prev = &mut dummy;

        loop {
            let mut node = prev.next.as_ref();
            for _ in 0..k {
                if node.is_none() {
                    return dummy.next;
                }
                node = node.unwrap().next.as_ref();
            }
            let mut curr = prev.next.take();
            let mut reversed: Option<Box<ListNode>> = None;

            for _ in 0..k {
                let mut temp = curr.unwrap();
                curr = temp.next.take();
                temp.next = reversed;
                reversed = Some(temp);
            }

            let mut tail = &mut reversed;
            while tail.as_mut().unwrap().next.is_some() {
                tail = &mut tail.as_mut().unwrap().next;
            }
            tail.as_mut().unwrap().next = curr;
            prev.next = reversed;

            for _ in 0..k {
                prev = prev.next.as_mut().unwrap();
            }
        }
    }
}

pub struct Solution;
