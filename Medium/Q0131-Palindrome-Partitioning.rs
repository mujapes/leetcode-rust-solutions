use std::collections::HashSet;


impl Solution {
    pub fn partition(s: String) -> Vec<Vec<String>> {
        // find non-trivial palindromes then recursively combine with all others
        // search for length 2 and length 3 palindromes as these will seed all larger onees
        // sort storage bounds by lower AND upper bounds?
        // data structure that quickly shows elements exluded by bounds
        // collapes bounds into one number
        
        // array of single chars then 
        let mut parts: HashSet<Vec<String>> = HashSet::new();
        let mut trivial = Vec::with_capacity(s.len());

        for c in s.chars() {
            trivial.push(c.to_string());
        }
        
        fn recurse(part: Vec<String>, parts: &mut HashSet<Vec<String>>) {
            parts.insert(part.clone());
            for i in 0..part.len()-1 {
                // 2 element palindrom builder
                if part[i].len() == 1 && part[i+1] == part[i] {
                    let mut new_part = Vec::with_capacity(part.len()-1);
                    new_part.extend_from_slice(&part[0..i]);
                    new_part.push( format!("{}{}", part[i], part[i]) );
                    new_part.extend_from_slice(&part[i+2..part.len()]);
                    recurse(new_part, parts);
                }
                if i > 0 {
                    // palindrome expansion
                    if part[i-1] == part[i+1].chars().rev().collect::<String>() {
                        let mut new_part = Vec::with_capacity(part.len()-1);
                        new_part.extend_from_slice(&part[0..i-1]);
                        new_part.push( format!("{}{}{}", part[i-1], part[i], part[i+1]) );
                        new_part.extend_from_slice(&part[i+2..part.len()]);
                        recurse(new_part, parts);
                    }
                }
            }
        }

        recurse(trivial, &mut parts);
        parts.into_iter().collect()
    }
}
