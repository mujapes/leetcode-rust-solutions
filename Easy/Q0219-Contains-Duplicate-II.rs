use std::collections::HashSet;

impl Solution {
    pub fn contains_nearby_duplicate(nums: Vec<i32>, k: i32) -> bool {
        // hash set containing k elements oldest dropped as newest added
        let k = k as usize;
        let mut set: HashSet<i32> = HashSet::new();
        for i in 0..nums.len() {
            if !set.insert(nums[i]) {return true;}
            if i >= k {set.remove(&nums[i-k]);}
        }
        false
    }
}

// Runtime: 9 ms, Beats 85.20%
// Memory: 3.42 MB, Beats 82.14%
