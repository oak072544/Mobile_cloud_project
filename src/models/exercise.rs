use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Exercise {
    pub username: String,
    pub score: String,
}
