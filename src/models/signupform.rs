use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct SignupForm {
    pub email: String,
    pub password: String,
}
