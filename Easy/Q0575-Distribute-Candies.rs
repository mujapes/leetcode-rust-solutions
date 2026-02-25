use std::collections::HashSet;

impl Solution {
    pub fn distribute_candies(candy_type: Vec<i32>) -> i32 {
        let limit = candy_type.len() as i32 / 2;
        let mut count = 0;
        let mut seen = HashSet::new();
        for t in candy_type {
            if !seen.contains(&t) {
                count += 1;
                if count == limit {return count}
                seen.insert(t);
            }
        }
        count
    }
}

// Runtime: 3 ms, Beats 95.35%
// Memory: 2.37 MB,Beats 44.19%
