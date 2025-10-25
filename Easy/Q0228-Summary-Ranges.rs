impl Solution {
    pub fn summary_ranges(nums: Vec<i32>) -> Vec<String> {
        if nums.len() == 0 {return Vec::<String>::new();}
        let mut ranges: Vec<String> = Vec::new();
        let mut start = 0;
        for i in 0..nums.len() {
            if i == nums.len()-1 || nums[i]+1 != nums[i+1] {
                if start == i {
                    ranges.push(nums[start].to_string());
                } else {
                    ranges.push(nums[start].to_string() + "->" + &nums[i].to_string());
                }
                start = i+1;
            }
        }
        ranges
    }
}

// Runtime: 0 ms, Beats 100.00%
// Memory: 2.23 MB, Beats 15.79%
