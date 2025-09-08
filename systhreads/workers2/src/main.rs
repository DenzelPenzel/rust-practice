use once_cell::sync::Lazy;
use std::{collections::VecDeque, sync::Mutex, sync::mpsc, time::Duration};

static WORK_QUEUE: Lazy<Mutex<VecDeque<String>>> = Lazy::new(|| Mutex::new(VecDeque::new()));

fn main() {
    let cpu_count = 2; //num_cpus::get();
    let mut threads = Vec::with_capacity(cpu_count);
    let mut broadcast = Vec::with_capacity(cpu_count);

    for cpu in 0..cpu_count {
        let (tx, rx) = mpsc::channel::<()>();
        broadcast.push(tx);

        let thread = std::thread::spawn(move || {
            while rx.recv().is_ok() {
                let mut lock = WORK_QUEUE.lock().unwrap();

                if let Some(work) = lock.pop_front() {
                    std::mem::drop(lock);
                    println!("CPU {cpu} is working on {work}");
                    std::thread::sleep(Duration::from_secs(3));
                    println!("CPU {cpu} is done with {work}");
                } else {
                    println!("CPU {cpu} is waiting for work");
                }
            }
        });

        threads.push(thread);
    }

    loop {
        let sent = {
            let mut lock = WORK_QUEUE.lock().unwrap();
            let len = lock.len();
            println!("Work queue length: {len}");

            if len < 5 {
                lock.push_back("Hello".to_string());
                true
            } else {
                false
            }
        };

        if sent {
            broadcast.iter().for_each(|tx| tx.send(()).unwrap());
        }

        std::thread::sleep(Duration::from_secs(1));
    }
}
