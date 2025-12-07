impl Solution {
    pub fn word_pattern(pattern: String, s: String) -> bool {
        let mut seen = [""; 26];
        let mut letters = pattern.as_bytes().iter();
        let mut words = s.split_whitespace();
        for ltr in letters {
            if let Some(word) = words.next() {
                if seen[*ltr as usize - 97].is_empty() && !seen.contains(&word) {
                    seen[*ltr as usize - 97] = word;
                }
                if seen[*ltr as usize - 97] != word {return false}
            } else {return false}
        }
        if let Some(_) = words.next() {return false}
        true
    }
}

// Runtime: 0 ms, Beats 100.00%
// Memory: 2.12 MB, Beats 55.71%
