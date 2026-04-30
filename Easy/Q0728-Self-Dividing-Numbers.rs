impl Solution {
    pub fn self_dividing_numbers(left: i32, right: i32) -> Vec<i32> {
        let mut valid_nums = Vec::with_capacity(((right-left)/16) as usize);
        for num in left..=right {
            let mut num_cpy = num;
            let mut valid = true;
            while num_cpy > 0 {
                let dig = num_cpy % 10;
                num_cpy /= 10;
                if dig == 0 || num % dig != 0 {
                    valid = false;
                    break;
                }
            }
            if valid {valid_nums.push(num);}
        }
        valid_nums
    }
}

// Runtime: 0 ms, Beats 100.00%
// Memory: 2.16 MB, Beats 58.93%
