impl Solution {
    pub fn find_shortest_sub_array(nums: Vec<i32>) -> i32 {
        // single element has highest frequency: subarray from first to last appearance
        // multiplied tied for highest frequency: compare all
        // track length of subarray for each value 
        let mut max_cnt = 0;
        let mut min_len = u16::MAX;
        let mut occurences = [[u16::MAX, 0]; 50_000];
        for i in 0..nums.len() {
            let num = nums[i] as usize;
            if occurences[num][0] == u16::MAX {
                occurences[num][0] = i as u16;
            }
            occurences[num][1] += 1;
            let (first, count) = (occurences[num][0], occurences[num][1]);
            let len = i as u16 - first + 1;
            if count > max_cnt {
                max_cnt = count;
                min_len = len;
             } else if count == max_cnt {
                min_len = min_len.min(len);
             }
        }
        min_len as i32
    }
}

// Runtime: 0 ms, Beats 100.00%
// Memory: 2.5 MB, Beats 80.00%
