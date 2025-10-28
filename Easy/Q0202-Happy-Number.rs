use std::collections::HashSet;

impl Solution {
    pub fn is_happy(mut n: i32) -> bool {
        let mut sum;
        let mut rem;
        let mut history = HashSet::new();
        while n != 1 && n != 10 && n != 100 {
            sum = 0;
            if history.contains(&n) {return false;}
            history.insert(n);
            while n > 0 {
                rem = n % 10;
                n = n / 10;
                sum += rem * rem;
            }
            n = sum;
        }
        true
    }
}

// Runtime: 0 ms, Beats 100.00%
// Memory: 2.09 MB, Beats 94.85%
