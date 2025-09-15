use axum::{Router, routing::get};
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(hello_world))
        .route("/json", get(hello_json));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("listening on {}", addr);
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn hello_world() -> &'static str {
    "Hello, World!"
}

use serde::Serialize;

#[derive(serde::Serialize)]
struct HelloJson {
    name: String,
}

async fn hello_json() -> axum::Json<HelloJson> {
    let msg = HelloJson {
        name: "World".to_string(),
    };

    axum::Json(msg)
}
