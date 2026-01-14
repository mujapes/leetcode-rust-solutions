impl Solution {
    pub fn count_segments(s: String) -> i32 {
        let mut prev_space = true;
        s.chars().filter(|&c| {
            let new_seg = prev_space && c != ' ';
            prev_space = c == ' ';
            new_seg
        }).count() as i32
    }
}

// Runtime: 0 ms, Beats 100.00%
// Memory: 2.17 MB, Beats 45.16%
