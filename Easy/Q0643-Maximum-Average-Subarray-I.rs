impl Solution {
    pub fn find_max_average(nums: Vec<i32>, k: i32) -> f64 {
        /*
        max average subarray also max total subarray
        */
        let k = k as usize;
        let mut cur_sum: i32 = nums[..k].iter().sum();
        let mut max_sum = cur_sum;
        for sub_end in k..nums.len() {
            cur_sum += nums[sub_end];
            cur_sum -= nums[sub_end - k];
            max_sum = max_sum.max(cur_sum);
        }
        max_sum as f64 / k as f64
    }
}

// Runtime: 2 ms, Beats 41.38%
// Memory: 3.06 MB, Beats 46.55%
