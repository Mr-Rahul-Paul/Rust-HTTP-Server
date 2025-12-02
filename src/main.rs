mod handlers;
mod models;
mod state;

use axum::{Router, routing::get};
use mongodb::Client;
use std::net::SocketAddr;
use tokio::net::TcpListener;

use crate::handlers::{get_users, health_check, post_users};
use crate::state::AppState;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok(); // Loads environment variables from .env file
    let mongodb_uri = std::env::var("mongodb_uri").expect("MONGODB_URI must be set in .env file");
    // db only connectes if ip is authorized ... and that can be changed in atlast cloud
    let client = Client::with_uri_str(mongodb_uri)
        .await
        .expect("Failed to connect to MongoDB");

    let db = client.database("sample_mflix");

    let state = AppState { db };

    let app = Router::new()
        .route("/health", get(health_check))
        .route("/api/users", get(get_users).post(post_users))
        .with_state(state);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("{}", addr);

    let listener = TcpListener::bind(addr).await.unwrap();

    println!("Server running on http://{}", addr);

    axum::serve(listener, app).await.unwrap();
}
