impl Solution {
    pub fn dominant_index(nums: Vec<i32>) -> i32 {
        nums.into_iter()
            .enumerate()
            .fold((0, -1), |(max, pos), (i, n)| {
                if n >= max*2 {return (n, i as i32);}
                if n >= max {return (n, -1);}
                if n*2 > max {return (max, -1);}
                (max, pos)
            }).1
    }
}

// Runtime: 0 ms, Beats 100.00%
// Memory: 2.15 MB, Beats 62.07%
