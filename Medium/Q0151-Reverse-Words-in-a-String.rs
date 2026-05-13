impl Solution {
    pub fn reverse_words(s: String) -> String {
        let mut reversed = String::with_capacity(s.len() + 1);
        reversed.extend(
            s.split_whitespace()
            .rev()
            .map(|s| String::from(s) + " ")
        );
        reversed.pop();
        reversed
    }
}

// Runtime: 0 ms, Beats 100.00%
// Memory: 2.26 MB, Beats 45.60%
