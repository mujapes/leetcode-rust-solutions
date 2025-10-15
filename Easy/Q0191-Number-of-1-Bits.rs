impl Solution {
    pub fn hamming_weight(n: i32) -> i32 {
        let mut res: i32 = 0;
        for s in 0..32 {
            if n & 1<<s == 1<<s {
                res += 1;
            }
        }
        res
    }
}

// Runtime: 1 ms, Beats 2.29%
// Memory: 2.17 MB, Beats 38.17%
