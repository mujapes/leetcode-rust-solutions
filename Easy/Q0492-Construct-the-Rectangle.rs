impl Solution {
    pub fn construct_rectangle(area: i32) -> Vec<i32> {
        let mut width = (area as f64).sqrt() as i32;
        loop {
            if area % width == 0 {
                break vec![area/width, width]
            }
            width -= 1;
        }
    }
}

// Runtime: 0ms, Beats 100.00%
// Memory: 2.12 MB, Beats 42.31%
