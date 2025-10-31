impl Solution {
    pub fn missing_number(nums: Vec<i32>) -> i32 {
        let mut mask = 0;
        for n in 0..nums.len() {mask ^= nums[n] ^ n as i32;}
        mask ^ nums.len() as i32
    }
}

// Runtime: 0 ms, Beats 100.00%
// Memory: 2.40 MB, Beats 20.65%
