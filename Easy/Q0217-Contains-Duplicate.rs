use std::cmp::Ordering;

impl Solution {
    pub fn contains_duplicate(mut nums: Vec<i32>) -> bool {
        let mut dup = false;
        nums.sort_by(|a, b| match a.cmp(&b) {
            Ordering::Equal => {
                dup = true;
                Ordering::Equal
            }
            // Return Less or Greater for non-equal cases
            // If duplicate was found end sorting early
            other => other,
});
        if dup {return true}
        for i in 0..nums.len()-1 {
            if nums[i] == nums[i+1]{
                return true;
            }
        }
        false
    }
}

// Runtime: 15 ms, Beats 8.16%
// Memory: 3.02 MB, Beats 96.23%
