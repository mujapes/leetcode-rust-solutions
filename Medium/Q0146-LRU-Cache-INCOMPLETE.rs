use std::collections::HashMap;

// get O(1) runtime: hashmap
// put O(1) runtime:
// if no gets then could use linked list, with pointer to end
// but pointer to end would need to bubble up with pops so doubly linked?
// doubly linked allows O(1) put by stitching last to next

struct DoublyLinkedNode {
    value: i32,
    // key identifier
    newer: Option<i32>,
    older: Option<i32>
}

struct LRUCache {
    capacity: i32,
    len: i32,
    // doubly linked hashmap
    data: HashMap<i32, DoublyLinkedNode>,
    // key identifiers
    newest: Option<i32>,
    oldest: Option<i32>
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl LRUCache {

    fn new(capacity: i32) -> Self {
        Self {
            capacity,
            len: 0,
            data: HashMap::with_capacity(capacity as usize),
            newest: None,
            oldest: None
        }
    }
    
    fn get(&mut self, key: i32) -> i32 {
        let Some(cur_node) = self.data.get_mut(&key) else { return -1 };
        let cur_value = cur_node.value;
        let cur_newer = cur_node.newer;
        let cur_older = cur_node.older;

        cur_node.newer = None;
        cur_node.older = self.newest;

        if let Some(newer_key) = cur_newer {
            self.data
                .get_mut(&newer_key)
                .unwrap()
                .older = cur_older;
        }
        if let Some(older_key) = cur_older {   
            self.data
                .get_mut(&older_key)
                .unwrap()
                .newer = cur_newer;
        }
        if let Some(newest_key) = self.newest {
            self.data
                .get_mut(&newest_key)
                .unwrap()
                .newer = Some(key);
        }
        self.newest = Some(key);

        cur_value
    }
    
    fn put(&mut self, key: i32, value: i32) {
        self.data.insert(
            key, 
            DoublyLinkedNode {
                value,
                newer: None,
                older: self.newest
            }
        );
        if let Some(next_newest_key) = self.newest {
            self.data
                .get_mut(&next_newest_key)
                .unwrap()
                .newer = Some(key);
        }
        self.newest = Some(key);

        if self.len == self.capacity {
            let next_oldest = self.data
                .get(&self.oldest.unwrap())
                .unwrap()
                .newer;

            self.data.remove(&self.oldest.unwrap());
            self.oldest = next_oldest;

            self.data
                .get_mut(&next_oldest.unwrap())
                .unwrap()
                .older = None;
        } else {
            if self.len == 0 { self.oldest = Some(key); }
            self.len += 1;
        }
    }
}

/**
 * Your LRUCache object will be instantiated and called as such:
 * let obj = LRUCache::new(capacity);
 * let ret_1: i32 = obj.get(key);
 * obj.put(key, value);
 */
