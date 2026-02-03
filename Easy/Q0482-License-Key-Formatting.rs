impl Solution {
    pub fn license_key_formatting(s: String, k: i32) -> String {
        let mut res = String::from("");
        let mut next_dash = k;
        for &b in s.as_bytes().iter().rev().filter(|&&b| b != b'-') {
            res.push(b.to_ascii_uppercase() as char);
            if next_dash == 1 {
                res.push('-');
                next_dash = k;
            } else { next_dash -= 1; }
        }
        if res.as_bytes().last().unwrap_or(&b'\0') == &b'-' { res.pop(); }
        res.as_bytes().iter().rev().map(|&b| b as char).collect()
    }
}

// Runtime: 0 ms, Beats 100.00%
// Memory: 2.26 MB, Beats 95.38%
