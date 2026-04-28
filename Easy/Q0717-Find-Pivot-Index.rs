impl Solution {
    pub fn pivot_index(nums: Vec<i32>) -> i32 {
        // prefix sum array
        let mut prefix_sum = Vec::with_capacity(nums.len());
        prefix_sum.push(0);
        prefix_sum.push(nums[0]);
        for i in 1..nums.len() {
            prefix_sum.push(nums[i] + prefix_sum[i]);
        }
        let right = prefix_sum[prefix_sum.len()-1];
        for i in 0..prefix_sum.len()-1 {
            if right - prefix_sum[i+1] == prefix_sum[i] {
                return i as i32;
            }
        }
        -1
     }
}

// Runtime: 0 ms, Beats 100.00%
// Memory: 2.30 MB, Beats 24.66%
