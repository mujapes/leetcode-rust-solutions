// Definition for singly-linked list.
// #[derive(PartialEq, Eq, Clone, Debug)]
// pub struct ListNode {
//   pub val: i32,
//   pub next: Option<Box<ListNode>>
// }
// 
// impl ListNode {
//   #[inline]
//   fn new(val: i32) -> Self {
//     ListNode {
//       next: None,
//       val
//     }
//   }
// }
use std::iter::successors;
impl Solution {
    pub fn rotate_right(mut head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
        let size = successors(head.as_ref(), |node| node.next.as_ref())
            .count() as i32;
        if size == 0 || k % size == 0 {return head}
        // Distance to the end of the rotated array
        let dist = size - (k % size);
        let mut cur = head.as_mut().unwrap();
        for _ in 0..dist-1 {cur = cur.next.as_mut().unwrap()}
        let mut res = cur.next.take();
        cur = res.as_mut().unwrap();
        while cur.next.is_some() {cur = cur.next.as_mut().unwrap();}
        cur.next = head;
        res
    }
}

// Runtime: 0 ms, Beats 100.00%
// Memory: 2.12 MB, Beats 59.09%
