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
    pub fn reverse_list(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        std::iter::from_fn(move || {
            let mut cur = head.take()?;
            head = cur.next.take();
            Some(cur)
        })
        .reduce(|mut list, mut node| {
            node.next = Some(list);
            node
        })            
    }
}

// Runtime: 0 ms, Beats 100.00%
// Memory: 2.49 MB, Beats 84.44%
