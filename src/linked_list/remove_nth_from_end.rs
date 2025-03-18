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
    pub fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
        let mut dummy = Box::new(ListNode::new(0));
        dummy.next = head;
        let mut fast = &mut dummy as *mut Box<ListNode>;
        let mut slow = &mut dummy as *mut Box<ListNode>;

        for _ in 0..n {
            unsafe {
                if let Some(ref mut next) = (*fast).next {
                    fast = next;
                }
            }
        }

        unsafe {
            while let Some(ref mut next) = (*fast).next {
                fast = next;
                slow = (*slow).next.as_mut().unwrap();
            }
        }

        unsafe {
            let to_delete = (*slow).next.take();
            if let Some(mut node) = to_delete {
                (*slow).next = node.next.take();
            }
        }

        dummy.next
    }
}

pub struct Solution;
