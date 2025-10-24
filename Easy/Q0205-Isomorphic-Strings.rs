impl Solution {
    pub fn is_isomorphic(s: String, t: String) -> bool {
        use std::collections::HashMap;
        
        let mut isomorphs = HashMap::<char, char>::new();
        let mut assigned = [false; 128];
        let mut tChar: char;
        
        for (i, c) in s.char_indices() {
            if let Some(&ch) = isomorphs.get(&c) {
                if t.chars().nth(i).unwrap() != ch {
                    return false;
                }
            } else {
                tChar = t.chars().nth(i).unwrap();
                if assigned[tChar as usize] {return false;}
                isomorphs.insert(c, tChar);
                assigned[tChar as usize] = true;
            }
        }
        true
    }
}

// Runtime: 271 ms, Beats 6.19%
// Memory: 2.42 MB, Beats 27.43%
