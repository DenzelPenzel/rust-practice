use std::time::Duration;
use tokio::sync::{broadcast, mpsc};

async fn receiver(mut rx: mpsc::Receiver<u32>, mut broadcast_rx: broadcast::Receiver<u32>) {
    loop {
        tokio::select! {
            Some(n) = rx.recv() => {
                println!("Received message from mpsc: {n}");
            },
            Ok(n) = broadcast_rx.recv() => {
                println!("Received message from broadcast: {n}");
            }
        }
    }
}

#[tokio::main]
async fn main() {
    let (tx, rx) = mpsc::channel::<u32>(16);
    let (tx_broadcast, rx_broadcast) = broadcast::channel::<u32>(1);

    tokio::spawn(receiver(rx, rx_broadcast));

    for i in 0..10 {
        if i % 2 == 0 {
            tx.send(i).await.unwrap();
        } else {
            tx_broadcast.send(i).unwrap();
        }
        tokio::time::sleep(Duration::from_secs(1)).await;
    }
}
