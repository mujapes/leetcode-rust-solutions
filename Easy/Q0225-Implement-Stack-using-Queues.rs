struct MyQueue {
    elements: Vec<i32>
}

impl MyQueue {
    fn new() -> Self {
        MyQueue {elements: Vec::new()}
    }
    
    fn push_to_back(&mut self, x: i32) {
        self.elements.push(x);
    }
    
    fn pop_from_front(&mut self) -> i32 {
        let res = self.elements[0];
        self.elements.remove(0);
        res
    }
    
    fn peek_from_front(&self) -> i32 {
        self.elements[0]
    }

    fn size(&self) -> i32 {
        self.elements.len() as i32
    }
    
    fn is_empty(&self) -> bool {
        self.elements.len() == 0
    }
}

struct MyStack {
    queue: MyQueue
}

/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyStack {

    fn new() -> Self {
        MyStack {
            queue: MyQueue::new()
        }
    }
    
    fn push(&mut self, x: i32) {
        let mut res = MyQueue::new();
        res.push_to_back(x);
        while !self.queue.is_empty() {
            res.push_to_back(self.queue.pop_from_front());
        }
        self.queue = res;
    }
    
    fn pop(&mut self) -> i32 {
        self.queue.pop_from_front()
    }
    
    fn top(&self) -> i32 {
        self.queue.peek_from_front()
    }
    
    fn empty(&self) -> bool {
        self.queue.is_empty()
    }
}

/**
 * Your MyStack object will be instantiated and called as such:
 * let obj = MyStack::new();
 * obj.push(x);
 * let ret_2: i32 = obj.pop();
 * let ret_3: i32 = obj.top();
 * let ret_4: bool = obj.empty();
 */

// Runtime: 0ms, Beats 100.00%
// Memory: 2.16 MB, Beats 88.68%
