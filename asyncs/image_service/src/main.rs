use axum::{extract::Multipart, response::Html, routing::get, Extension, Router};
use sqlx::Row;
use std::net::SocketAddr;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv::dotenv()?;
    let db_url = std::env::var("DATABASE_URL")?;
    let pool = sqlx::SqlitePool::connect(&db_url).await?;

    // Run migrations
    sqlx::migrate!("./migrations").run(&pool).await?;

    let app = Router::new()
        .route("/test", get(test))
        .route("/", get(index_page))
        .route("/upload", post(uploader))
        .layer(Extension(pool));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();

    Ok(())
}

async fn test(Extension(pool): Extension<sqlx::SqlitePool>) -> String {
    let result = sqlx::query("SELECT COUNT(id) FROM images")
        .fetch_one(&pool)
        .await
        .unwrap();

    let count = result.get::<i64, _>(0);
    format!("Number of images: {}", count)
}

async fn index_page() -> Html<String> {
    let path = std::path::Path::new("src.index.html");
    let content = tokio::fs::read_to_string(path).await.unwrap();
    Html(content)
}

async fn uploader(mut multipart: Multipart) -> String {
    while let Some(field) = multipart.next_field().await.unwrap() {
        let name = field.name().unwrap().to_string();
        let value = field.text().await.unwrap();
        println!("Name: {}, Value: {}", name, value);
    }

    "Ok".to_string()
}