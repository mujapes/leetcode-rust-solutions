impl Solution {
    pub fn arrange_coins(n: i32) -> i32 {
        ( (( 8. * n as f64 + 1. ).sqrt() - 1.) / 2. ) as i32
    }
}

// Runtime: 0 ms, Beats 100.00%
// Memory: 2.11 MB, Beats 66.67%
