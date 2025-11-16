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
    pub fn invert_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        let mut node = root?;
        {
            let mut cur = node.borrow_mut();
            if cur.left != None || cur.right != None {
                let left = Self::invert_tree(cur.left.take());
                cur.left = Self::invert_tree(cur.right.take());
                cur.right = left;
                //std::mem::swap(&mut cur.left, &mut cur.right);
            }
        }
        Some(node)
    }
}

// Runtime: 0ms, Beats 100.00%
// Memory: 2.11 MB, Beats 69.50%
