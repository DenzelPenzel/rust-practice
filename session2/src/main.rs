fn hello_thread(n: u32) {
    println!("Hello, world from thread! {n}");
}

fn do_math(i: u32) -> u32 {
    let mut x = i + 1;
    for _ in 0..10 {
        x *= 2;
    }
    x
}

fn main() {
    let mut thread_handlers = Vec::new();
    for i in 0..10 {
        let thread_handle = std::thread::spawn(move || do_math(i));
        thread_handlers.push(thread_handle);
    }

    thread_handlers
        .into_iter()
        .for_each(|h| println!("{}", h.join().unwrap()));
}
