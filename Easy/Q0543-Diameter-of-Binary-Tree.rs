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
    pub fn diameter_of_binary_tree(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut res = 0;
        fn get_height(node: &Option<Rc<RefCell<TreeNode>>>, res: &mut i32) -> i32 {
            if let Some(n) = node {
                let node_ref = n.borrow();
                let (left, right) = ( get_height(&node_ref.left, res), get_height(&node_ref.right, res) );
                *res = (*res).max(left + right);
                return left.max(right) + 1;
            }
            0
        }
        get_height(&root, &mut res);
        res
    }
}

// Runtime: 0 ms, Beats 100.00%
// Memory: 3.70 MB, Beats 65.67%
