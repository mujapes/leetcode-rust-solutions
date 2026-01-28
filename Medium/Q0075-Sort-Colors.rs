impl Solution {
    pub fn sort_colors(nums: &mut Vec<i32>) {
        let (mut red_hi, mut blue_lo) = (0, nums.len() as i32 -1);
        let mut i: usize = 0;
        while i as i32 <= blue_lo {
            if nums[i] == 2 {
                nums.swap(i, blue_lo as usize);
                blue_lo -= 1;
                continue
            }
            if nums[i] == 0 {
                nums.swap(red_hi, i);
                red_hi += 1;
            }
            i += 1;
        }
    }
}

// Runtime: 0 ms, Beats 100.00%
// Memory: 2.14 MB, Beats 59.52%
