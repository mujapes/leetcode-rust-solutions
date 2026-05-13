impl Solution {
    pub fn max_product(nums: Vec<i32>) -> i32 {
        // zeros are destructive, any subarray with 2 non-zero elements dominates
        // zeros are delimiters, track last_zero
        // odd number of negative numbers decreases value, track first_negative
        nums.into_iter()
            .fold((i32::MIN, 1, 0), |(max, cur_prod, post_negative_prod), n| {
                let cur_prod = cur_prod * n;
                let post_negative_prod = post_negative_prod * n;
                match n {
                    0 => (max.max(0), 1, 0),
                    n if n < 0 && post_negative_prod == 0 => {
                        (max.max(cur_prod), cur_prod, 1)
                    },
                    _ => (max.max(cur_prod).max(post_negative_prod), cur_prod, post_negative_prod)
                }
            }).0
    }
}

// Runtime: 0 ms, Beats 100.00%
// Memory: 2.38 MB, Beats 17.46%
