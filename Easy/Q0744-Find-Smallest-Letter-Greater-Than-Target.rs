impl Solution {
    pub fn next_greatest_letter(letters: Vec<char>, target: char) -> char {
        let (mut left, mut right) = (0, letters.len()-1);
        while left < right {
            let mid = left + (right - left)/2;
            if letters[mid] <= target {
                left = mid + 1;
            } else {right = mid;}
        }
        if letters[left] <= target {left = 0;}
        letters[left]
    }
}

// Runtime: 0 ms, Beats 100.00%
// Memory: 2.90 MB, Beats 40.48%
