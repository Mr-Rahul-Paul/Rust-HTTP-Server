use mongodb::Database;

// Purpose: Holds shared application state (MongoDB database connection)
// Why Clone: Axum requires state to be cloneable to share across request handlers
// Benefit: Avoids creating new DB connections for each request
#[derive(Clone)]
pub struct AppState {
    pub db: Database,
}
