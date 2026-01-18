use crate::{models::User, state::AppState};
use axum::{
    extract::State,
    http::{StatusCode, header},
    response::{IntoResponse, Json, Response},
};
use mongodb::Collection;
use serde_json::{Value, json};
use tokio::fs;
use tokio::io::AsyncReadExt;

use crate::state::MetricLabels;
use prometheus_client::encoding::text::encode;

pub async fn metrics_handler(State(state): State<AppState>) -> impl IntoResponse {
    let mut buffer = String::new();

    // Encode the registry contents into the buffer
    if encode(&mut buffer, &state.registry).is_err() {
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Failed to encode metrics",
        )
            .into_response();
    }

    // Return the buffer with the correct content type
    let response = Response::builder()
        .header(
            header::CONTENT_TYPE,
            "text/plain; version=0.0.4; charset=utf-8",
        )
        .body(buffer)
        .unwrap();

    response.into_response()
}
//db handler , sets connection , return arr of jsons or http err
pub async fn get_users(State(state): State<AppState>) -> Result<Json<Vec<User>>, StatusCode> {
    
    state
        .request_counter
        .get_or_create(&MetricLabels {
            method: "GET".to_string(),
            path: "/api/users".to_string(),
        })
        .inc();

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
pub async fn download_file() -> Response {
    let file_path = r"C:\Users\rpaul\OneDrive\Desktop\Requirements\PROFILE2.png";

    let mut file = match fs::File::open(file_path).await {
        Ok(f) => f,
        Err(e) => {
            return (StatusCode::NOT_FOUND, format!("file not found {}", e)).into_response();
        }
    };
    let mut buffer = Vec::new();

    if let Err(_) = file.read_to_end(&mut buffer).await {
        return (StatusCode::INTERNAL_SERVER_ERROR, "read error").into_response();
    }

    let mut response = Response::new(buffer.into());

    let headers = response.headers_mut();
    headers.insert(header::CONTENT_TYPE, "image/png".parse().unwrap());
    headers.insert(
        header::CONTENT_DISPOSITION,
        "attachment; filename=\"PROFILE2.png\"".parse().unwrap(),
    );

    response
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

pub async fn health_check(State(state): State<AppState>) -> Json<Value> {
    state
        .request_counter
        .get_or_create(&MetricLabels {
            method: "GET".to_string(),
            path: "/".to_string(),
        })
        .inc();

    Json(json!({
     "status": "Server is up and running",
     "timestamp": chrono::Utc::now().to_rfc3339()
    }))
}
