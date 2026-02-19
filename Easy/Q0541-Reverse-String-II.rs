impl Solution {
    pub fn reverse_str(s: String, k: i32) -> String {
        let mut pos = 0;
        let mut rev = true;
        let mut b_buff = Vec::<u8>::with_capacity(s.len());
        while pos < s.len() {
            let bound = s
                .len()
                .min(k as usize + pos);
            if rev {
                b_buff.extend(    
                    s.as_bytes()
                        [pos..bound]
                        .iter()
                        .rev()
                        .copied()
                )
            } else {
                b_buff.extend(
                    s.as_bytes()
                        [pos..bound]
                        .iter()
                        .copied()
                )
            }
            pos += k as usize;
            rev = !rev;
        }
        String::from_utf8(b_buff).unwrap()
    }
}

// Runtime: 0 ms, Beats 100.00%
// Memory: 2.26 MB, Beats 60.61%
