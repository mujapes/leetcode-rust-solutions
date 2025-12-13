impl Solution {
    pub fn find_the_difference(s: String, t: String) -> char {
        let mut cnt_s = [0i16; 26];
        for c in s.chars() { cnt_s[c as usize -97] += 1; }
        for c in t.chars() {
            if cnt_s[c as usize -97] == 0 {
                return c
            } else {cnt_s[c as usize -97] -= 1;}
        }
        '\0'
    }
}

// Runtime: 0 ms, Beats 100.00%
// Memory: 2.16 MB, Beats 74.29%
