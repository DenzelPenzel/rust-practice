use std::collections::HashMap;
use std::ptr::NonNull;
use std::sync::{Arc, Mutex};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

#[derive(Clone)]
struct CacheEntity<T> {
    key: String,
    value: Arc<T>,
    expire: u128,
    prev: Option<NonNull<Self>>,
    next: Option<NonNull<Self>>,
    exp_prev: Option<NonNull<Self>>,
    exp_next: Option<NonNull<Self>>,
}

pub struct LocalCache<T>(Mutex<InnerLocalCache<T>>);

struct InnerLocalCache<T> {
    max_numbers: usize,
    max_age_ns: u128,
    lru_head: Option<NonNull<CacheEntity<T>>>,
    lru_tail: Option<NonNull<CacheEntity<T>>>,
    exp_head: Option<NonNull<CacheEntity<T>>>,
    exp_tail: Option<NonNull<CacheEntity<T>>>,
    map: HashMap<String, NonNull<CacheEntity<T>>>,
}

impl<T> InnerLocalCache<T> {
    fn new(max_numbers: usize, max_age_ns: u128) -> Self {
        Self {
            max_numbers,
            max_age_ns,
            lru_head: None,
            lru_tail: None,
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

        if now > entity.expire {
            return None;
        }

        self.remove_lru(non_null.clone());

        if self.lru_head.is_some() {
            let mut old_lru_head = self.lru_head.unwrap();
            old_lru_head.as_mut().prev = Some(non_null.clone());
            entity.next = self.lru_head.clone();
        }

        if self.lru_tail.is_some() {
            self.lru_tail = Some(non_null.clone());
        }

        self.lru_head = Some(non_null);

        Some(entity.value.clone())
    }

    unsafe fn remove_lru(&mut self, mut non_null: NonNull<CacheEntity<T>>) {
        let entity = non_null.as_mut();
        let key = &entity.key;
        entity.prev.clone().inspect(|prev| {
            prev.clone().as_mut().next = entity.next.clone();
        });
        entity.next.clone().inspect(|next| {
            next.clone().as_mut().prev = entity.prev.clone();
        });

        if let Some(lru_head) = self.lru_head.clone() {
            if &lru_head.as_ref().key == key {
                self.lru_head = entity.next.clone();
            }
        }

        if let Some(lru_tail) = self.lru_tail.clone() {
            if &lru_tail.as_ref().key == key {
                self.lru_tail = entity.prev.clone();
            }
        }
    }
}

fn main() {
    println!("Hello, world!");
}
