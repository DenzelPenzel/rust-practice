use algos::lru::LRUCache;


fn main() {
    let mut cache = LRUCache::new(2);
    cache.put(1, 1);
    println!("Value {}", cache.get(1));
}
