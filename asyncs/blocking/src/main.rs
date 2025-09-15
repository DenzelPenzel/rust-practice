use std::time::Duration;
use tokio::task::spawn_blocking;

async fn fn1(task: u64, time: u64) {
    println!("Task {task} started");
    let _ = spawn_blocking(move || {
        std::thread::sleep(Duration::from_millis(time));
    })
    .await;
    // std::thread::sleep(Duration::from_millis(time));
    // tokio::time::sleep(Duration::from_millis(time)).await;
    println!("Task {task} finished");
}

#[tokio::main]
async fn main() {
    tokio::join!(fn1(1, 500), fn1(2, 1000), fn1(3, 1500),);
}
