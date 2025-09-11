
use std::time::Duration;

async fn do_work() {
    tokio::time::sleep(Duration::from_secs(2)).await;
}

async fn timeout(secs: f32) {
    tokio::time::sleep(Duration::from_secs_f32(secs)).await;
}

#[tokio::main]
async fn main() {
    tokio::select! {
        _ = do_work() => {
            println!("Work done finished first");
        }
        _ = timeout(3.0) => {
            println!("Timeout finished first");
        }
    }
}
