impl Solution {
    pub fn find_words(words: Vec<String>) -> Vec<String> {
        // keyboard row for each letter, increasing alphabetically from 'a'
        let letter_row = [2, 3, 3, 2, 1, 2, 2, 2, 1, 2, 2, 2, 3, 3, 1, 1, 1, 1, 2, 1, 1, 3, 1, 3, 1, 3];
        let mut found_words = vec![];
        for word in words {
            if let [first, rest @ ..] = word.as_bytes() {
                let row = letter_row[first.to_ascii_lowercase() as usize - 97];
                let mut valid = true;
                for letter in rest {
                    if letter_row[letter.to_ascii_lowercase() as usize - 97] != row {
                        valid = false;
                        break
                    }
                }
                if valid {found_words.push(word);}
            }
        }
        found_words
    }
}

// Runtime: 0 ms, Beats 100.00%
// Memory: 2.13 MB, Beats 86.84%
