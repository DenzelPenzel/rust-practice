use std::{io::{self, BufRead}, path::Path, fs::File};

use anyhow::Result;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

async fn read_count(filename: String) -> Result<usize> {
    let mut line_count = 0;
    
    if let Ok(lines) = read_lines(filename) {
       lines.for_each(|line| {
        if let Ok(line) = line {
            if !line.trim().is_empty() {
                line_count += 1;
            }
        }
       });
    }
    Ok(line_count)
}

async fn async_read_count(filename: String) -> Result<usize> {
    use tokio::io::AsyncBufReadExt;
    use tokio::io::BufReader;
    use tokio::fs::File;

    println!("Async reading file: {}", filename);
    let mut line_count = 0;
    let file = File::open(filename).await?;

    let reader = BufReader::new(file);
    let mut lines = reader.lines();

    while let Some(line) = lines.next_line().await? {
        if !line.trim().is_empty() {
            line_count += 1;
        }
    }

    Ok(line_count)
}

#[tokio::main]
async fn main() -> Result<()> {
    let now = std::time::Instant::now();
    let (c1, c2, ..) = tokio::join!(
        async_read_count("data.txt".to_string()),
        async_read_count("data.txt".to_string()),
        async_read_count("data.txt".to_string()),
        async_read_count("data.txt".to_string()),
        async_read_count("data.txt".to_string()),
        async_read_count("data.txt".to_string()),
    );
    println!("Total lines: {}", c1? + c2?);
    println!("Time taken: {:3} seconds", now.elapsed().as_secs_f32());
    Ok(())
}
