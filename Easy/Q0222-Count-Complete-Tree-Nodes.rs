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
    pub fn count_nodes(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        fn traverse(root: &Option<Rc<RefCell<TreeNode>>>, count: u16, height: u8) -> impl Iterator<Item = Rc<RefCell<TreeNode>>> {
            let mut depth = 0;
            let mut cur = root.clone();
            std::iter::from_fn(move || {
                if cur.is_none() {return None}
                depth += 1;
                if depth == height {return std::mem::replace(&mut cur, None)}
                let next = if count & 1 << (height - 1 - depth) == 0 {
                    cur.clone()?.borrow().left.clone()
                } else {
                    cur.clone()?.borrow().right.clone()
                };
                std::mem::replace(&mut cur, next)
            })
        }
        let mut count: u16 = 0;
        let height = traverse(&root, count, 16).count() as u8;
        if height == 0 {return 0}
        for halving in 0..height-1 {
            count |= 1 << (height -  2 - halving);
            if (traverse(&root, count, height).count() as u8) < height {count &= !(1 << (height - 2 - halving))}
        }
        (count + (1 << height - 1)) as i32
    }
}

// Runtime: 0ms, Beats 100.00%
// Memory: 4.68 MB, Beats 88.89%
