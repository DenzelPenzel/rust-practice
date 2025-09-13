use std::rc::Rc;
use std::cell::RefCell;
use std::sync::{Arc, Mutex};

#[derive(Debug)]
struct Droppable(i32);

impl Droppable {
    fn new(val: i32) -> Self {
        Self(val)
    }
}

impl Drop for Droppable {
    fn drop(&mut self) {
        println!("Dropping Droppable, {}", self.0);
    }
}

fn move_me(x: Rc<RefCell<Droppable>>) {
    println!("Moving Droppable, {}", x.borrow().0);
}

fn move_me_arc(x: Arc<Droppable>) {
    println!("Moving Droppable, {}", x.0);
}

struct SharedData(String);

fn main() {
    // ref cell example
    let my_shared = Rc::new(RefCell::new(Droppable::new(42)));
    {
        println!("Initial value: {:?}", my_shared.borrow());
        let x1 = my_shared.clone();
        x1.borrow_mut().0 = 100;
        println!("Value after x1 modified: {:?}", my_shared.borrow());

        let x2 = my_shared.clone();
        x2.borrow_mut().0 = 200;
        println!("Value after x2 modified: {:?}", my_shared.borrow());

        let x3 = my_shared.clone();
        println!("x3: {:?}", x3);
    }
    move_me(my_shared.clone());

    println!("Final value: {:?}", my_shared.borrow());
    println!("Done_1");


    // arc example
    println!("================");

    let my_shared2 = Arc::new(Droppable::new(1));
    {
        let x1 = my_shared2.clone();
        println!("x1: {:?}", x1);
        let x2 = my_shared2.clone();
        println!("x2: {:?}", x2);
        let x3 = my_shared2.clone();
        println!("x3: {:?}", x3);
    }

    move_me_arc(my_shared2.clone());

    let mut thread = Vec::new();
    for i in 0..10 {
        let my_shared2_clone = my_shared2.clone();
        thread.push(std::thread::spawn(move || {
            println!("Thread {i} is working on {my_shared2_clone:?}");
        }));
    }

    for t in thread {
        t.join().unwrap();
    }

    println!("Done_2");

    // arc + mutex example
    println!("================");

    let shared_data = Arc::new(Mutex::new(SharedData("Hello".to_string())));
    let mut thread = Vec::new();
    for i in 0..10 {
        let my_shared_data = shared_data.clone();
        thread.push(std::thread::spawn(move || {
            let mut lock = my_shared_data.lock().unwrap();
            lock.0.push_str(&format!(" {i}"));
        }));
    }

    for t in thread {
        t.join().unwrap();
    }

    let data = shared_data.lock().unwrap();
    println!("{}", data.0);
}
