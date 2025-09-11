use std::time::Duration;
use anyhow::Ok;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{TcpListener, TcpStream},
    spawn,
};

async fn tcp_client() -> anyhow::Result<()> {
    let mut stream = TcpStream::connect("127.0.0.1:8080").await?;
    println!("Connected to server");
    stream.write_all(b"hello world").await?;
    let mut buf = vec![0u8; 1024];
    let bytes_read = stream.read(&mut buf).await?;

    println!("Received from server: {}", String::from_utf8_lossy(&buf[0..bytes_read]));

    Ok(())
}

async fn client_runner() -> anyhow::Result<()> {
    loop {
        tokio::time::sleep(Duration::from_secs(1)).await;
        let _ = tcp_client().await;
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tokio::join!(client_runner());

    let listener = TcpListener::bind("127.0.0.1:8080").await?;

    loop {
        let (mut socket, addr) = listener.accept().await?;

        spawn(async move {
            println!("New connection from {addr}");
            let mut buf = vec![0; 1024];

            loop {
                let n = socket
                    .read(&mut buf)
                    .await
                    .expect("failed to read from socket");

                if n == 0 {
                    return;
                }

                socket
                    .write_all(&buf[0..n])
                    .await
                    .expect("failed to write to socket");
            }
        });
    }
}
