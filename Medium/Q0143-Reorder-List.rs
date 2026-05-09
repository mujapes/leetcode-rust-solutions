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
    pub fn reorder_list(head: &mut Option<Box<ListNode>>) {
        let mut next = &head.as_ref().unwrap().next;
        let mut len = 1;
        while let Some(node) = next {
            len += 1;
            next = &node.next;
        }
        let mut mid = None;
        let mut cur = head.as_mut();
        for n in 0..len/2 {
            if let Some(node) = cur {
                cur = node.next.as_mut();
            }
        }
        mid = cur.unwrap().next.take();
        let mut cur = head.as_mut();
        fn helper(reflection: Option<Box<ListNode>>, cur: &mut Option<&mut Box<ListNode>>) {
            if let Some(mut reflection_node) = reflection {
                helper(reflection_node.next.take(), cur);
                if let Some(cur_node) = cur.take() {
                    reflection_node.next = cur_node.next.take();
                    cur_node.next = Some(reflection_node);
                    *cur = cur_node.next.as_mut().unwrap().next.as_mut();
                }

            }
        }
        helper(mid, &mut cur);
    }
}

// Runtime: 0 ms, Beats 100.00%
// Memory: 4.21 MB, Beats 69.23%
