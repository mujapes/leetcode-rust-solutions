impl Solution {
    pub fn reverse_bits(n: i32) -> i32 {
        let mut m: i32 = 0;
        for s in 0..31 {
            if n & 1<<s == 1<<s {m += 1}
            m = m << 1;
        }
        m
    }
}
// Runtime: 2 ms, Beats 47.54%
// Memory: 2.03 MB, Beats 92.62%
