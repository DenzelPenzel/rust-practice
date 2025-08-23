use dashmap::DashMap;
use once_cell::sync::Lazy;
use std::time::Duration;

static SHARED_DATA: Lazy<DashMap<u32, u32>> = Lazy::new(DashMap::new);

fn main() {
    for n in 0..100 {
        std::thread::spawn(move || {
            if let Some(mut value) = SHARED_DATA.get_mut(&n) {
                *value += 1;
            } else {
                SHARED_DATA.insert(n, n);
            }
        });
    }

    std::thread::sleep(Duration::from_secs(3));
    println!("{SHARED_DATA:#?}");
}
