impl Solution {
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        let (mut cum_sum, mut min_cum_sum, mut res) = (nums[0], nums[0], nums[0]);
        for i in 1..nums.len() {
            cum_sum += nums[i];
            res = *[res, cum_sum, cum_sum - min_cum_sum].iter().max().unwrap();
            if cum_sum < min_cum_sum {min_cum_sum = cum_sum;}
        }
        res
    }
}

// Runtime: 0 ms, Beats 100.00%
// Memory: 3.22 MB, Beats 63.72%
