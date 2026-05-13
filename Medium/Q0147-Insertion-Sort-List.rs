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
    pub fn insertion_sort_list(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut sorted: Option<Box<ListNode>> = None;
        while let Some(mut unsorted_node) = head {
            head = unsorted_node.next.take();
            let mut sorted_cur = &mut sorted;
            while sorted_cur.is_some() && sorted_cur.as_ref().unwrap().val < unsorted_node.val {
                sorted_cur = &mut sorted_cur.as_mut().unwrap().next;
            }
            unsorted_node.next = sorted_cur.take();
            *sorted_cur = Some(unsorted_node);
        }
        sorted
    }
}

// Runtime: 17 ms, Beats 45.45%
// Memory: 2.42 MB, Beats 54.55%
