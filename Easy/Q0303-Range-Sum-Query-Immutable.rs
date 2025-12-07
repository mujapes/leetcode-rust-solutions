struct NumArray {
    nums: Vec<i32>
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl NumArray {

    fn new(nums: Vec<i32>) -> Self {
        NumArray { nums }
    }
    
    fn sum_range(&self, left: i32, right: i32) -> i32 {
        self.nums[left as usize..right as usize+1].iter().sum()
    }
}

/**
 * Your NumArray object will be instantiated and called as such:
 * let obj = NumArray::new(nums);
 * let ret_1: i32 = obj.sum_range(left, right);
 */

// Runtime: 11 ms, Beats 13.64%
// Memory: 8.47 MB, Beats 87.88%
