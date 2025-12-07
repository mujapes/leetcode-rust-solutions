impl Solution {
    pub fn is_power_of_four(n: i32) -> bool {
        // -1431655766 = 10101010101010101010101010101010
        ( -1431655766 & n == 0 ) && ( n & n-1 == 0 ) && n != 0
    }
}

// Runtime: 0 ms, Beats 100.00%
// Memory: 2.16 MB, Beats 39.22%
