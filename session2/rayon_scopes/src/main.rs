fn run() {
    println!("Running");
}

// Create a rayon thread pool with 4 threads.
// The `build()` method constructs the pool, and `unwrap()` will panic if it fails.
fn main() {
    let pool = rayon::ThreadPoolBuilder::new()
        .num_threads(4)
        .build()
        .unwrap();

    pool.join(run, run);

    // Spawn a single task onto the thread pool.
    // The `move` keyword is used to transfer ownership of any captured variables to the closure.
    //pool.spawn(move || println!("Hello from the main pool thread"));

    // A scope allows spawning tasks that can borrow from the current stack frame.
    // The scope will automatically wait for all spawned tasks to complete before exiting.
    // pool.scope(|scope| {
    //     // Loop 20 times to spawn 20 tasks.
    //     for i in 0..20 {
    //         // Spawn a task within the scope.
    //         // `move |_|` is a closure that takes the scope as an argument (which we ignore with `_`)
    //         // and moves the `i` variable into the closure's environment.
    //         scope.spawn(move |_| {
    //             println!("Hello from thread {i}");
    //         });
    //     }
    // });
    

    pool.scope(|scope| {
        scope.spawn_broadcast(|_score, context| {
            println!("Hello from thread {}", context.index());
        });
    });


    println!("Done");
}
