impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        if s.len() != t.len() {return false;}
        let s_bytes = s.as_bytes();
        let t_bytes = t.as_bytes();
        let mut s_char_counts = [0u32; 26];
        let mut t_char_counts = [0u32; 26];
        for i in 0..s.len() {
            s_char_counts[s_bytes[i] as usize - 'a' as usize] += 1;
            t_char_counts[t_bytes[i] as usize - 'a' as usize] += 1;
        }
        s_char_counts == t_char_counts
    }
}

// Runtime: 0 ms, Beats 100.00%
// Memory: 2.24 MB, Beats 82.85%
