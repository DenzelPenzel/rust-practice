use std::time::Instant;
use rayon::prelude::*;

fn is_prime(n: u32) -> bool {
    (2 ..= n / 2).into_par_iter().all(|i| n % i != 0)
}

fn main() {
    let nums = (0..1_000_000).collect::<Vec<u64>>();
    let sum = nums.par_iter().sum::<u64>();
    println!("Sum: {}", sum);

    let now = Instant::now();
    let nums = (0..1000).collect::<Vec<u32>>();
    let mut primes: Vec<&u32> = nums.par_iter().filter(|i| is_prime(**i)).collect(); 
    primes.par_sort_unstable();

    let elapsed = now.elapsed().as_secs_f32();
    println!("Primes: {:?}", primes);
    println!("Time: {}", elapsed);
}
