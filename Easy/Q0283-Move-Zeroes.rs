impl Solution {
    pub fn move_zeroes(nums: &mut Vec<i32>) {
        (0..nums.len())
            .fold(None, |first_zero, cur_idx| match nums[cur_idx] == 0 {
                true => first_zero.or_else(|| Some(cur_idx)),
                false => first_zero.map(|zero_idx| {
                    nums.swap(zero_idx, cur_idx);
                    zero_idx + 1
                })
            });
    }
}

// Runtime: 0 ms, Beats 100.00%
// Memory: 2.39 MB, Beats 60.31%
