use std::collections::HashSet;

impl Solution {
    pub fn intersection(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let mut dup: HashSet<i32> = nums1.into_iter().collect();
        nums2.into_iter()
            .filter(|n| dup.remove(n))
            .collect()
    }
}

// Runtime: 0 ms, Beats 100.00%
// Memory: 2.17 MB, Beats 73.44%
