use serde::{Deserialize, Serialize};

// Define a struct to represent a learning course
#[derive(Debug, Serialize, Deserialize)]
pub struct Course {
    pub id: i32,
    pub name: String,
    pub description: String,
}
