impl Solution {
    pub fn find_complement(num: i32) -> i32 {
        num ^ ( i32::MAX >> num.leading_zeros() - 1 )
    }
}

// Runtime: 0 ms, Beats 100.00%
// Memory: 2.01 MB, Beats 88.46%
