impl Solution {
    pub fn count_bits(n: i32) -> Vec<i32> {
        (0..n+1).map( |i| i.count_ones() as i32 ).collect()
    }
}

// Runtime: 0 ms, Beats 100.00%
// Memory: 2.55 MB, Beats 89.38%
