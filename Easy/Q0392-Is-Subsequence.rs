impl Solution {
    pub fn is_subsequence(s: String, t: String) -> bool {
        let mut seen = [10001i16; 26];
        let mut ahead: Vec<[i16; 26]> = t.bytes()
                .enumerate()
                .rev()
                .map( |(i, b)| {seen[b as usize - 97] = i as i16 + 1; seen} )
                .collect();
        ahead.reverse();
        ahead.push([10001i16; 26]);
        let mut next_idx = 0;
        if s.len() > t.len() {return false}
        for b in s.bytes() {
            next_idx = ahead[next_idx][b as usize - 97] as usize;
            if next_idx == 10001 {return false}
        } true
    }
}

// Runtime: 0ms, Beats 100.00%
// Memory: 2.69 MB, Beats 1.27%
