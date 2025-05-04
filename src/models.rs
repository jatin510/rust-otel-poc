use serde::{Deserialize, Serialize};
use std::fs;
use std::io::Error;

// User data structure
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct User {
    pub id: String,
    pub name: String,
    pub email: String,
}

// Save users to JSON file
pub fn save_users_to_file(users: &[User]) -> Result<(), Error> {
    let json = serde_json::to_string_pretty(users).unwrap();
    fs::write("users.json", json)?;
    Ok(())
}

// Load users from JSON file
pub fn load_users_from_file() -> Vec<User> {
    match fs::read_to_string("users.json") {
        Ok(contents) => serde_json::from_str(&contents).unwrap_or_else(|_| Vec::new()),
        Err(_) => Vec::new(),
    }
} 