fn parkable_thread(n: u32) {
    loop {
        std::thread::park();
        println!("Thread {n} is unparked");
    }
}

pub fn read_line() -> String {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

fn main() {
    let mut threads = Vec::new();

    for i in 0..10 {
        let thread_handle = std::thread::spawn(move || {
            parkable_thread(i);
        });
        threads.push(thread_handle);
    }

    loop {
        println!("Thread to unpark (0-9) or q to quit");
        let input = read_line();
        if input == "q" {
            break;
        }

        if let Ok(number) = input.parse::<usize>() {
            if number < threads.len() {
                threads[number].thread().unpark();
            }
        }
    }
}
