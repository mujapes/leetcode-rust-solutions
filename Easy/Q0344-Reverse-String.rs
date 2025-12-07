impl Solution {
    pub fn reverse_string(s: &mut Vec<char>) {
        let len = s.len();
        for i in 0..len/2 {
            s.swap(i, len-1-i);
        }
    }
}

// Runtime: 0 ms, Beats 100.00%
// Memory: 5.55 MB, Beats 66.67%
