impl Solution {
    pub fn fib(n: i32) -> i32 {
        if n < 2 {return n}
        Self::fib(n-1) + Self::fib(n-2)
    }
}

// Runtime: 7 ms, Beats 7.43%
// Memory: 2.14 MB, Beats 39.11%
