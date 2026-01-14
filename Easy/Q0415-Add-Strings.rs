impl Solution {
    pub fn add_strings(num1: String, num2: String) -> String {
        let mut res = String::new();
        let mut r = 0;
        let (mut n1, mut n2) = (num1.chars().rev(), num2.chars().rev());
        loop {
            match (n1.next(), n2.next()) {
                (None, None) => {
                    if r == 1 {res.push('1');}
                    break res.chars().rev().collect()
                },
                (next1, next2) => {
                    let sum = next1.unwrap_or('0') as u8 + next2.unwrap_or('0') as u8 + r - b'0' - b'0';
                    res.push((sum % 10 + b'0') as char);
                    r = if sum < 10 {0} else {1};
                }
            }
        }
    }
}

// Runtime: 0 ms, Beats 100.00%
// Memory: 2.12 MB, Beats 86.49%
