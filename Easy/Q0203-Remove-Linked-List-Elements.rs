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
impl Solution {
    pub fn remove_elements(mut head: Option<Box<ListNode>>, val: i32) -> Option<Box<ListNode>> {
        if let Some(mut node) = head {
            node.next = Self::remove_elements(node.next, val);
            if node.val == val {node = node.next?;}
            head = Some(node);
        }
        head
    }
}

// Runtime: 0 ms, Beats 100.00%
// Memory: 2.78 MB, Beats 98.36%
