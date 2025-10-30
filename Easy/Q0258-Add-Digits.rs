impl Solution {
    pub fn add_digits(mut num: i32) -> i32 {
        let mut sum;
        while num > 9 {
            sum = 0;
            while num > 0 {
                sum += num % 10;
                num /= 10;
            }
            num = sum;
        }
        num
    }
}

// Runtime: 0 ms, Beats 100.00%
// Memory: 2.16 MB, Beats 45.00%
