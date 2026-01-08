impl Solution {
    pub fn to_hex(num: i32) -> String {
        if num == 0 { return String::from("0") }
        let to_hex = [
            '0', '1', '2', '3', '4', '5', '6', '7',
            '8', '9', 'a', 'b', 'c', 'd', 'e', 'f'  
        ];
        let mut s = String::with_capacity(8);
        let mut leading_zero = true;
        for b in num.to_be_bytes() { 
            if leading_zero {
                if b >> 4 != 0 {
                    leading_zero = false;
                    s.push(to_hex[b as usize >> 4]);
                }
            } else { s.push(to_hex[b as usize >> 4]); }
            if leading_zero {
                if b & 15 != 0 {
                    leading_zero = false;
                    s.push(to_hex[b as usize & 15]);
                }
            } else { s.push(to_hex[b as usize & 15]); }
        }
        s
    }
}

// Runtime: 0 ms, Beats 100.00%
// Memory: 2.24 MB, Beats 2.78%
