struct KthLargest {
    k: i32,
    nums: Vec<i32>,
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl KthLargest {

    fn new(k: i32, mut nums: Vec<i32>) -> Self {
        nums.sort_by(|a, b| b.cmp(a));
        nums.truncate(k as usize);
        Self { k, nums }
    }
    
    fn add(&mut self, val: i32) -> i32 {
        self.nums.push(val);
        self.nums.sort_by(|a, b| b.cmp(a));
        self.nums.truncate(self.k as usize);
        self.nums[self.nums.len() - 1]
    }
}

/**
 * Your KthLargest object will be instantiated and called as such:
 * let obj = KthLargest::new(k, nums);
 * let ret_1: i32 = obj.add(val);
 */

// Runtime: 383 ms, Beats 5.80%
// Memory: 6.98 MB, Beats 66.67%
