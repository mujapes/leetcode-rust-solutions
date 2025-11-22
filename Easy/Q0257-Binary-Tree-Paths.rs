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
    pub fn binary_tree_paths(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<String> {
        let Some(node) = &root else {return vec![]};
        let cur = node.borrow();
        if let (None, None) = (&cur.left, &cur.right) {return vec![cur.val.to_string()]}
        Self::binary_tree_paths(cur.left.clone())
            .into_iter()
            .chain(Self::binary_tree_paths(cur.right.clone()).into_iter())
            .map(|s| format!("{}->{}", cur.val, s))
            .collect()
    }
}

// Runtime: 0 ms, Beats 100.00%
// Memory: 2.18 MB, Beats 100.00%
