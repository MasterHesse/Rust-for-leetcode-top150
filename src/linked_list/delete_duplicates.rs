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
    pub fn delete_duplicates(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut dummy = Some(Box::new(ListNode { val: 0, next: head }));
        let mut prev = &mut dummy;

        while let Some(mut current) = prev.as_mut().and_then(|node| node.next.take()) {
            let mut has_duplicate = false;

            // Check if the current node has duplicates
            while let Some(next) = current.next.take() {
                if next.val == current.val {
                    has_duplicate = true;
                    current.next = next.next;
                } else {
                    current.next = Some(next);
                    break;
                }
            }

            if has_duplicate {
                // Skip the current node if it has duplicates
                prev.as_mut().unwrap().next = current.next;
            } else {
                // Otherwise, move the prev pointer forward
                prev.as_mut().unwrap().next = Some(current);
                prev = &mut prev.as_mut().unwrap().next;
            }
        }

        dummy.unwrap().next
    }
}

pub struct Solution;
