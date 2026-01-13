impl Solution {
    pub fn third_max(nums: Vec<i32>) -> i32 {
        let mut max = Vec::new();
        for n in nums {
            for i in 0..3 {
                if i == max.len() { max.push(n); break }
                if max[i] == n {break}
                if max[i] < n { max.insert(i, n); break }
            }
        }
        if max.len() < 3 {return max[0]}
        max[2]
    }
}

// Runtime: 0 ms, Beats 100.00%
// Memory: 2.13 MB, Beats 97.78%
