impl Solution {
    pub fn check_record(s: String) -> bool {
        let (mut absence, mut late_cnt) = (0, 0);
        for &record in s.as_bytes() {
            if record == b'A' {absence += 1;} 
            if record == b'L' {
                late_cnt += 1;
            } else {
                late_cnt = 0;
            }
            if absence > 1 || late_cnt > 2 {return false}
        }
        true
    }
}

// Runtime: 0 ms, Beats 100.00%
// Memory: 2.17 MB, Beats 42.11%
