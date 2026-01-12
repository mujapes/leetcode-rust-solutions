impl Solution {
    pub fn longest_palindrome(s: String) -> i32 {
        let mut res = 0;
        let mut odd_freq_mask = 0u64;
        for b in s.bytes() {
            let prev = odd_freq_mask;
            let offset = if b < 97u8 {b - 65} else {b - 71};
            odd_freq_mask ^= 1 << offset;
            if odd_freq_mask < prev {res += 2;}
        }
        if odd_freq_mask != 0 {res += 1;}
        res
    }
}

// Runtime: 0 ms. Beats 100.00%
// Memory: 2.28 MB, Beats 24.71%
