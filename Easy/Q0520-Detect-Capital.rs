impl Solution {
    pub fn detect_capital_use(word: String) -> bool {
        let mut expect_upper = word
            .chars()
            .nth(0)
            .unwrap()
            .is_uppercase();
        if expect_upper {
            if let Some(c) = word.chars().nth(1) {
                expect_upper = c.is_uppercase();
            }
        }   
        for c in word.chars().skip(1) {
            if c.is_uppercase() != expect_upper {return false}
        }
        true
    }
}

// Runtime: 0 ms, Beats 100.00%
// Memory: 2.04 MB, Beats 93.85%
