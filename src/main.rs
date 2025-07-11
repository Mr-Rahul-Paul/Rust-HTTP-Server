use axum::{Router, response::Json, routing::get};
use serde_json::{Value, json};
use std::net::SocketAddr;
use tokio::net::TcpListener;

mod handlers;
mod models;
use handlers::{create_user, get_users};

async fn hello_world() -> &'static str {
    "Hello, World!"
}

async fn health_check() -> Json<Value> {
    Json(json!({
     "status": "Server is up and running",
     "timestamp": chrono::Utc::now().to_rfc3339()
    }))
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(hello_world))
        .route("/health", get(health_check))
        .route("/api/users", get(get_users).post(create_user));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("{}", addr);

    let listener = TcpListener::bind(addr).await.unwrap();

    println!("Server running on http://{}", addr);
    axum::serve(listener, app).await.unwrap();
}
