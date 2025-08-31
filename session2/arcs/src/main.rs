use std::sync::{Arc, Mutex};

fn main() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = Vec::new();

    for _ in 0..10 {
        // Clone the Arc for each new thread
        let counter_clone = Arc::clone(&counter);

        let handle = std::thread::spawn(move || {
            let mut num = counter_clone.lock().unwrap();

            *num += 1;

            // When the closure ends here, the `num` variable goes out of scope.
            // The `MutexGuard`'s destructor is called automatically, which releases the lock.
            // There is no manual `unlock()` call needed.
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}
