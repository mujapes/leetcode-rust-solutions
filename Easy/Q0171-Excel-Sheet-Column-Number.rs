impl Solution {
    pub fn title_to_number(column_title: String) -> i32 {
        let length = column_title.len() as u32;
        let mut res: i32 = 0;
        let mut cnt = length-1;
        for char in column_title.chars() {
            res += 26i32.pow(cnt) * (1 + char as i32 - 'A' as i32);
            cnt -= 1;
        }
        res
    }
}

// Runtime: 0 ms, Beats 100.00%
// Memory: 2.13 MB, Beats 41.67%
