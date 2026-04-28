impl Solution {
    pub fn is_one_bit_character(bits: Vec<i32>) -> bool {
        // second to last bit 0 = true
        // any 1 must be a 2 bit char
        let mut second_bit = false;
        for i in 0..bits.len()-1 {
            if bits[i] == 1 && !second_bit {
                second_bit = true;
            } else {
                second_bit = false;
            }
        }
        !second_bit
    }
}

// Runtime: 0 ms, Beats 100.00%
// Memory: 2.16 MB, Beats 50.00%
