impl Solution {
    pub fn read_binary_watch(turned_on: i32) -> Vec<String> {
        let mut times = vec![];
        for h in 0..12u8 {
            for m in 0..60u8 {
                if h.count_ones() + m.count_ones() == turned_on as u32 {
                    times.push(format!("{}:{:02}", h, m));
                }
            }
        }
        times
    }
}

// Runtime: 0 ms, Beats 100.00%
// Memory: 2.18 MB, Beats 90.00%
