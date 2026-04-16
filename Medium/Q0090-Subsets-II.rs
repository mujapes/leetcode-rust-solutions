use std::collections::HashSet;

impl Solution {
    pub fn subsets_with_dup(nums: Vec<i32>) -> Vec<Vec<i32>> {
        fn powerset_recursive(nums: Vec<i32>) -> Vec<Vec<i32>> {
            let mut powerset = vec![vec![]];
            for cur in 0..nums.len() {
                let mut rest_of_nums = nums[cur+1..].to_vec();
                for mut subset in powerset_recursive(rest_of_nums) {
                    subset.push(nums[cur]);
                    powerset.push(subset);
                }
            }
            powerset
        }
        let mut powerset = powerset_recursive(nums);
        for subset in &mut powerset {subset.sort_unstable();}
        powerset.into_iter()
            .collect::<HashSet<_>>()
            .into_iter()
            .collect()
    }
}

// Runtime: 0 ms, Beats 100.00%
// Memory: 2.42 MB, Beats-%
