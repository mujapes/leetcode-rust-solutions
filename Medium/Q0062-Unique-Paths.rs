use std::cmp::{max, min};
impl Solution {
    pub fn unique_paths(m: i32, n: i32) -> i32 {
        ( max(m, n) as u64..(m+n-1) as u64 )
            .zip( (1..min(m, n) as u64) )
            .fold(1u64, |res, (num, den)| res * num / den) as i32
    }
}

// Runtime: 0 ms, Beats 100.00%
// Memory: 2.14 MB, Beats 50.44%
