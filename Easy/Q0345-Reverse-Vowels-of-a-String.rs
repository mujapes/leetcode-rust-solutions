impl Solution {
    pub fn reverse_vowels(s: String) -> String {
        const VOWELS: [char; 10] = ['a', 'e', 'i', 'o', 'u', 'A', 'E', 'I', 'O', 'U'];
        let mut vwls = s.chars().rev().filter( |c| VOWELS.contains(&c) );
        s.chars()
            .map( |c| if VOWELS.contains(&c) {vwls.next().expect("No matching vowel")} else {c} )
            .collect()
    }
}

// Runtime: 0 ms, Beats 100.00%
// Memory: 2.33 MB. Beats 99.33%
