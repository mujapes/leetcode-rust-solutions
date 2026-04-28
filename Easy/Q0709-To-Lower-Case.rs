impl Solution {
    pub fn to_lower_case(s: String) -> String {
        let mut new_bytes: Vec<u8> = Vec::with_capacity(s.len());
        for &(mut b) in s.as_bytes() {
            if b > 64 && b < 91 { b += 32; }
            new_bytes.push(b);
        }
        String::from_utf8(new_bytes).unwrap()
    }
}

// Runtime: 0 ms, Beats 100.00%
// Memory: 2.04 MB, Beats 100.00%
