impl Solution {
    pub fn permute_unique(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
    // Narayana Pandita iterative lexicographical algorithm
        nums.sort();
        let mut res = vec![nums.clone()];
    'build_res:
        loop {
            for i in (0..nums.len().saturating_sub(1)).rev() {
                if nums[i] < nums[i+1] {
                    for j in (0..nums.len()).rev() {
                        if nums[j] > nums[i] {
                            nums.swap(i, j);
                            nums[i+1..].reverse();
                            res.push(nums.clone());
                            continue 'build_res
                        }
                    }
                }
            }
            break res
        }
    }
}

// Runtime: 0 ms, Beats 100.00%
// Memory: 2.39 MB, Beats 23.19%
