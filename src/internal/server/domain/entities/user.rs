use serde_derive::{Deserialize, Serialize};

pub struct Login {
    pub username: String,
    pub password: String,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct LoginToken {
    pub token: String,
}

pub struct User {
    pub id: i64,
    pub username: String,
    pub password: String,
}