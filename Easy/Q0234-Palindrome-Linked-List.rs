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
use std::ptr;

impl Solution {
    pub fn is_palindrome(head: Option<Box<ListNode>>) -> bool {
        let mut front = &head;
        let mut done = false;
        fn pal (back: &Option<Box<ListNode>>, front: &mut &Option<Box<ListNode>>, done: &mut bool) -> bool {
            back.as_ref().is_none_or(|end| {
                pal(&end.next, front, done) && (*done || (*front).as_ref().is_some_and(|start| {
                    if ptr::eq(*front, back) || ptr::eq(&start.next, back) {*done = true;}
                    *front = &start.next;
                    start.val == end.val
                }))
            })
        }
        pal(&head, &mut front, &mut done)
    }
}

// Runtime: 7 ms, Beats 92.55%
// Memory: 12.42 MB, Beats 6.38%
