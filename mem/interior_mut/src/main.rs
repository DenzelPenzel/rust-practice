use core::str;
use std::{
    cell::RefCell,
    sync::{Arc, Mutex},
};

struct SharedData {
    data: Mutex<String>,
}

impl SharedData {
    fn new(s: &str) -> Self {
        Self {
            data: Mutex::new(s.to_string()),
        }
    }
}

struct MyData {
    data: RefCell<String>,
}

impl MyData {
    fn new() -> Self {
        Self {
            data: RefCell::new("Hello".to_string()),
        }
    }
}

fn move_data(x: Arc<MyData>) {
    let mut data = x.data.borrow_mut();
    data.push_str(" World");
}

fn main() {
    let my_shared = Arc::new(SharedData::new("Hello"));
    let mut thread = Vec::new();

    for i in 0..10 {
        let my_shared_data = my_shared.clone();
        thread.push(std::thread::spawn(move || {
            let mut data = my_shared_data.data.lock().unwrap();
            data.push_str(&format!(" {i}"));
        }));
    }

    for t in thread {
        t.join().unwrap();
    }

    let data = my_shared.data.lock().unwrap();
    println!("{}", data);

    // ref cell example
    println!("================");

    let shared_data = Arc::new(MyData::new());
    move_data(shared_data.clone());
    let data = shared_data.data.borrow();
    println!("{}", data);
}
