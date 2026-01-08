// Definition for a binary tree node.
// #[derive(Debug, PartialEq, Eq)]
// pub struct TreeNode {
//   pub val: i32,
//   pub left: Option<Rc<RefCell<TreeNode>>>,
//   pub right: Option<Rc<RefCell<TreeNode>>>,
// }
// 
// impl TreeNode {
//   #[inline]
//   pub fn new(val: i32) -> Self {
//     TreeNode {
//       val,
//       left: None,
//       right: None
//     }
//   }
// }
use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn sum_of_left_leaves(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if let Some(node) = root {
            if let Some(left) = node.borrow().left.clone() {
                if left.borrow().left.clone() == None && left.borrow().right.clone() == None {
                    return left.borrow().val + Self::sum_of_left_leaves(node.borrow().right.clone())
                }
            }
            Self::sum_of_left_leaves(node.borrow().left.clone()) + Self::sum_of_left_leaves(node.borrow().right.clone())
        } else { 0 }
    }
}

// Runtime: 0 ms, Beats 100.00%
// Memory: 2.32 MB, Beats 35.29%
