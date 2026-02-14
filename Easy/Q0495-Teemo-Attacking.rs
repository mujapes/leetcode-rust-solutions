impl Solution {
    pub fn find_poisoned_duration(time_series: Vec<i32>, duration: i32) -> i32 {
        let (mut poison_end, mut res) = (0, 0);
        for t in time_series {
            if poison_end >= t {
                res += duration + t - poison_end;
            } else {
                res += duration;
            }
            poison_end = t + duration;
        }
        res
    }
}
// Runtime: 0 ms, Beats 100.00%
// Memory: 2.32 MB, Beats 17.24%
