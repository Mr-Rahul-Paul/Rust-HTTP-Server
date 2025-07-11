// These are our HTTP handler functions
// They receive HTTP requests and return HTTP responses

use axum::{Json, http::StatusCode};
use serde_json::json;
// Import our database and user models
use crate::models::{
    db::DB,
    users::{CreateUser, User},
};

// This function handles GET requests to /api/users
// It returns all users as JSON
#[axum::debug_handler]
pub async fn get_users() -> Json<serde_json::Value> {
    // Get all users from our database
    let users = DB.get_all_users();

    // Return a JSON response with success status and data
    Json(json!({
        "success": true,
        "data": users,
        "message": format!("Found {} users", users.len())
    }))
}

// This function handles POST requests to /api/users
// It creates a new user from the request body
#[axum::debug_handler]
pub async fn create_user(
    Json(user_data): Json<CreateUser>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    // Basic validation - check if name is not empty
    if user_data.name.trim().is_empty() {
        // Return 400 Bad Request if validation fails
        return Err(StatusCode::BAD_REQUEST);
    }

    // Basic validation - check if email contains @
    if !user_data.email.contains('@') {
        // Return 400 Bad Request if validation fails
        return Err(StatusCode::BAD_REQUEST);
    }

    // Create the user in our database
    let user = DB.create_user(user_data);

    // Return success response with the created user
    Ok(Json(json!({
        "success": true,
        "data": user,
        "message": "User created successfully"
    })))
}
