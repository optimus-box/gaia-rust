use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, FromRow)]
pub struct User {
    pub id: i64,
    pub name: String,
    pub phone: Option<String>,
    pub role: Option<String>,
    pub email: String,
    pub username: String,
    #[serde(skip)]
    pub password: Vec<u8>,
    pub created_at: i64,
    pub updated_at: i64,
    pub deleted_at: Option<i64>,
}

#[derive(Debug, Deserialize)]
pub struct UserCreate {
    pub name: String,
    pub phone: Option<String>,
    pub role: Option<String>,
    pub email: String,
    pub username: String,
    pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct UserUpdate {
    pub name: Option<String>,
    pub phone: Option<String>,
    pub role: Option<String>,
}
