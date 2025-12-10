impl Solution {
    pub fn intersect(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let mut cnt1 = [0u16; 1001];
        for n in nums1 { cnt1[n as usize] += 1 }
        nums2.into_iter()
            .filter( |n| if cnt1[*n as usize] > 0 {cnt1[*n as usize] -= 1; true} else {false} )
            .collect()
    }
}

// Runtime: 0 ms, Beats 100.00%
// Memory: 2.08 MB, Beats 100.00%
