impl Solution {
    pub fn first_uniq_char(s: String) -> i32 {
        let mut pos = [-1i32; 26];
        let mut char_cnt: i8 = 0;
        let mut char_to_pos = [-1i8; 26];
        for (i, b) in s.bytes().enumerate() {
            let idx = b as usize - 97;
            if char_to_pos[idx] == -1 {
                pos[char_cnt as usize] = i as i32;
                char_to_pos[idx] = char_cnt;
                char_cnt += 1;
            } else if char_to_pos[idx] >= 0 {
                pos[char_to_pos[idx] as usize] = -1;
                char_to_pos[idx] = -2;
            }
        }
        for p in pos {if p >= 0 {return p}}
        -1
    }
}

// Runtime: 0 ms, Beats 100.00%
// Memory: 2.38 MB, Beats 36.00%
