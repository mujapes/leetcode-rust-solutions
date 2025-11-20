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
    pub fn is_palindrome(head: Option<Box<ListNode>>) -> bool {
        let mut mid_distance = u16::MAX;
        let mut front = &head;
        fn pal (back: &Option<Box<ListNode>>, front: &mut &Option<Box<ListNode>>, mid_distance: &mut u16) -> bool {
            if let Some(end) = back {
                *mid_distance += 1;
                if pal(&end.next, front, mid_distance) {
                    if *mid_distance < 0 {return true}
                    *mid_distance -= 1;
                    if let Some(start) = *front {
                        if start.val == end.val {
                            *front =  &start.next;
                            return true
                        }
                    }
                }
                false
            } else {
                *mid_distance /= 2;
                true
            }
        }
        pal(&head, &mut front, &mut mid_distance)
    }
}

// Runtime: 20 ms, Beats 37.23%
// Memory: 9.16 MB. Beats 44.68%
