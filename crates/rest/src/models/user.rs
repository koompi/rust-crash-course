use mongodb::bson::DateTime;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub name: String,
    pub email: String,
    pub password: String,
    pub created_at: DateTime,
}

impl Default for User {
    fn default() -> Self {
        User {
            name: "".to_string(),
            email: "".to_string(),
            password: "".to_string(),
            created_at: DateTime::now(),
        }
    }
}

impl User {
    pub fn new() -> Self {
        User::default()
    }
}
