impl Solution {
    pub fn fib(n: i32) -> i32 {
        let (mut first, mut second) = (0, 1);
        if n == 0 {return first}
        for i in (2..=n) {
            (first, second) = (second, first + second);
        }
        second
    }
}

// Runtime: 0 ms, Beats 100.00%
// Memory: 2.09 MB, Beats 86.63%
