impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        let mut dups = 0;
        let mut prev = nums[0];
        let mut prev_cnt: usize = 1;
        for i in 1..nums.len() {
            if nums[i] != prev {
                dups += prev_cnt.saturating_sub(2);
                prev_cnt = 0;
                prev = nums[i];
            }
            if dups > 0 && prev_cnt < 3 { nums.swap(i-dups, i); }
            prev_cnt += 1;
        }
        dups += prev_cnt.saturating_sub(2);
        (nums.len() - dups) as i32
    }
}

// Runtime: 3 ms, Beats 74.86%
// Memory: 2.32 MB, Beats 21.23%
