impl Solution {
    pub fn judge_circle(moves: String) -> bool {
        let mut counts = [0; 18];
        for &mov in moves.as_bytes() { counts[mov as usize - 68] += 1; }
        counts[0] == counts[17] && counts[8] == counts[14]
    }
}

// Runtime: 0 ms, Beats 100.00%
// Memory: 2.16 MB, Beats 75.25%
