use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::mem::MaybeUninit;

const PAGE_SIZE: usize = size_of::<u64>() * 8;

type LRUCache = LruCache;

pub struct LruCache {
    capacity: i32,
    len: i32,
    map: HashMap<i32, usize>,
    log: SparseRing<(i32, i32)>,
}

impl LRUCache {
    pub fn new(capacity: i32) -> Self {
        Self {
            capacity,
            len: 0,
            map: HashMap::with_capacity(capacity as usize),
            log: SparseRing::new((capacity * 20) as usize),
        }
    }

    pub fn get(&mut self, key: i32) -> i32 {
        self.compress();
        
        match self.map.entry(key) {
            Entry::Occupied(mut entry) => {
                let idx = *entry.get();
                let (k, v) = self.log.remove(idx).unwrap();
                let new_index = self.log.push_back((k, v)).unwrap();
                entry.insert(new_index);
                v
            }
            Entry::Vacant(_) => -1
        }
    }

    pub fn put(&mut self, key: i32, value: i32) {
        self.compress();

        match self.map.entry(key) {
            Entry::Occupied(mut entry) => {
                self.log.remove(*entry.get()).unwrap();
                
                let new_entry = self.log.push_back((key, value)).unwrap();
                entry.insert(new_entry);
            }
            Entry::Vacant(entry) if self.len >= self.capacity => {
                let (k, _) = self.log.pop_front().unwrap();

                let new_entry = self.log.push_back((key, value)).unwrap();
                entry.insert(new_entry);

                self.map.remove(&k);
            }
            Entry::Vacant(entry) => {
                self.len += 1;
                let index = self.log.push_back((key, value)).unwrap();
                entry.insert(index);
            }
        }

    }

    fn compress(&mut self) {
        if self.log.is_full() {
            self.log.compact();
            for idx in 0..self.log.len() {
                let (key, _) = self.log.get(idx).unwrap();
                self.map.insert(key, idx);
            }
        }

    }
}

struct SparseRing<T: Clone> {
    buffer: Vec<MaybeUninit<T>>,
    bitmap: Bitmap,
    offset: usize,
    len: usize,
    capacity: usize,
}

impl <T: Clone> SparseRing<T> {
    pub fn new(capacity: usize) -> Self {
        let tot = (capacity + PAGE_SIZE - 1) / PAGE_SIZE * PAGE_SIZE;

        let buffer = unsafe {
            let mut buffer = Vec::with_capacity(tot);
            buffer.set_len(tot);
            buffer
        };

        Self { 
            buffer: buffer,
            bitmap: Bitmap::new(tot), 
            offset: 0, 
            len: 0, 
            capacity: tot,
        }
    }

    pub fn push_back(&mut self, val: T) -> Option<usize> {
        if self.len >= self.buffer.len() {
            return None;
        }

        let i = self.physical_index(self.len);
        self.set(i, val);
        self.len += 1;
        Some(i)
    }

    pub fn pop_front(&mut self) -> Option<T> {
        self.remove(self.offset)
    }

    pub fn remove(&mut self, idx: usize) -> Option<T> {
        let removed_item = self.get(idx);
        self.bitmap.set(idx, false);
        self.advance();
        removed_item
    }

    pub fn is_full(&self) -> bool {
        self.len == self.capacity
    }

    pub fn len(&self) -> usize {
        self.len
    }

    fn set(&mut self, idx: usize, val: T) {
        self.bitmap.set(idx, true);
        self.buffer[idx] = MaybeUninit::new(val);
    }

    fn get(&self, idx: usize) -> Option<T> {
        if self.bitmap.get(idx) {
            unsafe {
                Some(self.buffer[idx].assume_init_ref().clone())
            }
        } else {
            None
        }
    }

    fn advance(&mut self) {
        if self.bitmap.get(self.offset) {
            return;
        }

        let zeros = self.bitmap.zero_run(self.offset);
        if zeros != self.capacity {
            self.offset = (self.offset + zeros) % self.buffer.len();
            self.len = self.len - zeros;
        } else {
            self.offset = 0;
            self.len = 0;
        }
    }

    fn compact(&mut self) {
        let mut write = 0;
        for read in 0..self.capacity {
            let physical_read = self.physical_index(read);
            if let Some(val) = self.get(physical_read) {
                let physical_write = self.physical_index(write);
                self.set(physical_write, val);
                write += 1;
            }
        }

        self.buffer.rotate_left(self.offset);
        self.offset = 0;
        self.len = write;
        self.bitmap.refill(write);
    }

    fn physical_index(&self, idx: usize) -> usize {
        (idx + self.offset) % self.capacity
    }

}


pub struct Bitmap {
    pages: Vec<u64>,
    capacity: usize,
}

impl Bitmap {
    pub fn new(capacity: usize) -> Self {
        let pages_num = (capacity + PAGE_SIZE - 1) / PAGE_SIZE;
        Self {
            pages: vec![0; pages_num],
            capacity,
        }
    }
    
    pub fn clear(&mut self) {
        for page in &mut self.pages {
            *page = 0;
        }
    }
    
    pub fn fill(&mut self, idx: usize) {
        self.border_check(idx - 1);
        let (full_pages, remaining_bits) =  (idx / PAGE_SIZE, idx % PAGE_SIZE);
        self.pages[..full_pages].fill(u64::MAX);
        if remaining_bits > 0 {
            self.pages[full_pages] = (1 << remaining_bits) - 1;
        }
    }
    
    pub fn refill(&mut self, n: usize) {
        self.clear();
        self.fill(n);
    }

    // First `n` bits are set to 1, the rest are 0
    pub fn filled(idx: usize, capacity: usize) -> Self {
        let mut bitmap = Self::new(capacity);
        if idx == 0 {
            return bitmap;
        }
        bitmap.border_check(idx - 1); 

        let (full_pages, remaining_bits) =  (idx / PAGE_SIZE, idx % PAGE_SIZE);

        bitmap.pages[..full_pages].fill(u64::MAX);

        if remaining_bits > 0 {
            bitmap.pages[full_pages] = (1 << remaining_bits) - 1;
        }

        bitmap
    }

    pub fn get(&self, idx: usize) -> bool {
        self.border_check(idx);
        let (page_idx, bit_idx) = (idx / PAGE_SIZE, idx % PAGE_SIZE);
        self.pages[page_idx] & (1 << bit_idx) != 0
    }

    pub fn set(&mut self, idx: usize, value: bool) {
        self.border_check(idx);
        let (page_idx, bit_idx) = (idx / PAGE_SIZE, idx % PAGE_SIZE);
        let page = &mut self.pages[page_idx];
        
        if value {
            *page |= 1 << bit_idx;
        } else {
            *page &= !(1 << bit_idx);
        }
    }

    fn page_bit(idx: usize) -> (usize, usize) {
        (idx / PAGE_SIZE, idx % PAGE_SIZE)
    }

    fn border_check(&self, idx: usize)  {
        if idx >= self.capacity {
            panic!("Index out of bounds for bitmap");
        }
    }

    // Count the number of bits set to 0
    pub fn zero_run(&self, idx: usize) -> usize {
        self.border_check(idx);

        let (page_idx, bit_idx) =  (idx / PAGE_SIZE, idx % PAGE_SIZE);
        let mut zeros = 0usize;

        // First page
        let page = self.pages[page_idx];
        let mask = page >> bit_idx;
        if mask == 0 {
            zeros += PAGE_SIZE - bit_idx;
        } else {
            return mask.trailing_zeros() as usize;
        }
        
        
        let wrapped_pages = self.pages[page_idx + 1..]
            .iter()
            .chain(&self.pages[..=page_idx]);

        for page in wrapped_pages {
            if *page == 0 {
                zeros += PAGE_SIZE;
            } else {
                zeros += page.trailing_zeros() as usize;
                break
            }
        }

        zeros.min(self.capacity) 
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
        assert_eq!(cache.get(2), 6);
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