use std::collections::HashMap;

impl Solution {
    pub fn find_error_nums(nums: Vec<i32>) -> Vec<i32> {
        let mut count = vec![0_i32; nums.len()];
        for num in nums {
            count[num as usize - 1] += 1;
        }
        let (mut dup, mut abs) = (0, 0);
        for i in 0..count.len() {
            let cnt = count[i];
            if cnt == 0 {
                abs = i as i32 + 1;
            } else if cnt == 2 {
                dup = i as i32 + 1;
            }
            if dup != 0 && abs != 0 {
                return vec![dup, abs]
            }
        }
        vec![]
    }
}

// Runtime: 0 ms, Beats 100.00%
// Memory: 2.32 MB, Beats 35.10%
