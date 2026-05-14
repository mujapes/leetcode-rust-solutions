struct MinStack {
    min_data: Vec<i32>,
    data: Vec<i32>
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MinStack {

    fn new() -> Self {
        Self {
            min_data: Vec::new(),
            data: Vec::new()
        }
    }
    
    fn push(&mut self, val: i32) {
        if val <= self.get_min() { self.min_data.push(val); }
        self.data.push(val);
    }
    
    fn pop(&mut self) {
        if self.top() == self.get_min() { self.min_data.pop(); }
        self.data.pop().unwrap();
    }
    
    fn top(&self) -> i32 {
        *self.data.last().unwrap()
    }
    
    fn get_min(&self) -> i32 {
        // track min element, if it gets popped need a fallback
        // separate stack storing min, only needs to be pushed to when element lower than current min is pushed
        *self.min_data.last().unwrap_or(&i32::MAX)
    }
}

/**
 * Your MinStack object will be instantiated and called as such:
 * let obj = MinStack::new();
 * obj.push(val);
 * obj.pop();
 * let ret_3: i32 = obj.top();
 * let ret_4: i32 = obj.get_min();
 */

// Runtime: 1 ms, Beats 69.92%
// Memory: 6.00 MB, Beats 63.16%
