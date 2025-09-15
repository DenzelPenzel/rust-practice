#[tracing::instrument]
async fn hello() {
    println!("Hello");
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let subs = tracing_subscriber::FmtSubscriber::new();
    tracing::subscriber::set_global_default(subs)?;

    tracing::info!("Starting up");

    hello().await;

    Ok(())
}
