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
// Code for inserting a single user
async fn post_users(
    State(state): State<AppState>, // Access application state for database client
    Json(new_user): Json<User>,    // The user data to be inserted, from the request body
) -> Result<Json<User>, StatusCode> { // Return type: Success (JSON User) or Error (HTTP Status Code)

    // 1. Access the 'users' collection
    // Explanation: This line retrieves a handle to the "users" collection from your database state.
    // The `Collection<User>` type indicates that this collection is expected to store documents
    // that map to your Rust `User` struct. The `Collection` struct is the client-side abstraction
    // for a MongoDB Collection and is used to perform "collection-level operations such as CRUD operations" [1].
    let collection: Collection<User> = state.db.collection("users");

    // 2. Perform the insert operation
    // Explanation: The `insert_one` method on the `Collection` struct is used to add a single
    // document to the collection. This is a core "CRUD operation" supported by the `Collection` [1].
    //
    // - `new_user`: This is the `User` struct instance received from the HTTP request body.
    //   The MongoDB Rust driver uses the `bson` crate for BSON support [2], which handles the
    //   serialization of your `User` struct into the BSON format required by MongoDB.
    // - `None`: This represents `InsertOneOptions`, and passing `None` means no specific options
    //   (like write concern or bypass document validation) are being provided for this operation.
    // - `.await`: The MongoDB Rust driver provides a "fully async API that requires tokio" [2].
    //   Therefore, `insert_one` is an asynchronous operation, and `.await` is used to pause execution
    //   until the database operation completes.
    match collection.insert_one(&new_user, None).await {
        // 3. Handle the result of the insertion
        Ok(_insert_result) => {
            // Explanation: If the `insert_one` operation is successful, it returns an `Ok` variant
            // containing an `InsertOneResult`. This result typically includes the `_id` of the newly
            // inserted document.
            //
            // For a successful POST request, returning `201 Created` is a common HTTP standard.
            // In this example, we're returning `200 OK` (the default for `Ok(Json(...))`)
            // and the `new_user` object itself. If you needed the generated `_id`, you would
            // extract it from `insert_result.inserted_id` and potentially update your `User` struct
            // before returning it.
            Ok(Json(new_user))
        }
        Err(_) => {
            // Explanation: If an error occurs during the insertion process (e.g., network issues,
            // database connection problems, or a violation of a unique constraint), the `Err`
            // variant is returned.
            //
            // `StatusCode::INTERNAL_SERVER_ERROR` is returned to indicate that something went wrong
            // on the server side while trying to process the request. More sophisticated error
            // handling could differentiate between specific database errors and return more precise
            // HTTP status codes.
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

async fn _create_user(
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
