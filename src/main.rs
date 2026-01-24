mod handlers;
mod models;
mod state;

use axum::{Router, routing::get};
use mongodb::Client;
use std::net::SocketAddr;
use tokio::net::TcpListener;

use crate::handlers::{download_file, get_users, health_check, post_users, metrics_handler};
use crate::state::AppState;

use prometheus_client::registry::Registry;
use prometheus_client::metrics::family::Family;
use prometheus_client::metrics::counter::Counter;
use std::sync::Arc;
use crate::state::MetricLabels;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok(); // Loads environment variables from .env file
    let mongodb_uri = std::env::var("mongodb_uri").expect("MONGODB_URI must be set in .env file");
    // db only connectes if ip is authorized ... and that can be changed in atlast cloud
    let client = Client::with_uri_str(mongodb_uri)
        .await
        .expect("Failed to connect to MongoDB");

    let db = client.database("sample_mflix");

    // registry (for storing) 
    let mut registry = Registry::default();

    // Metric Family
    let request_counter = Family::<MetricLabels, Counter>::default();

    registry.register(
        "http_requests",
        "Number of HTTP requests",
        request_counter.clone(),
    );
    
    let state = AppState { db , registry: Arc::new(registry) , request_counter };

    let app = Router::new()
        .route("/", get(health_check))
        .route("/api/users", get(get_users).post(post_users))
        .route("/download", get(download_file))
        .route("/metrics", get(metrics_handler))
        .with_state(state);

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    println!("{}", addr);

    let listener = TcpListener::bind(addr).await.unwrap();

    println!("Server running on http://{}", addr);

    axum::serve(listener, app).await.unwrap();
}
