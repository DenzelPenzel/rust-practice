use once_cell::sync::Lazy;
use tokio::sync::Mutex;

static SHARED_DATA: Lazy<Mutex<u32>> = Lazy::new(|| Mutex::new(0));

async fn add_one(n: u32) -> u32 {
    n + 1
}

async fn increment() {
    let mut counter = SHARED_DATA.lock().await;
    *counter = add_one(*counter).await;
}

#[tokio::main]
async fn main() {
    tokio::join!(increment(), increment(), increment());
    println!("Result: {}", *SHARED_DATA.lock().await);
}
