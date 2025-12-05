impl Solution {
    pub fn word_pattern(pattern: String, s: String) -> bool {
        let mut seen = [""; 26];
        let pattern_b = pattern.as_bytes();
        let mut wrd_cnt = 0;
        s.split_whitespace()
            .enumerate()
            .fold(true, |ok, (i, word)| { wrd_cnt += 1; ok && i < pattern.len() && match seen {
                seen if seen[pattern_b[i] as usize - 97] == word => true,
                ref mut seen if (*seen)[pattern_b[i] as usize - 97].is_empty() && !(*seen).contains(&word) => {
                    (*seen)[pattern_b[i] as usize - 97] = word;
                    true
                },
                _ => false
            } } ) && wrd_cnt == pattern.len()
    }
}

// Runtime: 0 ms, Beats 100.00%
// Memory: 2.12 MB, Beats 55.71%
