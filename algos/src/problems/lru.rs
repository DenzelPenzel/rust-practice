use std::collections::HashMap;
use std::ptr;

#[derive(Clone)]
// heap-allocated
struct Node {
    key: i32,
    value: i32,
    prev: *mut Node,
    next: *mut Node,
}

pub struct LRUCache {
    capacity: usize,
    map: HashMap<i32, *mut Node>,

    head: *mut Node,
    tail: *mut Node,
}

impl LRUCache {
    pub fn new(capacity: i32) -> Self {
        // Allocate sentinel head and tail nodes on the heap
        let head = Box::into_raw(Box::new(Node {
            key: -1,
            value: -1,
            prev: ptr::null_mut(),
            next: ptr::null_mut(),
        }));

        let tail = Box::into_raw(Box::new(Node {
            key: -1,
            value: -1,
            prev: ptr::null_mut(),
            next: ptr::null_mut(),
        }));

        unsafe {
            (*head).next = tail;
            (*tail).prev = head;
        }

        LRUCache {
            capacity: capacity as usize,
            map: HashMap::with_capacity(capacity as usize),
            head: head,
            tail: tail,
        }
    }

    pub fn get(&mut self, key: i32) -> i32 {
        if let Some(&ptr) = self.map.get(&key) {
            unsafe {
                self.move_to_head(ptr);
                (*ptr).value
            }
        } else {
            -1
        }
    }

    pub fn put(&mut self, key: i32, value: i32) {
        if let Some(&ptr) = self.map.get(&key) {
            unsafe {
                (*ptr).value = value;
                self.move_to_head(ptr);
            }
        } else {
            if self.map.len() >= self.capacity {
                let node = unsafe { (*self.tail).prev };
                let lru_key = unsafe { (*node).key };

                self.map.remove(&lru_key);
                unsafe {
                    self.detach_node(node);
                    let _ = Box::from_raw(node);
                }
            }

            let new_node = Box::into_raw(Box::new(Node {
                key,
                value,
                prev: ptr::null_mut(),
                next: ptr::null_mut(),
            }));

            self.map.insert(key, new_node);
            unsafe {
                self.attach_to_head(new_node);
            }
        }
    }

    /// Unlinks a node from its current position in the list.
    unsafe fn detach_node(&mut self, node: *mut Node) {
        (*(*node).prev).next = (*node).next;
        (*(*node).next).prev = (*node).prev;
    }

    unsafe fn attach_to_head(&mut self, node: *mut Node) {
        (*node).next = (*self.head).next;
        (*node).prev = self.head;
        (*(*self.head).next).prev = node;
        (*self.head).next = node;
    }

    unsafe fn move_to_head(&mut self, node: *mut Node) {
        self.detach_node(node);
        self.attach_to_head(node);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lru_cache_case_1() {
        let mut cache = LRUCache::new(2);
        assert_eq!(cache.get(2), -1);
        cache.put(2, 6);
        assert_eq!(cache.get(1), -1);
        cache.put(1, 5);
        cache.put(1, 2);
        assert_eq!(cache.get(1), 2);
        assert_eq!(cache.get(2), 6);
    }

    #[test]
    fn test_lru_cache_case_2() {
        let mut cache = LRUCache::new(2);
        cache.put(1, 1);
        cache.put(2, 2);
        assert_eq!(cache.get(1), 1);
        cache.put(3, 0);
        assert_eq!(cache.get(2), -1);
        cache.put(4, 4);
        assert_eq!(cache.get(1), -1);
        assert_eq!(cache.get(3), 0);
        assert_eq!(cache.get(4), 4);
    }

    #[test]
    fn test_lru_cache_case_3() {
        let mut cache = LRUCache::new(2);
        cache.put(1, 1);
        cache.put(2, 2);
        assert_eq!(cache.get(1), 1);
        cache.put(3, 3);
        assert_eq!(cache.get(2), -1);
        cache.put(4, 4);
        assert_eq!(cache.get(1), -1);
        assert_eq!(cache.get(3), 3);
        assert_eq!(cache.get(4), 4);
    }
}
