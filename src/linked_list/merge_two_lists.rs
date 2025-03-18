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
    pub fn merge_two_lists(
        list1: Option<Box<ListNode>>,
        list2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut l1 = list1;
        let mut l2 = list2;
        let mut dummy = ListNode::new(0);
        let mut current = &mut dummy;

        while let (Some(n1), Some(n2)) = (&l1, &l2) {
            if n1.val <= n2.val {
                current.next = l1;
                current = current.next.as_mut().unwrap();
                l1 = current.next.take();
            } else {
                current.next = l2;
                current = current.next.as_mut().unwrap();
                l2 = current.next.take();
            }
        }

        current.next = l1.or(l2);
        dummy.next
    }
}

pub struct Solution;
