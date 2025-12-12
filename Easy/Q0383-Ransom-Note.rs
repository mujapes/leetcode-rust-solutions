impl Solution {
    pub fn can_construct(ransom_note: String, magazine: String) -> bool {
        let mut frq = [0u16; 26];
        for c in magazine.chars() {frq[c as usize - 97] += 1}
        for c in ransom_note.chars() {
            let i = c as usize - 97;
            if frq[i] == 0 {return false}
            frq[i] -= 1;
        }
        true
    }
}

// Runtime: 0 ms, Beats 100.00%
// Memory: 2.25 MB, Beats 51.41%
