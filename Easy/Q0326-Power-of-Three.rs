impl Solution {
    pub fn is_power_of_three(n: i32) -> bool {
        if n < 1 {return false}
        let mut prod = 1;
        loop {
            match prod {
                p if p == n => return true,
                p if p > n || p == 1162261467 => return false,
                _ => {prod *= 3;}
            }
        }
    }
}

// Runtime: 0 ms, Beats 100.00%
// Memory: 2.16 MB. Beats 40.00%
