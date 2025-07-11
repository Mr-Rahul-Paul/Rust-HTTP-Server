///import the user model
use crate::models::users::{CreateUser, User};
use std::collections::HashMap;
use std::sync::LazyLock;
use std::sync::Mutex;

pub struct DB {
    users: Mutex<HashMap<u64, User>>,
    next_id: Mutex<u64>,
}
impl DB {
    pub fn new() -> Self {
        DB {
            users: Mutex::new(HashMap::new()),
            next_id: Mutex::new(1), // id sstart from 1
        }
    }

    pub fn get_all_users(&self) -> Vec<User> {
        // Lock the HashMap to read from it
        let users = self.users.lock().unwrap();
        // Convert HashMap values to a Vec and clone them
        users.values().cloned().collect() //returned value
    }

    pub fn create_user(&self, user_data: CreateUser) -> User {
        let mut next_id = self.next_id.lock().unwrap();
        let id = *next_id;
        *next_id += 1;

        // Create the new user
        let user = User {
            id,
            name: user_data.name,
            email: user_data.email,
        };

        // Add user to the HashMap
        let mut users = self.users.lock().unwrap();
        users.insert(id, user.clone());

        user
    }
}

// Create a global instance of our database
// This will be shared across all HTTP requests
pub static DB: LazyLock<DB> = LazyLock::new(|| DB::new());
