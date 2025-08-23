//! # A Thread-Safe, Performance-Oriented Local Cache
//!
//! This module implements a thread-safe local cache with both size-based (LRU - Least Recently Used)
//! and time-based (TTL - Time To Live) eviction policies. It is designed for high performance,
//! which has led to the use of `unsafe` Rust for manual memory management of a doubly-linked list.
//!
//! ## Core Data Structures
//!
//! The cache is built around a few core components:
//!
//! *   **`HashMap<String, NonNull<CacheEntity<T>>>`**: This provides O(1) average time complexity
//!     for lookups, insertions, and deletions of cache entries by their key. The `NonNull` is a
//!     raw, non-null pointer to a `CacheEntity` on the heap.
//!
//! *   **Two Doubly-Linked Lists**: Instead of storing values directly, the `HashMap` points to nodes
//!     in a doubly-linked list. This is a common pattern for implementing an LRU cache. This implementation
//!     uniquely uses *two* separate doubly-linked lists, using pointers within the same `CacheEntity` struct:
//!     1.  **LRU List (`prev`, `next`)**: This list tracks the usage order of items. When an item is
//!         accessed (`get`) or added (`put`), it's moved to the head of this list. When the cache is full,
//!         the item at the tail (the least recently used) is evicted.
//!     2.  **Expiration List (`exp_prev`, `exp_next`)**: This list keeps items sorted by their
//!         expiration time. This allows for efficient cleanup of expired items without iterating
//!         through all entries. New entries are added to the head of this list, and since they all get the same
//!         max age, the tail will contain the items that will expire first.
//!
//! *   **`CacheEntity<T>`**: This struct represents a single entry in the cache. It holds the key, value,
//!     expiration timestamp, and the pointers (`prev`, `next`, `exp_prev`, `exp_next`) for both
//!     linked lists. The value is wrapped in an `Arc<T>` to allow for cheap cloning and sharing
//!     of the cached data across threads without deep copying.
//!
//! ## Why `unsafe`?
//!
//! A doubly-linked list is a classic data structure that is tricky to implement in safe Rust
//! due to the strict ownership and borrowing rules. Each node needs to be owned by the list, but also have
//! mutable references from its neighbors (previous and next nodes). This multiple-mutable-references scenario
//! is what Rust's borrow checker is designed to prevent.
//!
//! Using `unsafe` with raw pointers (`NonNull`) allows us to bypass these compile-time checks.
//! This gives us the freedom to implement C-style pointers, which is efficient but also dangerous.
//! We are telling the compiler, "I know what I'm doing, and I will uphold Rust's memory safety
//! guarantees myself."
//!
//! The key invariants we must manually uphold are:
//! - All pointers in the `map` and in the linked list `next`/`prev` fields are valid and point to heap-allocated `CacheEntity` objects.
//! - There are no dangling pointers. When an entity is removed, we must ensure all pointers to it are also removed.
//! - We must correctly deallocate the memory for a `CacheEntity` exactly once when it's removed from the cache, using `Box::from_raw`.
//! - We must prevent data races. All access to the internal data structures is guarded by a `Mutex`.
//!
//! While `unsafe` can yield performance benefits, it's crucial to encapsulate it within a safe API,
//! which this implementation does with the `LocalCache` struct. For many use cases, a fully safe
//! implementation using `Rc<RefCell<...>>` (for single-threaded) or `Arc<Mutex<...>>` (for multi-threaded)
//! to manage list nodes, or using a battle-tested crate from `crates.io`, would be preferable.
//!
use std::collections::HashMap;
use std::ptr::NonNull;
use std::sync::{Arc, Mutex};
use std::time::{Duration, SystemTime, UNIX_EPOCH};


#[derive(Clone)]
struct CacheEntity<T> {
    key: String,
    value: Arc<T>,
    exp: u128,
    prev: Option<NonNull<Self>>,
    next: Option<NonNull<Self>>,
    exp_prev: Option<NonNull<Self>>,
    exp_next: Option<NonNull<Self>>,
}

pub struct LocalCache<T>(Mutex<InnerLocalCache<T>>);

struct InnerLocalCache<T> {
    max_numbers: usize,
    max_age_ns: u128,
    head: Option<NonNull<CacheEntity<T>>>,
    tail: Option<NonNull<CacheEntity<T>>>,
    exp_head: Option<NonNull<CacheEntity<T>>>,
    exp_tail: Option<NonNull<CacheEntity<T>>>,
    map: HashMap<String, NonNull<CacheEntity<T>>>,
}

impl<T> InnerLocalCache<T> {
    fn new(max_numbers: usize, max_age_ns: u128) -> Self {
        Self {
            max_numbers,
            max_age_ns,
            head: None,
            tail: None,
            exp_head: None,
            exp_tail: None,
            map: Default::default(),
        }
    }

    unsafe fn get(&mut self, key: &String) -> Option<Arc<T>> {
        let value = self.map.get(key);
        if value.is_none() {
            return None;
        }

        let mut non_null = value.unwrap().clone();
        let entity = non_null.as_mut();

        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        if now > entity.exp {
            return None;
        }

        self.remove_lru(non_null.clone());
        if self.head.is_some() {
            let mut old_head = self.head.clone().unwrap();
            old_head.as_mut().prev = Some(non_null.clone());
            entity.next = self.head.clone();
        }

        if self.tail.is_none() {
            self.tail = Some(non_null.clone());
        }

        self.head = Some(non_null);

        Some(entity.value.clone())
    }

    unsafe fn put(&mut self, key: String, value: Arc<T>) {
        self.remove(&key);

        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        self.clean(now);

        let exp = now + self.max_age_ns;

        let new_entity = Box::new(CacheEntity {
            key: key.clone(),
            value,
            exp,
            prev: None,
            next: self.head.clone(),
            exp_prev: self.exp_tail.clone(),
            exp_next: None,
        });

        let mut cur_entity = NonNull::from(Box::leak(new_entity));

        let _ = self.map.insert(key, cur_entity.clone());

        let old_lru_head = self.head.replace(cur_entity.clone());
        let old_exp_head = self.exp_head.replace(cur_entity.clone());

        match self.map.len() {
            0 | 1 => {
                let _ = self.tail.replace(cur_entity.clone());
                let _ = self.exp_tail.replace(cur_entity);
                return;
            }
            _ => {
                let mut old_lru_head = old_lru_head.unwrap();
                let inner_old_lru_head = old_lru_head.as_mut();
                inner_old_lru_head.prev = Some(cur_entity.clone());
                cur_entity.as_mut().next = Some(old_lru_head.clone());

                let mut old_exp_head = old_exp_head.unwrap();
                let inner_old_exp_head = old_exp_head.as_mut();
                inner_old_exp_head.exp_prev = Some(cur_entity.clone());
                cur_entity.as_mut().exp_next = Some(old_exp_head.clone());
            }
        }
    }

    unsafe fn clean(&mut self, now: u128) {
        if self.map.len() < self.max_numbers {
            return;
        }

        let mut cur = self.exp_tail.clone();
        while cur.is_some() {
            let e = cur.unwrap();
            let b = e.as_ref();
            if b.exp > now {
                break;
            }
            self.remove(&b.key);
            cur = b.exp_prev.clone();
        }

        while self.map.len() >= self.max_numbers {
            let key = self.tail.map(|e| e.as_ref().key.clone()).unwrap();
            self.remove(&key);
        }
    }

    unsafe fn remove(&mut self, key: &String) {
        let old = self.map.remove(key);
        if old.is_none() {
            return;
        }

        let old = old.unwrap();
        self.remove_lru(old.clone());
        self.remove_exp(old.clone());
        let _ = Box::from_raw(old.as_ptr());
    }

    unsafe fn remove_lru(&mut self, mut non_null: NonNull<CacheEntity<T>>) {
        let entity = non_null.as_mut();
        let key = &entity.key;

        entity.prev.clone().inspect(|e| {
            e.clone().as_mut().next = entity.next.clone();
        });

        entity.next.clone().inspect(|e| {
            e.clone().as_mut().prev = entity.prev.clone();
        });

        if let Some(head) = self.head.clone() {
            if &head.as_ref().key == key {
                self.head = entity.next.clone();
            }
        }

        if let Some(tail) = self.tail.clone() {
            if &tail.as_ref().key == key {
                self.tail = entity.prev.clone();
            }
        }
    }

    unsafe fn remove_exp(&mut self, mut non_null: NonNull<CacheEntity<T>>) {
        let entity = non_null.as_mut();
        let key = &entity.key;

        entity.exp_prev.clone().inspect(|e| {
            e.clone().as_mut().exp_next = entity.exp_next.clone();
        });

        entity
            .exp_next
            .clone()
            .inspect(|e| e.clone().as_mut().exp_prev = entity.exp_prev.clone());

        if let Some(exp_head) = self.exp_head.clone() {
            if &exp_head.as_ref().key == key {
                self.exp_head = entity.exp_next.clone();
            }
        }

        if let Some(exp_tail) = self.exp_tail.clone() {
            if &exp_tail.as_ref().key == key {
                self.exp_tail = entity.exp_prev.clone();
            }
        }
    }
}

impl<T> LocalCache<T> {
    pub fn new(max_numbers: usize, max_age_secs: u64) -> Self {
        Self(Mutex::new(InnerLocalCache::new(
            max_numbers,
            Duration::from_secs(max_age_secs).as_nanos(),
        )))
    }

    pub fn get(&self, key: &String) -> Option<Arc<T>> {
        let mut local_cache = self.0.lock().unwrap();
        unsafe { local_cache.get(key) }
    }

    pub fn put(&self, key: String, value: Arc<T>) {
        let mut local_cache = self.0.lock().unwrap();
        unsafe { local_cache.put(key, value) }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;
    use std::time::Duration;

    #[test]
    fn test_put_and_get() {
        let cache = LocalCache::<String>::new(10, 3600);
        let key = "key1".to_string();
        let value = Arc::new("value1".to_string());
        cache.put(key.clone(), value.clone());
        let retrieved = cache.get(&key);
        assert!(retrieved.is_some());
        assert_eq!(*retrieved.unwrap(), *value);
    }

    #[test]
    fn test_get_non_existent() {
        let cache = LocalCache::<String>::new(10, 3600);
        let key = "key1".to_string();
        let retrieved = cache.get(&key);
        assert!(retrieved.is_none());
    }

    #[test]
    fn test_update_value() {
        let cache = LocalCache::<String>::new(10, 3600);
        let key = "key1".to_string();
        let value1 = Arc::new("value1".to_string());
        let value2 = Arc::new("value2".to_string());
        cache.put(key.clone(), value1.clone());
        let retrieved1 = cache.get(&key);
        assert_eq!(*retrieved1.unwrap(), *value1);

        cache.put(key.clone(), value2.clone());
        let retrieved2 = cache.get(&key);
        assert_eq!(*retrieved2.unwrap(), *value2);
    }

    #[test]
    fn test_lru_eviction() {
        let cache = LocalCache::<String>::new(2, 3600);
        let key1 = "key1".to_string();
        let value1 = Arc::new("value1".to_string());
        let key2 = "key2".to_string();
        let value2 = Arc::new("value2".to_string());
        let key3 = "key3".to_string();
        let value3 = Arc::new("value3".to_string());

        cache.put(key1.clone(), value1.clone());
        cache.put(key2.clone(), value2.clone());

        // Access key1 to make it most recently used
        cache.get(&key1);

        cache.put(key3.clone(), value3.clone());

        assert!(cache.get(&key1).is_some());
        assert!(cache.get(&key3).is_some());
        assert!(cache.get(&key2).is_none()); // key2 should be evicted
    }

    #[test]
    fn test_lru_eviction_no_get() {
        let cache = LocalCache::<String>::new(2, 3600);
        let key1 = "key1".to_string();
        let value1 = Arc::new("value1".to_string());
        let key2 = "key2".to_string();
        let value2 = Arc::new("value2".to_string());
        let key3 = "key3".to_string();
        let value3 = Arc::new("value3".to_string());

        cache.put(key1.clone(), value1.clone());
        cache.put(key2.clone(), value2.clone());

        // No get, so key1 is the least recently used
        cache.put(key3.clone(), value3.clone());

        assert!(cache.get(&key1).is_none()); // key1 should be evicted
        assert!(cache.get(&key2).is_some());
        assert!(cache.get(&key3).is_some());
    }

    #[test]
    fn test_time_expiration() {
        let cache = LocalCache::<String>::new(10, 1); // 1 second expiry
        let key = "key1".to_string();
        let value = Arc::new("value1".to_string());

        cache.put(key.clone(), value.clone());
        assert!(cache.get(&key).is_some());

        thread::sleep(Duration::from_secs(2));

        // The value should be expired, but it's only cleaned on `put`.
        // `get` on an expired value should return None.
        assert!(cache.get(&key).is_none());
    }

    #[test]
    fn test_put_triggers_cleaning() {
        let cache = LocalCache::<String>::new(2, 1); // max 2 items, 1 sec expiry
        let key1 = "key1".to_string();
        let value1 = Arc::new("value1".to_string());
        let key2 = "key2".to_string();
        let value2 = Arc::new("value2".to_string());

        cache.put(key1.clone(), value1.clone()); // k1 is LRU

        thread::sleep(Duration::from_secs(2)); // k1 expires

        cache.put(key2.clone(), value2.clone()); // k2 is added, k1 should be cleaned

        // Now add another item. If k1 was cleaned, k2 should not be evicted.
        let key3 = "key3".to_string();
        let value3 = Arc::new("value3".to_string());
        cache.put(key3.clone(), value3.clone());

        assert!(cache.get(&key1).is_none());
        assert!(cache.get(&key2).is_some()); // k2 should still be there
        assert!(cache.get(&key3).is_some());
    }
}

fn main() {
    // This is a dummy main function to satisfy the compiler for a binary crate.
    // You can use this space to add examples of how to use your LocalCache.
    println!("LocalCache binary entry point. Run tests with `cargo test`.");
}
