use std::cmp::min;
use std::cmp::Ordering;

impl Solution {
    pub fn is_perfect_square(num: i32) -> bool {
        let (mut left, mut right) = (1, min(num, 46340));
        loop {
            if left > right {break false}
            let mid = left + (right-left)/2;
            (left, right) = match (mid*mid).cmp(&num) {
                Ordering::Less => (mid + 1, right),
                Ordering::Greater => (left, mid - 1),
                Ordering::Equal => break true
            }
        }
    }
}

// Runtime: 0 ms, Beats 100.00%
// Memory: 2.16 MB, Beats 52.00%
