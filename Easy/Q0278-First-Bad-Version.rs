// The API isBadVersion is defined for you.
// isBadVersion(version:i32)-> bool;
// to call it use self.isBadVersion(version)
use std::iter::successors;
use std::cmp::max;

impl Solution {
    pub fn first_bad_version(&self, n: i32) -> i32 {
		    successors(Some(n.saturating_add(1)/2), |gap| match gap {
            0 => None,
            1 => Some(0),
            _ => Some((gap + 1)/2)
        }).reduce(|mid, gap| match self.isBadVersion(mid) {
            true => mid - gap,
            false => mid + max(gap, 1)
        }).expect("Input must be > 0")
    }
}

// Runtime: 0 ms, Beats 100.00%
// Memory: 2.17 MB, Beats 37.14%
