impl Solution {
    pub fn fib(n: i32) -> i32 {
        if n < 2 {return n}
        (2..=n)
            .fold( (0, 1), |(first, second), _| (second, first + second) )
            .1
    }
}

// Runtime: 0 ms, Beats 100.00%
// Memory: 2.16 MB, Beats 39.11%
