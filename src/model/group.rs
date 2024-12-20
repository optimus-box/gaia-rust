use serde::{Deserialize, Serialize};
use sqlx::{types::Json, FromRow};

#[derive(Debug, FromRow, Serialize)]
pub struct Group {
    pub id: i64,
    pub name: String,
    pub description: Option<String>,
    pub roles: Json<Vec<String>>,
    pub created_at: i64,
    pub updated_at: i64,
    pub deleted_at: Option<i64>,
}

#[derive(Debug, Deserialize)]
pub struct GroupCreate {
    pub name: String,
    pub description: Option<String>,
    pub roles: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct GroupUpdate {
    pub name: String,
    pub description: Option<String>,
    pub roles: Vec<String>,
}
