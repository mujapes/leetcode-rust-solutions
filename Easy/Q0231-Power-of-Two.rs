impl Solution {
    pub fn is_power_of_two(mut n: i32) -> bool {
        if n < 1 {return false;}
        while n & 1 != 1 {n = n >> 1;}
        if n != 1 {return false;}
        true
    }
}

// Runtime: 0 ms, Beats 100.00%
// Memory: 2.20 MB, Beats 43.69%
