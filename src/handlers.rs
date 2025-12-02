use crate::{models::User, state::AppState};
use axum::{extract::State, http::StatusCode, response::Json};
use mongodb::Collection;
use serde_json::{Value, json};

//db handler , sets connection , return arr of jsons or http err
pub async fn get_users(State(state): State<AppState>) -> Result<Json<Vec<User>>, StatusCode> {
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

// inserting a single user
pub async fn post_users(
    State(state): State<AppState>, // access application state for db
    Json(new_user): Json<User>,    // user data to be inserted, from the req body
) -> Result<Json<User>, StatusCode> {
    let collection: Collection<User> = state.db.collection("users");

    match collection.insert_one(&new_user, None).await {
        Ok(_insert_result) => Ok(Json(new_user)),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub async fn _create_user(
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

pub async fn health_check() -> Json<Value> {
    Json(json!({
     "status": "Server is up and running",
     "timestamp": chrono::Utc::now().to_rfc3339()
    }))
}
