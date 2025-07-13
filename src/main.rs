use axum::{Router, response::Json, routing::get};
use axum::{extract::State, http::StatusCode}; // Added for state management and HTTP responses
use mongodb::{Client, Collection, Database}; //DB
use serde::{Deserialize, Serialize}; // Added for JSON serialization/deserialization
use serde_json::{Value, json};

use std::net::SocketAddr;
use tokio::net::TcpListener;

// Purpose: Holds shared application state (MongoDB database connection)
// Why Clone: Axum requires state to be cloneable to share across request handlers
// Benefit: Avoids creating new DB connections for each request
#[derive(Clone)]
struct AppState {
    db: Database,
}
// Serialize/Deserialize: Converts between Rust structs and JSON
// _id field: MongoDB's default primary key field
// Optional ID: New users don't have IDs until inserted
#[derive(Serialize, Deserialize)]
struct User {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    id: Option<mongodb::bson::oid::ObjectId>,
    name: String,
    email: String,
    password: String, // Now included
}
//db handler , sets connection , return arr of jsons or http err
async fn get_users(State(state): State<AppState>) -> Result<Json<Vec<User>>, StatusCode> {
    let collection: Collection<User> = state.db.collection("users");
    match collection.find(None, None).await {
        Ok(mut cursor) => {
            let mut users = Vec::new();
            while cursor.advance().await.unwrap_or(false) {
                if let Ok(user) = cursor.deserialize_current() {
                    users.push(user);
                }
            }
            Ok(Json(users))
        }
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

async fn create_user(
    State(state): State<AppState>,
    Json(user): Json<User>,
) -> Result<Json<Value>, StatusCode> {
    let collection: Collection<User> = state.db.collection("users");

    match collection.insert_one(user, None).await {
        Ok(result) => Ok(Json(json!({
            "success": true,
            "inserted_id": result.inserted_id
        }))),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

async fn health_check() -> Json<Value> {
    Json(json!({
     "status": "Server is up and running",
     "timestamp": chrono::Utc::now().to_rfc3339()
    }))
}

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok(); // Loads environment variables from .env file
    let mongodb_uri = std::env::var("mongodb_uri").expect("MONGODB_URI must be set in .env file");
    // db only connectes if ip is authorized
    let client = Client::with_uri_str(mongodb_uri)
        .await
        .expect("Failed to connect to MongoDB");

    let db = client.database("sample_mflix");

    let state = AppState { db };

    let app = Router::new()
        .route("/health", get(health_check))
        .route("/api/users", get(get_users).post(create_user))
        .with_state(state);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("{}", addr);

    let listener = TcpListener::bind(addr).await.unwrap();

    println!("Server running on http://{}", addr);

    axum::serve(listener, app).await.unwrap();
}
