use std::sync::Mutex;

static MY_SHARED: Mutex<u32> = Mutex::new(3);

fn poisoner() {
    let mut lock = MY_SHARED.lock().unwrap();
    *lock += 1;
    panic!("The poisoner is here");
}

fn main() {
    //let var_shared = Mutex::new(0);

    // let lock = MY_SHARED.lock().unwrap();

    // std::mem::drop(lock);

    // if let Ok(_lock) = MY_SHARED.try_lock() {
    //     println!("Got the lock")
    // } else {
    //     println!("Could not get the lock")
    // }

    let handle = std::thread::spawn(poisoner);
    println!("Trying to return from thread");
    println!("{:?}", handle.join());

    let lock = MY_SHARED.lock();
    println!("{lock:?}");

    let _recovered_data = lock.unwrap_or_else(|p| {
        println!("Mutex was poisoned, recovering data...");
        p.into_inner()
    });
}
