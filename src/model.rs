use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Serialize, Deserialize, FromRow)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub occupation: String,
    pub email: String,
    pub phone: String,
}

#[derive(Serialize, Deserialize)]
pub struct UserInfo {
    pub name: String,
    pub occupation: String,
    pub email: String,
    pub phone: String,
}
