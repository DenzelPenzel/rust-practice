use std::thread;

fn main() {
    const N_THREADS: usize = 4;
    let to_add: Vec<u32> = (0..5000).collect();
    let chunks = to_add.chunks(N_THREADS);

    let sum = thread::scope(|s| {
        let mut thread_handles = vec![];

        for chunk in chunks {
            thread_handles.push(s.spawn(move || chunk.iter().sum::<u32>()))
        }

        thread_handles
            .into_iter()
            .map(|h| h.join().unwrap())
            .sum::<u32>()
    });

    println!("Total sum {}", sum)
}
