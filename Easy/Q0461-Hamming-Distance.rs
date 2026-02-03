impl Solution {
    pub fn hamming_distance(x: i32, y: i32) -> i32 {
        (x ^ y).count_ones() as i32
    }
}

// Runtime: 0 ms, Beats 100.00%
// Memory: 2.25 MB, Beats 12.50%
