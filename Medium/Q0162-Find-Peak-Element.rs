impl Solution {
    pub fn find_peak_element(nums: Vec<i32>) -> i32 {
        // between the middle element and the side that is larger must be a peak
        // if both sides are smaller then whichever side has the larger neighbour must have a peak

        //  discrete local maxima search
        // trivial solution is following a larger neighbour, how to make logn?
        // at each point we know a maxima exists to any side with larger neighbour
        // we can afford to move half way into that subarray, dont need to sample left or right
        let mut left = 0;
        let mut right = nums.len()-1;
        while left < right {
            let mid = left + (right - left)/2;
            let mid_num = nums[mid];
            if (mid == 0 || nums[mid-1] < mid_num) {
                if (mid == nums.len()-1 || nums[mid+1] < mid_num) {
                    return mid as i32;
                }
                left = mid + 1;
            } else { right = mid; }
        }
        left as i32
    }
}

// Runtime: 0 ms, Beats 100.00%
// Memory: 2.14 MB, Beats 53.97%
