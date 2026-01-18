use std::cmp::{min, max};
impl Solution {
    pub fn insert(intervals: Vec<Vec<i32>>, new_interval: Vec<i32>) -> Vec<Vec<i32>> {
        let mut res = vec![];
        let mut merged = new_interval.clone();
        let mut pos = intervals.len();
        for i in 0..intervals.len() {
            if intervals[i][0] > new_interval[0] {
                pos = min(pos, i);
            }
            if intervals[i][0] <= new_interval[1] && intervals[i][1] >= new_interval[0] {
                merged[0] = min(merged[0], intervals[i][0]);
                merged[1] = max(merged[1], intervals[i][1]);
                pos = min(pos, i);
            } else {
                res.push(intervals[i].clone());
            }
        }
        res.insert(pos, merged);
        res
    }
}

// Runtime: 0 ms. Beats 100.00%
// Memory: 2.68 MB, Beats 81.91%
