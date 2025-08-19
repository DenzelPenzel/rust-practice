fn main() {
    const N_THREADS: usize = 8;
    let to_add: Vec<u32> = (0..5000).collect();
    let chunks = to_add.chunks(N_THREADS);

    let mut threads_handles = Vec::new();
    for chunk in chunks {
        let my_chunk = chunk.to_owned();
        threads_handles.push(std::thread::spawn(move || my_chunk.iter().sum::<u32>()))
    }

    let mut sum: u32 = 0;
    for handle in threads_handles {
        sum += handle.join().unwrap();
    }

    println!("Total: {}", sum);
}
